use bumpalo::{Bump, collections::Vec};
use css_lexer::{AtomSet, Cursor, DynAtomSet, Lexer, ToSpan};
use css_parse::{
	CursorOverlaySet, CursorToSourceCursorSink, NodeMetadata, NodeWithMetadata, OverlayKind, OverlaySegment, Parse,
	Parser, SourceCursor, SourceOffset, Span, ToCursors,
};
use std::{cell::RefCell, marker::PhantomData};

#[derive(Debug)]
pub enum TransformEdit<'a> {
	Replace { target: Span, cursors: Vec<'a, SourceCursor<'a>> },
	InsertBefore { anchor: SourceOffset, cursors: Vec<'a, SourceCursor<'a>> },
	InsertAfter { anchor: SourceOffset, cursors: Vec<'a, SourceCursor<'a>> },
	Delete { target: Span },
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
	bump: &'a Bump,
	atoms: &'static dyn DynAtomSet,
	pub(crate) features: F,
	changed: RefCell<bool>,
	overlays: RefCell<CursorOverlaySet<'a>>,
	edits: RefCell<Vec<'a, TransformEdit<'a>>>,
	pub(crate) source_text: &'a str,
	_phantom: PhantomData<(M, N)>,
}

impl<'a, M: NodeMetadata, N: NodeWithMetadata<M>, F: TransformerFeatures<M, N>> Transformer<'a, M, N, F> {
	pub fn new_in(bump: &'a Bump, features: F, atoms: &'static dyn DynAtomSet, source_text: &'a str) -> Self {
		Self {
			bump,
			features,
			atoms,
			changed: RefCell::new(false),
			overlays: RefCell::new(CursorOverlaySet::new(bump)),
			edits: RefCell::new(Vec::new_in(bump)),
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

	pub fn bump(&self) -> &'a Bump {
		self.bump
	}

	pub fn to_source_cursor(&self, cursor: Cursor) -> SourceCursor<'a> {
		SourceCursor::from(cursor, cursor.str_slice(self.source_text))
	}

	pub fn to_source_cursors(&self, parsed: &impl ToCursors) -> Vec<'a, SourceCursor<'a>> {
		let mut cursors = Vec::new_in(self.bump());
		let mut sink = CursorToSourceCursorSink::new(self.source_text, &mut cursors);
		parsed.to_cursors(&mut sink);
		cursors
	}

	pub fn to_atom<A: AtomSet + PartialEq>(&self, c: Cursor) -> A {
		let bits = c.atom_bits();
		if bits == 0 {
			let source_cursor = self.to_source_cursor(c);
			return A::from_str(&source_cursor.parse(self.bump));
		}
		A::from_bits(bits)
	}

	pub fn overlays(&self) -> std::cell::Ref<'_, CursorOverlaySet<'a>> {
		self.overlays.borrow()
	}

	pub fn parse_value<T>(&self, source: &'a str) -> Vec<'a, SourceCursor<'a>>
	where
		T: Parse<'a> + ToCursors,
	{
		let lexer = Lexer::new(self.atoms, source);
		let mut parser = Parser::new(self.bump, source, lexer);
		let parsed = parser.parse_entirely::<T>();
		debug_assert!(
			parsed.output.is_some(),
			"Transformer::parse_value failed to parse {:?}: {:?}",
			source,
			parsed.errors
		);
		let mut cursors = Vec::new_in(self.bump());
		let mut sink = CursorToSourceCursorSink::new(source, &mut cursors);
		parsed.to_cursors(&mut sink);
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
		let mut edits = self.edits.borrow_mut();
		let len_before = edits.len();
		edits.retain(|edit| match edit {
			TransformEdit::Replace { target, .. } | TransformEdit::Delete { target } => target != &span,
			_ => true,
		});
		len_before != edits.len()
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

	pub fn replace_parsed<T>(&self, span: impl ToSpan, css: &str)
	where
		T: Parse<'a> + ToCursors,
	{
		let owned = self.bump.alloc_str(css);
		self.replace(span, self.parse_value::<T>(owned));
	}

	pub fn commit_overlays(&self) -> Result<(), CommitError> {
		let mut edits = self.edits.borrow_mut();
		if edits.is_empty() {
			return Ok(());
		}

		let mut pending_segments: Vec<'a, PendingSegment<'a>> = Vec::with_capacity_in(edits.len(), self.bump);

		for (order, edit) in edits.drain(..).enumerate() {
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
						cursors: Vec::with_capacity_in(0, self.bump()),
					});
				}
			}
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
					$(
						if $variant::may_change(transformer.features, node) {
							let mut transform = $variant::new(transformer);
							node.accept(&mut transform);
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
	use bumpalo::Bump;
	use css_ast::{CssAtomSet, CssMetadata};
	use css_parse::{ComponentValues, SourceOffset, Span};

	#[test]
	fn commit_overlays_rejects_overlapping_edits() {
		let bump = Bump::default();
		let context: Transformer<CssMetadata, ComponentValues, CssMinifierFeature> =
			Transformer::new_in(&bump, CssMinifierFeature::all_bits(), &CssAtomSet::ATOMS, "");
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
		let bump = Bump::default();
		let context: Transformer<CssMetadata, ComponentValues, CssMinifierFeature> =
			Transformer::new_in(&bump, CssMinifierFeature::all_bits(), &CssAtomSet::ATOMS, "");
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
}
