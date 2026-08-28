use crate::Arena;
use allocator_api2::alloc::{Allocator, Layout};
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

impl<'a, T> Box<'a, T> {
	/// Gives up ownership of the value. The value stays in the arena, thus its `Drop` does not run.
	#[inline]
	pub fn leak(self) -> &'a mut T {
		let ptr = self.ptr;
		std::mem::forget(self);
		// SAFETY: `ptr` addresses a live value in an arena that outlives `'a`, and this `Box` was the
		// only owner of it. The arena frees the whole region at once.
		unsafe { &mut *ptr.as_ptr() }
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

#[cfg(feature = "serde")]
impl<'a, T: serde::Serialize, A: Allocator> serde::Serialize for Box<'a, T, A> {
	fn serialize<S: serde::Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
		(**self).serialize(serializer)
	}
}
