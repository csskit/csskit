use crate::{Arena, Cursor, CursorSink, Parse, Parser, Peek, SemanticEq, ToCursors};
use allocator_api2::alloc::{Allocator, Layout};
use css_lexer::{KindSet, Span, ToSpan};
use std::{
	fmt,
	hash::{Hash, Hasher},
	marker::PhantomData,
	ops::{Deref, DerefMut},
	ptr::NonNull,
};

/// An arena-allocated box that retains a reference to its allocator, enabling [`Clone`] support.
///
/// This type is intended for recursive AST nodes (e.g. `color-mix()` containing nested `<color>` values) where
/// indirection is required to break the cycle, but the allocation should still live in the parsing arena.
#[repr(C)]
pub struct Box<'a, T, A: Allocator = &'a Arena> {
	ptr: NonNull<T>,
	alloc: A,
	marker: PhantomData<&'a mut T>,
}

impl<'a, T, A: Allocator> Box<'a, T, A> {
	/// Allocate `value` in the given `alloc`.
	#[inline]
	pub fn new_in(alloc: A, value: T) -> Self {
		let ptr = alloc.allocate(Layout::new::<T>()).expect("arena exhausted").cast::<T>();
		unsafe { ptr.as_ptr().write(value) };
		Self { ptr, alloc, marker: PhantomData }
	}
}

impl<'a, T, A: Allocator> Deref for Box<'a, T, A> {
	type Target = T;

	#[inline]
	fn deref(&self) -> &T {
		unsafe { self.ptr.as_ref() }
	}
}

impl<'a, T, A: Allocator> DerefMut for Box<'a, T, A> {
	#[inline]
	fn deref_mut(&mut self) -> &mut T {
		unsafe { self.ptr.as_mut() }
	}
}

impl<'a, T, A: Allocator> Drop for Box<'a, T, A> {
	fn drop(&mut self) {
		unsafe { self.ptr.as_ptr().drop_in_place() };
	}
}

impl<'a, T: Clone, A: Allocator + Clone> Clone for Box<'a, T, A> {
	fn clone(&self) -> Self {
		Box::new_in(self.alloc.clone(), (**self).clone())
	}
}

impl<'a, T: fmt::Debug, A: Allocator> fmt::Debug for Box<'a, T, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Debug::fmt(&**self, f)
	}
}

impl<'a, T: fmt::Display, A: Allocator> fmt::Display for Box<'a, T, A> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(&**self, f)
	}
}

impl<'a, T: PartialEq, A: Allocator> PartialEq for Box<'a, T, A> {
	fn eq(&self, other: &Self) -> bool {
		(**self).eq(&**other)
	}
}

impl<'a, T: Eq, A: Allocator> Eq for Box<'a, T, A> {}

impl<'a, T: PartialOrd, A: Allocator> PartialOrd for Box<'a, T, A> {
	fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
		(**self).partial_cmp(&**other)
	}
}

impl<'a, T: Ord, A: Allocator> Ord for Box<'a, T, A> {
	fn cmp(&self, other: &Self) -> std::cmp::Ordering {
		(**self).cmp(&**other)
	}
}

impl<'a, T: Hash, A: Allocator> Hash for Box<'a, T, A> {
	fn hash<H: Hasher>(&self, state: &mut H) {
		(**self).hash(state);
	}
}

impl<'a, T: ToCursors, A: Allocator> ToCursors for Box<'a, T, A> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		(**self).to_cursors(s);
	}
}

impl<'a, T: SemanticEq, A: Allocator> SemanticEq for Box<'a, T, A> {
	fn semantic_eq(&self, other: &Self) -> bool {
		(**self).semantic_eq(other)
	}
}

impl<'a, T: ToSpan, A: Allocator> ToSpan for Box<'a, T, A> {
	fn to_span(&self) -> Span {
		(**self).to_span()
	}
}

impl<'a, M: crate::NodeMetadata, T: crate::NodeWithMetadata<M>, A: Allocator> crate::NodeWithMetadata<M>
	for Box<'a, T, A>
{
	fn self_metadata(&self) -> M {
		(**self).self_metadata()
	}

	fn metadata(&self) -> M {
		(**self).metadata()
	}
}

impl<'a, T: Peek<'a>, A: Allocator> Peek<'a> for Box<'a, T, A> {
	const PEEK_KINDSET: KindSet = T::PEEK_KINDSET;

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		T::peek(p, c)
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for Box<'a, T> {
	fn parse<I>(p: &mut Parser<'a, I>) -> crate::Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let value = T::parse(p)?;
		Ok(Box::new_in(p.alloc(), value))
	}
}

#[cfg(feature = "serde")]
impl<'a, T: serde::Serialize, A: Allocator> serde::Serialize for Box<'a, T, A> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
		(**self).serialize(serializer)
	}
}
