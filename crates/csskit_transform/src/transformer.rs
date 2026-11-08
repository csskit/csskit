use atom_set::{AtomSet, DynAtomSet};
use css_lexer::{Cursor, KindSet, Lexer, ToSpan};
use css_parse::Vec;
use css_parse::{
	Arena, CursorOverlaySet, CursorSink, CursorToSourceCursorSink, NodeMetadata, NodeWithMetadata, OverlayKind,
	OverlaySegment, Parse, Parser, SourceCursor, SourceOffset, Span, ToCursors,
};
use std::{
	cell::{Ref, RefCell},
	fmt::Write,
	marker::PhantomData,
};

struct LenCounter(usize);

impl Write for LenCounter {
	fn write_str(&mut self, str: &str) -> std::fmt::Result {
		self.0 += str.len();
		Ok(())
	}
}

#[derive(Debug)]
pub enum TransformEdit<'a> {
	Replace { target: Span, cursors: Vec<'a, SourceCursor<'a>> },
	InsertBefore { anchor: SourceOffset, cursors: Vec<'a, SourceCursor<'a>> },
	InsertAfter { anchor: SourceOffset, cursors: Vec<'a, SourceCursor<'a>> },
	Delete { target: Span },
}

impl TransformEdit<'_> {
	fn span(&self) -> Span {
		match self {
			Self::Replace { target, .. } | Self::Delete { target } => *target,
			Self::InsertBefore { anchor, .. } | Self::InsertAfter { anchor, .. } => Span::new(*anchor, *anchor),
		}
	}

	fn rewrites_target(&self) -> bool {
		matches!(self, Self::Replace { .. } | Self::Delete { .. })
	}
}

struct PendingEdit<'a> {
	edit: TransformEdit<'a>,
	span: Span,
	run_max_end: SourceOffset,
	removed: bool,
}

struct PendingEdits<'a> {
	edits: Vec<'a, PendingEdit<'a>>,
	run_starts: Vec<'a, usize>,
}

impl<'a> PendingEdits<'a> {
	fn new_in(alloc: &'a Arena) -> Self {
		Self { edits: Vec::new_in(alloc), run_starts: Vec::new_in(alloc) }
	}

	fn is_empty(&self) -> bool {
		self.edits.is_empty()
	}

	fn len(&self) -> usize {
		self.edits.len()
	}

	/// Drains the edits that were not removed, paired with their insertion order.
	fn drain(&mut self) -> impl Iterator<Item = (usize, TransformEdit<'a>)> + '_ {
		self.run_starts.clear();
		self.edits
			.drain(..)
			.enumerate()
			.filter_map(|(order, pending)| (!pending.removed).then_some((order, pending.edit)))
	}

	fn clear(&mut self) {
		self.edits.clear();
		self.run_starts.clear();
	}

	fn push(&mut self, edit: TransformEdit<'a>) {
		let span = edit.span();
		let run_max_end = match self.edits.last() {
			Some(last) if last.span.start() <= span.start() => last.run_max_end.max(span.end()),
			_ => {
				self.run_starts.push(self.edits.len());
				span.end()
			}
		};
		self.edits.push(PendingEdit { edit, span, run_max_end, removed: false });
	}

	fn run_end(&self, run: usize) -> usize {
		self.run_starts.get(run + 1).copied().unwrap_or(self.edits.len())
	}

	/// Removes every edit within `span`, returning true if any were removed.
	fn remove_within(&mut self, span: Span) -> bool {
		let mut removed_any = false;
		for run in 0..self.run_starts.len() {
			let run_range = self.run_starts[run]..self.run_end(run);
			let run_edits = &mut self.edits[run_range];
			let first = run_edits.partition_point(|pending| pending.span.start() < span.start());
			let last = run_edits.partition_point(|pending| pending.span.start() <= span.end());
			for pending in &mut run_edits[first..last] {
				if !pending.removed && pending.span.end() <= span.end() {
					pending.removed = true;
					removed_any = true;
				}
			}
		}
		removed_any
	}

	fn any_rewrite_covering(&self, span: Span) -> bool {
		(0..self.run_starts.len()).any(|run| {
			let run_edits = &self.edits[self.run_starts[run]..self.run_end(run)];
			let candidates = &run_edits[..run_edits.partition_point(|pending| pending.span.start() <= span.start())];
			candidates
				.iter()
				.rev()
				.take_while(|pending| pending.run_max_end >= span.end())
				.any(|pending| !pending.removed && pending.edit.rewrites_target() && pending.span.end() >= span.end())
		})
	}
}

#[derive(Debug)]
pub enum CommitError {
	OverlappingEdit { previous: Span, new: Span },
	InvalidEdit { span: Span },
}

struct PendingSegment<'a> {
	span: Span,
	intent: OverlayKind,
	order: usize,
	cursors: Vec<'a, SourceCursor<'a>>,
}

