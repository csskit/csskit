use super::Vec;
use std::alloc::Allocator;
use std::borrow::Cow;
use std::collections::VecDeque;

macro_rules! impl_slice_eq {
	([$($vars:tt)*] $lhs:ty, $rhs:ty) => {
		impl<$($vars)*> PartialEq<$rhs> for $lhs
		where
			T: PartialEq<U>,
		{
			#[inline]
			fn eq(&self, other: &$rhs) -> bool {
				self[..] == other[..]
			}
		}
	};
}

impl_slice_eq! { ['a, 'b, T, U, A1: Allocator, A2: Allocator] Vec<'a, T, A1>, Vec<'b, U, A2> }
impl_slice_eq! { ['a, T, U, A: Allocator] Vec<'a, T, A>, [U] }
impl_slice_eq! { ['a, 'b, T, U, A: Allocator] Vec<'a, T, A>, &'b [U] }
impl_slice_eq! { ['a, 'b, T, U, A: Allocator] Vec<'a, T, A>, &'b mut [U] }
impl_slice_eq! { ['a, T, U, A: Allocator, const N: usize] Vec<'a, T, A>, [U; N] }
impl_slice_eq! { ['a, 'b, T, U, A: Allocator, const N: usize] Vec<'a, T, A>, &'b [U; N] }
impl_slice_eq! { ['a, T, U, A1: Allocator, A2: Allocator] Vec<'a, T, A1>, std::vec::Vec<U, A2> }
impl_slice_eq! { ['a, T, U, A: Allocator] [T], Vec<'a, U, A> }
impl_slice_eq! { ['a, T, U, A: Allocator] &[T], Vec<'a, U, A> }
impl_slice_eq! { ['a, T, U, A: Allocator] &mut [T], Vec<'a, U, A> }
impl_slice_eq! { ['a, T, U, A1: Allocator, A2: Allocator] std::vec::Vec<T, A1>, Vec<'a, U, A2> }
impl_slice_eq! { ['a, T: Clone, U, A: Allocator] Cow<'_, [T]>, Vec<'a, U, A> }

impl<'a, T: PartialEq<U>, U, A: Allocator> PartialEq<Vec<'a, U, A>> for VecDeque<T> {
	fn eq(&self, other: &Vec<'a, U, A>) -> bool {
		*self == &other[..]
	}
}
