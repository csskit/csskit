use atom_set::{AtomSet, DynAtomSet};
use css_lexer::{Cursor, KindSet, Lexer};
use css_parse::{
	Arena, CursorCompactWriteSink, NodeMetadata, NodeWithMetadata, Parse, Parser, SourceCursor, SourceCursorSink,
	SourceOffset, ToCursors,
};
use std::{
	cell::{Cell, Ref, RefCell},
	fmt::Write,
	marker::PhantomData,
	ops::Deref,
};

struct LenCounter(usize);

impl Write for LenCounter {
	fn write_str(&mut self, str: &str) -> std::fmt::Result {
		self.0 += str.len();
		Ok(())
	}
}

impl<'t> SourceCursorSink<'t> for LenCounter {
	fn append(&mut self, cursor: SourceCursor<'t>) {
		cursor.write_to(self).expect("LenCounter never errors");
	}
}

pub trait TransformerFeatures<M, N>: Sized + Default + Copy {
	fn transforms<'a, 'ctx>(self, transformer: &'ctx Transformer<'a, M, N, Self>, node: &mut N)
	where
		M: NodeMetadata,
		N: NodeWithMetadata<M> + css_ast::VisitableMut<'a>;
}

/// The text of a transformed tree, from [Transformer::text].
pub enum TransformedText<'t, 'a> {
	/// No node has been parsed into the tree, so it reads only from the source.
	Source(&'a str),
	/// The source followed by the text of every parsed node.
	Buffer(Ref<'t, std::string::String>),
}

impl Deref for TransformedText<'_, '_> {
	type Target = str;

	fn deref(&self) -> &str {
		match self {
			Self::Source(text) => text,
			Self::Buffer(buffer) => buffer,
		}
	}
}

pub struct Transformer<'a, M: NodeMetadata, N: NodeWithMetadata<M>, F: TransformerFeatures<M, N>> {
	alloc: &'a Arena,
	atoms: &'static dyn DynAtomSet,
	pub(crate) features: F,
	changed: Cell<bool>,
	/// The source text followed by the text of every node parsed by [Transformer::parse]; empty until the first parse.
	text: RefCell<std::string::String>,
	source_text: &'a str,
	_phantom: PhantomData<(M, N)>,
}

impl<'a, M: NodeMetadata, N: NodeWithMetadata<M>, F: TransformerFeatures<M, N>> Transformer<'a, M, N, F> {
	pub fn new_in(alloc: &'a Arena, features: F, atoms: &'static dyn DynAtomSet, source_text: &'a str) -> Self {
		Self {
			alloc,
			features,
			atoms,
			changed: Cell::new(false),
			text: RefCell::default(),
			source_text,
			_phantom: PhantomData,
		}
	}

	/// Rewrites `node` in place with every enabled transform. Nodes parsed into the tree take offsets past the end of
	/// the source, so write `node` against [Transformer::text], and use a new transformer for each parsed tree.
	pub fn transform(&mut self, node: &mut N)
	where
		N: css_ast::VisitableMut<'a>,
	{
		self.changed.set(false);
		self.features.transforms(self, node);
	}

	pub fn has_changed(&self) -> bool {
		self.changed.get()
	}

	/// Records that a transform changed the tree without minting tokens, for example by removing a node.
	pub fn mark_changed(&self) {
		self.changed.set(true);
	}

	/// Puts `new` in place of `node`, recording that the tree changed.
	pub fn replace<T>(&self, node: &mut T, new: T) {
		*node = new;
		self.changed.set(true);
	}

	pub fn alloc(&self) -> &'a Arena {
		self.alloc
	}

	/// Returns the text every cursor in the transformed tree reads from: the source, followed by the text of every
	/// parsed node. Drop it before calling [Transformer::parse].
	pub fn text(&self) -> TransformedText<'_, 'a> {
		let buffer = self.text.borrow();
		if buffer.is_empty() { TransformedText::Source(self.source_text) } else { TransformedText::Buffer(buffer) }
	}

