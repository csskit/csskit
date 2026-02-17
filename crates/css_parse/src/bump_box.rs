use crate::{Cursor, CursorSink, Parse, Parser, Peek, SemanticEq, ToCursors};
use bumpalo::Bump;
use css_lexer::{KindSet, Span, ToSpan};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Deref, DerefMut};

/// An arena-allocated box that retains a reference to its [`Bump`] allocator, enabling [`Clone`] support.
///
/// Unlike [`bumpalo::boxed::Box`], which only stores a mutable reference to the allocated value (and thus cannot
/// implement [`Clone`] without the allocator), `BumpBox` stores both the allocator reference and the value pointer.
/// This allows it to allocate a new copy during [`Clone::clone`].
///
/// This type is intended for recursive AST nodes (e.g. `color-mix()` containing nested `<color>` values) where
/// indirection is required to break the cycle, but the allocation should still live in the parsing arena.
pub struct BumpBox<'a, T> {
	bump: &'a Bump,
	ptr: &'a mut T,
}

impl<'a, T> BumpBox<'a, T> {
	/// Allocates a new value in the given bump allocator.
	#[inline]
	pub fn new_in(bump: &'a Bump, value: T) -> Self {
		Self { bump, ptr: bump.alloc(value) }
	}
}

impl<T> Deref for BumpBox<'_, T> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &T {
		self.ptr
	}
}

impl<T> DerefMut for BumpBox<'_, T> {
	#[inline]
	fn deref_mut(&mut self) -> &mut T {
		self.ptr
	}
}

impl<T: Clone> Clone for BumpBox<'_, T> {
	fn clone(&self) -> Self {
		let cloned = self.ptr.clone();
		Self { bump: self.bump, ptr: self.bump.alloc(cloned) }
	}
}

impl<T: fmt::Debug> fmt::Debug for BumpBox<'_, T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(&**self, f)
	}
}

impl<T: PartialEq> PartialEq for BumpBox<'_, T> {
	fn eq(&self, other: &Self) -> bool {
		(**self).eq(&**other)
	}
}

impl<T: Eq> Eq for BumpBox<'_, T> {}

impl<T: PartialOrd> PartialOrd for BumpBox<'_, T> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		(**self).partial_cmp(&**other)
	}
}

impl<T: Ord> Ord for BumpBox<'_, T> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		(**self).cmp(&**other)
	}
}

impl<T: Hash> Hash for BumpBox<'_, T> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(**self).hash(state)
	}
}

impl<T: ToCursors> ToCursors for BumpBox<'_, T> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		(**self).to_cursors(s);
	}
}

impl<T: SemanticEq> SemanticEq for BumpBox<'_, T> {
	fn semantic_eq(&self, other: &Self) -> bool {
		(**self).semantic_eq(other)
	}
}

impl<T: ToSpan> ToSpan for BumpBox<'_, T> {
	fn to_span(&self) -> Span {
		(**self).to_span()
	}
}

impl<M: crate::NodeMetadata, T: crate::NodeWithMetadata<M>> crate::NodeWithMetadata<M> for BumpBox<'_, T> {
	fn self_metadata(&self) -> M {
		(**self).self_metadata()
	}

	fn metadata(&self) -> M {
		(**self).metadata()
	}
}

impl<'a, T: Peek<'a>> Peek<'a> for BumpBox<'a, T> {
	const PEEK_KINDSET: KindSet = T::PEEK_KINDSET;

	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		T::peek(p, c)
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for BumpBox<'a, T> {
	fn parse<I>(p: &mut Parser<'a, I>) -> crate::Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let value = T::parse(p)?;
		Ok(BumpBox::new_in(p.bump(), value))
	}
}

#[cfg(feature = "serde")]
impl<T: serde::Serialize> serde::Serialize for BumpBox<'_, T> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
		(**self).serialize(serializer)
	}
}
