use crate::{Span, ToSpan};
use allocator_api2::alloc::Allocator;
use csskit_arena::{Box, Vec};

impl<'a, T: ToSpan, A: Allocator> ToSpan for Box<'a, T, A> {
	fn to_span(&self) -> Span {
		(**self).to_span()
	}
}

impl<'a, T: ToSpan, A: Allocator> ToSpan for Vec<'a, T, A> {
	fn to_span(&self) -> Span {
		let mut span = Span::ZERO;
		for item in self.iter() {
			if span == Span::ZERO {
				span = item.to_span();
			} else {
				span = span + item.to_span();
			}
		}
		span
	}
}