	/// Pairs `cursor` with its text. Text of a parsed node's token is copied into the arena, as
	/// [Transformer::text] may grow.
	pub fn to_source_cursor(&self, cursor: Cursor) -> SourceCursor<'a> {
		if (cursor.offset().0 as usize) < self.source_text.len() {
			return SourceCursor::from(cursor, cursor.str_slice(self.source_text));
		}
		let text = css_parse::String::from_str_in(cursor.str_slice(&self.text()), self.alloc).into_str();
		SourceCursor::from(cursor, text)
	}

	/// Returns the length of `node` once written by [CursorCompactWriteSink].
	pub fn compacted_len(&self, node: &impl ToCursors) -> usize {
		let text = self.text();
		let mut counter = LenCounter(0);
		node.to_cursors(&mut CursorCompactWriteSink::new(&text, &mut counter));
		counter.0
	}

	/// Returns `text`, which must be a single token, in its compacted form, or `None` if it is not exactly one token.
	pub fn compact(&self, text: &'a str) -> Option<&'a str> {
		let token = Lexer::new(self.atoms, text).advance();
		if token.len() as usize != text.len() {
			return None;
		}
		let cursor = SourceCursor::from(Cursor::new(SourceOffset(0), token), text);
		if !cursor.may_compact() {
			return Some(text);
		}
		let mut compacted = css_parse::String::new_in(self.alloc);
		cursor.compact().write_to(&mut compacted).ok()?;
		Some(compacted.into_str())
	}

	/// Parses `text` as a `T` to put into the tree, returning `None` if `text` does not parse entirely.
	///
	/// The new node's tokens take offsets past the end of everything parsed so far, and `text` is appended to
	/// [Transformer::text] so those offsets resolve to it.
	pub fn parse<T: Parse<'a> + ToCursors>(&self, text: &'a str) -> Option<T> {
		let base = SourceOffset(self.text().len() as u32);
		let cursors = Lexer::new(self.atoms, text)
			.map(|cursor| Cursor::new(SourceOffset(cursor.offset().0 + base.0), cursor.token()));
		let parsed = Parser::new_at(self.alloc, text, base, cursors).parse_entirely::<T>();
		let output = parsed.output.filter(|_| parsed.errors.is_empty())?;
		let mut buffer = self.text.borrow_mut();
		if buffer.is_empty() {
			buffer.push_str(self.source_text);
		}
		buffer.push_str(text);
		Some(output)
	}

	pub fn to_atom<A: AtomSet + PartialEq>(&self, c: Cursor) -> A {
		let bits = c.token().atom_bits();
		if bits == 0 {
			if c != KindSet::ATOM_LIKE {
				return A::from_bits(0);
			}
			return A::from_str(&self.to_source_cursor(c).parse(self.alloc));
		}
		A::from_bits(bits)
	}
}

#[macro_export]
macro_rules! transformer {
	($(#[$meta:meta])* $vis:vis enum $feature: ident [ $metadata: ident ] { $( $(#[$varmeta:meta])* $variant: ident$(,)?)+ } ) => {
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
				N: ::css_parse::NodeWithMetadata<$metadata>
			{
				fn transforms<'a, 'ctx>(self, transformer: &'ctx $crate::Transformer<'a, $metadata, N, Self>, node: &mut N)
				where
					N: ::css_ast::VisitableMut<'a>,
				{
					$(
						if self.contains(Self::$variant)
							&& !<$variant<'a, 'ctx, N> as $crate::Transform<'a, 'ctx, $metadata, N, Self>>::skips_subtree(
								&::css_parse::NodeWithMetadata::metadata(node),
							) {
							let _ = ::css_ast::VisitableMut::accept_mut(node, &mut $variant::new(transformer));
						}
					)+
				}
			}
    };
	}