pub trait TransformerFeatures<M, N>: Sized + Default + Copy {
	fn transforms<'a, 'ctx>(self, transformer: &'ctx Transformer<'a, M, N, Self>, node: &N)
	where
		M: NodeMetadata,
		N: NodeWithMetadata<M>;
}

pub struct Transformer<'a, M: NodeMetadata, N: NodeWithMetadata<M>, F: TransformerFeatures<M, N>> {
	alloc: &'a Arena,
	atoms: &'static dyn DynAtomSet,
	pub(crate) features: F,
	changed: RefCell<bool>,
	overlays: RefCell<CursorOverlaySet<'a>>,
	edits: RefCell<PendingEdits<'a>>,
	pub(crate) source_text: &'a str,
	_phantom: PhantomData<(M, N)>,
}

impl<'a, M: NodeMetadata, N: NodeWithMetadata<M>, F: TransformerFeatures<M, N>> Transformer<'a, M, N, F> {
	pub fn new_in(alloc: &'a Arena, features: F, atoms: &'static dyn DynAtomSet, source_text: &'a str) -> Self {
		Self {
			alloc,
			features,
			atoms,
			changed: RefCell::new(false),
			overlays: RefCell::new(CursorOverlaySet::new(alloc)),
			edits: RefCell::new(PendingEdits::new_in(alloc)),
			source_text,
			_phantom: PhantomData,
		}
	}

	pub fn transform(&mut self, node: &mut N) {
		self.reset();
		self.features.transforms(self, node);
		if let Err(err) = self.commit_overlays() {
			panic!("Transform overlays commit failed: {:?}", err);
		}
	}

	pub fn has_changed(&self) -> bool {
		*self.changed.borrow()
	}

	pub fn alloc(&self) -> &'a Arena {
		self.alloc
	}

	pub fn to_source_cursor(&self, cursor: Cursor) -> SourceCursor<'a> {
		SourceCursor::from(cursor, cursor.str_slice(self.source_text))
	}

	pub fn to_source_cursors(&self, parsed: &impl ToCursors) -> Vec<'a, SourceCursor<'a>> {
		let mut cursors = Vec::new_in(self.alloc());
		let mut sink = CursorToSourceCursorSink::new(self.source_text, &mut cursors);
		parsed.to_cursors(&mut sink);
		cursors
	}

	/// Returns the length of `cursors` once written by a compacting sink.
	pub fn compacted_len(&self, cursors: &[SourceCursor<'a>]) -> usize {
		let mut counter = LenCounter(0);
		for cursor in cursors {
			write!(counter, "{}", cursor.compact()).expect("LenCounter never errors");
		}
		counter.0
	}

	pub fn to_atom<A: AtomSet + PartialEq>(&self, c: Cursor) -> A {
		let bits = c.token().atom_bits();
		if bits == 0 {
			if c != KindSet::ATOM_LIKE {
				return A::from_bits(0);
			}
			let source_cursor = self.to_source_cursor(c);
			return A::from_str(&source_cursor.parse(self.alloc));
		}
		A::from_bits(bits)
	}

	pub fn overlays(&self) -> Ref<'_, CursorOverlaySet<'a>> {
		self.overlays.borrow()
	}

	pub fn parse_value<T>(&self, source: &'a str) -> Vec<'a, SourceCursor<'a>>
	where
		T: Parse<'a> + ToCursors,
	{
		let lexer = Lexer::new(self.atoms, source);
		let mut parser = Parser::new(self.alloc, source, lexer);
		let parsed = parser.parse_entirely::<T>();
		debug_assert!(
			parsed.output.is_some(),
			"Transformer::parse_value failed to parse {:?}: {:?}",
			source,
			parsed.errors
		);
		let mut cursors = Vec::new_in(self.alloc());
		let mut sink = CursorToSourceCursorSink::new(source, &mut cursors);
		parsed.to_cursors(&mut sink as &mut dyn CursorSink);
		cursors
	}

	pub fn reset(&self) {
		*self.changed.borrow_mut() = false;
		self.overlays.borrow_mut().clear();
		self.edits.borrow_mut().clear();
	}

	pub fn has_replacement(&self, span: impl ToSpan) -> bool {
		self.overlays.borrow().has_overlay(span.to_span())
	}

	pub fn clear_pending_edits(&self, span: Span) -> bool {
		self.edits.borrow_mut().remove_within(span)
	}

	/// Returns true if a pending edit already replaces or deletes a span that covers `span`.
	pub fn has_pending_edit_covering(&self, span: Span) -> bool {
		self.edits.borrow().any_rewrite_covering(span)
	}

	pub fn replace(&self, span: impl ToSpan, cursors: Vec<'a, SourceCursor<'a>>) {
		let span = span.to_span();
		debug_assert!(span.start() <= span.end(), "Transformer::replace received invalid span: {:?}", span);
		*self.changed.borrow_mut() = true;
		self.edits.borrow_mut().push(TransformEdit::Replace { target: span, cursors });
	}

	pub fn delete(&self, span: impl ToSpan) {
		let span = span.to_span();
		debug_assert!(span.start() <= span.end(), "Transformer::delete received invalid span: {:?}", span);
		*self.changed.borrow_mut() = true;
		self.edits.borrow_mut().push(TransformEdit::Delete { target: span });
	}

	pub fn insert_before(&self, anchor: SourceOffset, cursors: Vec<'a, SourceCursor<'a>>) {
		*self.changed.borrow_mut() = true;
		self.edits.borrow_mut().push(TransformEdit::InsertBefore { anchor, cursors });
	}

	pub fn insert_after(&self, anchor: SourceOffset, cursors: Vec<'a, SourceCursor<'a>>) {
		*self.changed.borrow_mut() = true;
		self.edits.borrow_mut().push(TransformEdit::InsertAfter { anchor, cursors });
	}

	pub fn replace_parsed<T>(&self, span: impl ToSpan, css: &'a str)
	where
		T: Parse<'a> + ToCursors,
	{
		self.replace(span, self.parse_value::<T>(css));
	}

	pub fn commit_overlays(&self) -> Result<(), CommitError> {
		let mut edits = self.edits.borrow_mut();
		if edits.is_empty() {
			return Ok(());
		}

		let mut pending_segments: Vec<'a, PendingSegment<'a>> = Vec::with_capacity_in(edits.len(), self.alloc);

		for (order, edit) in edits.drain() {
			match edit {
				TransformEdit::Replace { target, cursors } => {
					if target.start() > target.end() {
						return Err(CommitError::InvalidEdit { span: target });
					}
					pending_segments.push(PendingSegment {
						span: target,
						intent: OverlayKind::Replace,
						order,
						cursors,
					});
				}
				TransformEdit::InsertBefore { anchor, cursors } => {
					let span = Span::new(anchor, anchor);
					pending_segments.push(PendingSegment { span, intent: OverlayKind::InsertBefore, order, cursors });
				}
				TransformEdit::InsertAfter { anchor, cursors } => {
					let span = Span::new(anchor, anchor);
					pending_segments.push(PendingSegment { span, intent: OverlayKind::InsertAfter, order, cursors });
				}
				TransformEdit::Delete { target } => {
					pending_segments.push(PendingSegment {
						span: target,
						intent: OverlayKind::Replace,
						order,
						cursors: Vec::with_capacity_in(0, self.alloc()),
					});
				}
			}
		}

		if pending_segments.is_empty() {
			return Ok(());
		}

		pending_segments.sort_by(|a, b| {
			a.span
				.start()
				.cmp(&b.span.start())
				.then_with(|| a.span.end().cmp(&b.span.end()))
				.then_with(|| a.intent.cmp(&b.intent))
				.then_with(|| a.order.cmp(&b.order))
		});

		let mut last_non_zero: Option<Span> = None;
		for segment in &pending_segments {
			if segment.span.start() > segment.span.end() {
				return Err(CommitError::InvalidEdit { span: segment.span });
			}
			if segment.span.start() == segment.span.end() {
				continue;
			}
			if let Some(prev) = last_non_zero
				&& segment.span.start() < prev.end()
			{
				return Err(CommitError::OverlappingEdit { previous: prev, new: segment.span });
			}
			last_non_zero = Some(segment.span);
		}

		let mut overlays = self.overlays.borrow_mut();
		overlays.clear();
		for segment in pending_segments {
			let overlay_segment = OverlaySegment::new(segment.span, segment.cursors, segment.intent);
			overlays.push_segment(overlay_segment);
		}

		Ok(())
	}
}

#[macro_export]
macro_rules! transformer {
	($(#[$meta:meta])* $vis:vis enum $feature: ident [ $metadata: ident, $($node:tt)+ ] { $( $(#[$varmeta:meta])* $variant: ident$(,)?)+ } ) => {
			use $crate::Transform;

			$(#[$meta])*
			#[bitmask(u16)]
			pub enum $feature {
				$(
					$(#[$varmeta])*
					$variant,
				)+
			}

			impl<N> $crate::TransformerFeatures<$metadata, N> for $feature
			where
				N: $($node)+ + ::css_parse::NodeWithMetadata<$metadata>
			{
				fn transforms<'a, 'ctx>(self, transformer: &'ctx $crate::Transformer<'a, $metadata, N, Self>, node: &N) {
					let metadata = ::css_parse::NodeWithMetadata::metadata(node);
					$(
						if self.contains(Self::$variant)
							&& !<$variant<'a, 'ctx, N> as $crate::Transform<'a, 'ctx, $metadata, N, Self>>::skips_subtree(
								&metadata,
							) {
							let mut transform = $variant::new(transformer);
							let _ = node.accept(&mut transform);
						}
					)+
				}
			}
    };
	}

#[cfg(test)]
mod tests {
	use crate::CssMinifierFeature;

	use super::*;
	use css_ast::{CssAtomSet, CssMetadata};
	use css_parse::{Arena, ComponentValues, SourceOffset, Span};

	#[test]
	fn commit_overlays_rejects_overlapping_edits() {
		let alloc = Arena::default();
		let context: Transformer<CssMetadata, ComponentValues, CssMinifierFeature> =
			Transformer::new_in(&alloc, CssMinifierFeature::all_bits(), &CssAtomSet::ATOMS, "");
		let first = context.parse_value::<ComponentValues>("a");
		let second = context.parse_value::<ComponentValues>("b");

		context.replace(Span::new(SourceOffset(0), SourceOffset(2)), first);
		context.replace(Span::new(SourceOffset(1), SourceOffset(3)), second);

		let err = context.commit_overlays().expect_err("expected overlapping edits to fail");
		match err {
			CommitError::OverlappingEdit { previous, new } => {
				assert_eq!(previous, Span::new(SourceOffset(0), SourceOffset(2)));
				assert_eq!(new, Span::new(SourceOffset(1), SourceOffset(3)));
			}
			other => panic!("unexpected commit error: {other:?}"),
		}
	}

	#[test]
	fn commit_overlays_preserves_insert_order() {
		let alloc = Arena::default();
		let context: Transformer<CssMetadata, ComponentValues, CssMinifierFeature> =
			Transformer::new_in(&alloc, CssMinifierFeature::all_bits(), &CssAtomSet::ATOMS, "");
		let anchor = SourceOffset(5);

		context.insert_before(anchor, context.parse_value::<ComponentValues>("A"));
		context.insert_before(anchor, context.parse_value::<ComponentValues>("B"));
		context.insert_after(anchor, context.parse_value::<ComponentValues>("C"));
		context.insert_after(anchor, context.parse_value::<ComponentValues>("D"));

		context.commit_overlays().expect("commit should succeed");
		let overlays = context.overlays();
		let segments = overlays.segments();

		assert_eq!(segments.len(), 4);
		assert_eq!(segments[0].cursors()[0].source(), "A");
		assert_eq!(segments[1].cursors()[0].source(), "B");
		assert_eq!(segments[2].cursors()[0].source(), "C");
		assert_eq!(segments[3].cursors()[0].source(), "D");
	}

	#[test]
	fn committing_cleared_edits_preserves_existing_overlays() {
		let alloc = Arena::default();
		let context: Transformer<CssMetadata, ComponentValues, CssMinifierFeature> =
			Transformer::new_in(&alloc, CssMinifierFeature::all_bits(), &CssAtomSet::ATOMS, "");
		context.delete(span(0, 2));
		context.commit_overlays().unwrap();
		context.delete(span(4, 6));
		assert!(context.clear_pending_edits(span(4, 6)));
		context.commit_overlays().unwrap();
		assert!(context.has_replacement(span(0, 2)));
		assert!(!context.has_replacement(span(4, 6)));
	}

	fn span(start: u32, end: u32) -> Span {
		Span::new(SourceOffset(start), SourceOffset(end))
	}

	#[test]
	fn clear_pending_edits_finds_edits_pushed_out_of_order() {
		let alloc = Arena::default();
		let context: Transformer<CssMetadata, ComponentValues, CssMinifierFeature> =
			Transformer::new_in(&alloc, CssMinifierFeature::all_bits(), &CssAtomSet::ATOMS, "");

		context.delete(span(20, 22));
		context.delete(span(30, 32));
		context.delete(span(10, 12));
		context.delete(span(24, 26));

		assert!(context.clear_pending_edits(span(20, 27)));
		assert!(!context.clear_pending_edits(span(20, 27)));
		assert!(!context.has_pending_edit_covering(span(20, 22)));
		assert!(!context.has_pending_edit_covering(span(24, 26)));
		assert!(context.has_pending_edit_covering(span(10, 12)));
		assert!(context.has_pending_edit_covering(span(30, 32)));
	}

	#[test]
	fn has_pending_edit_covering_sees_past_later_nested_edits() {
		let alloc = Arena::default();
		let context: Transformer<CssMetadata, ComponentValues, CssMinifierFeature> =
			Transformer::new_in(&alloc, CssMinifierFeature::all_bits(), &CssAtomSet::ATOMS, "");

		context.delete(span(0, 50));
		context.delete(span(5, 6));
		context.insert_before(SourceOffset(8), context.parse_value::<ComponentValues>("a"));

		assert!(context.has_pending_edit_covering(span(10, 20)));
		assert!(!context.has_pending_edit_covering(span(40, 60)));
	}
}
