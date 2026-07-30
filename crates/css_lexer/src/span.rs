use crate::SourceOffset;
use allocator_api2::alloc::{Allocator, Global};
use allocator_api2::boxed::Box;
use std::{fmt::Display, hash::Hash, marker::PhantomData, ops::Add};

/// Represents a range of text within a document, as a Start and End offset.
///
/// Effectively two [SourceOffsets][SourceOffset] in one struct.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Span {
	start: SourceOffset,
	end: SourceOffset,
}

impl Span {
	/// Represents a fake Span with [SourceOffset::DUMMY] as the start/end offsets.
	pub const DUMMY: Span = Span::new(SourceOffset::DUMMY, SourceOffset::DUMMY);
	pub const ZERO: Span = Span::new(SourceOffset::ZERO, SourceOffset::ZERO);

	/// Creates a new [Span] given a starting [SourceOffset] and an ending [SourceOffset].
	///
	/// Asserts: start <= end
	#[inline]
	pub const fn new(start: SourceOffset, end: SourceOffset) -> Self {
		debug_assert!(start.0 <= end.0);
		Self { start, end }
	}

	/// Gets the starting [SourceOffset].
	#[inline]
	pub const fn start(&self) -> SourceOffset {
		self.start
	}

	/// Gets the ending [SourceOffset].
	#[inline]
	pub const fn end(&self) -> SourceOffset {
		self.end
	}

	/// Extends this [Span] into a new one with the end altered to be [SourceOffset].
	///
	/// Asserts: start <= end
	#[inline]
	pub fn with_end(self, end: SourceOffset) -> Self {
		debug_assert!(self.start <= end);
		Self { start: self.start, end }
	}

	/// Checks if the given [Span] would fit entirely within this [Span].
	pub fn contains(&self, span: Span) -> bool {
		self.start <= span.start && span.end <= self.end
	}

	/// Checks if this [Span] overlaps with the given [Span] (i.e. they share at least one byte).
	pub fn overlaps(&self, span: Span) -> bool {
		self.start < span.end && span.start < self.end
	}

	/// Checks if the [Span] has no length.
	pub const fn is_empty(&self) -> bool {
		self.start.0 == self.end.0
	}

	/// Returns the length of the [Span].
	pub fn len(&self) -> u32 {
		debug_assert!(self.start <= self.end);
		self.end.0 - self.start.0
	}

	/// Given a string `source`, establish the line number and column number that this span would reside in.
	pub fn line_and_column(self, source: &'_ str) -> (u32, u32) {
		let mut line = 0;
		let mut column = 0;
		let mut offset = self.start.0;
		for char in source.chars() {
			if offset == 0 {
				break;
			}
			if char == '\n' {
				column = 0;
				line += 1;
			} else {
				column += 1;
			}
			offset -= char.len_utf8() as u32;
		}
		(line, column)
	}
}

/// Precomputed line-start offsets for a source string, enabling O(log lines) line/column lookup.
///
/// Building the index is a single linear pass over the source; each subsequent [`LineIndex::line_and_column`] is a
/// binary search plus a per-line character count. Prefer this over [`Span::line_and_column`] whenever
/// resolving more than a couple of spans against one source.
#[derive(Debug, Clone)]
pub struct LineIndex<'a, A: Allocator = Global> {
	source: &'a str,
	/// Byte offset of the start of each line. Always begins with 0.
	line_starts: Box<[u32], A>,
}

impl<'a> LineIndex<'a, Global> {
	/// Build a line index for `source` in a single pass.
	pub fn new(source: &'a str) -> Self {
		Self::new_in(source, Global)
	}
}

impl<'a, A: Allocator> LineIndex<'a, A> {
	/// Build a line index for `source` in a single pass, allocating the backing table in `alloc`.
	pub fn new_in(source: &'a str, alloc: A) -> Self {
		let mut line_starts = allocator_api2::vec::Vec::with_capacity_in(source.len() / 32 + 1, alloc);
		line_starts.push(0);
		for (i, b) in source.bytes().enumerate() {
			if b == b'\n' {
				line_starts.push(i as u32 + 1);
			}
		}
		Self { source, line_starts: line_starts.into_boxed_slice() }
	}

	/// Returns the zero-based `(line, column)` of the span's start, where column counts Unicode
	/// scalar values from the line start. Matches [`Span::line_and_column`] exactly.
	pub fn line_and_column(&self, span: Span) -> (u32, u32) {
		let line_starts = &self.line_starts[..];
		let offset = span.start().0;
		// Largest line whose start is <= offset.
		let line = line_starts.partition_point(|&start| start <= offset) - 1;
		let line_start = line_starts[line] as usize;
		let end = (offset as usize).min(self.source.len());
		// Count characters (not bytes) within the line up to the offset.
		let column = self.source[line_start..end].chars().count() as u32;
		(line as u32, column)
	}
}

/// Extends this [Span], ensuring that the resulting new [Span] is broader than both this and the given [Span].
/// In other words the resulting span will always [Span::contains()] both [Spans][Span].
impl Add for Span {
	type Output = Self;
	fn add(self, rhs: Self) -> Self::Output {
		if rhs == Span::DUMMY {
			return self;
		}
		if self == Span::DUMMY {
			return rhs;
		}
		let start = if self.start < rhs.start { self.start } else { rhs.start };
		let end = if self.end > rhs.end { self.end } else { rhs.end };
		Self { start, end }
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "[{}..{})", self.start.0, self.end.0)
	}
}

#[cfg(feature = "miette")]
impl From<Span> for miette::SourceSpan {
	fn from(val: Span) -> Self {
		Self::new(miette::SourceOffset::from(val.start.0 as usize), val.len() as usize)
	}
}

impl<T: ToSpan> ToSpan for Vec<T> {
	fn to_span(&self) -> Span {
		let mut span = Span::ZERO;
		for item in self {
			if span == Span::ZERO {
				span = item.to_span();
			} else {
				span = span + item.to_span()
			}
		}
		span
	}
}

impl<T: ToSpan, A: allocator_api2::alloc::Allocator> ToSpan for allocator_api2::vec::Vec<T, A> {
	fn to_span(&self) -> Span {
		let mut span = Span::ZERO;
		for item in self {
			if span == Span::ZERO {
				span = item.to_span();
			} else {
				span = span + item.to_span()
			}
		}
		span
	}
}

#[cfg(feature = "bumpalo")]
impl<'a, T: ToSpan> ToSpan for bumpalo::collections::Vec<'a, T> {
	fn to_span(&self) -> Span {
		let mut span = Span::ZERO;
		for item in self {
			if span == Span::ZERO {
				span = item.to_span();
			} else {
				span = span + item.to_span()
			}
		}
		span
	}
}

macro_rules! impl_tuple {
    ($len:tt: $($name:ident),+) => {
        impl<$($name: ToSpan),+> ToSpan for ($($name),+) {
            fn to_span(&self) -> Span {
                self.0.to_span() + self.$len.to_span()
            }
        }
    };
}
impl_tuple!(1: A, B);
impl_tuple!(2: A, B, C);
impl_tuple!(3: A, B, C, D);
impl_tuple!(4: A, B, C, D, E);
impl_tuple!(5: A, B, C, D, E, F);
impl_tuple!(6: A, B, C, D, E, F, G);
impl_tuple!(7: A, B, C, D, E, F, G, H);
impl_tuple!(8: A, B, C, D, E, F, G, H, I);
impl_tuple!(9: A, B, C, D, E, F, G, H, I, J);
impl_tuple!(10: A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(11: A, B, C, D, E, F, G, H, I, J, K, L);

impl<T: ToSpan> ToSpan for Option<T> {
	fn to_span(&self) -> Span {
		self.as_ref().map_or(Span::DUMMY, |t| t.to_span())
	}
}

impl<T> ToSpan for PhantomData<T> {
	fn to_span(&self) -> Span {
		Span::DUMMY
	}
}

/// A trait representing an object that can derive its own [Span]. This is very similar to `From<MyStuct> for Span`,
/// however `From<MyStruct> for Span` requires `Sized`, meaning it is not `dyn` compatible.
pub trait ToSpan {
	fn to_span(&self) -> Span;
}

impl ToSpan for Span {
	fn to_span(&self) -> Span {
		*self
	}
}

impl<T: ToSpan> ToSpan for &T {
	fn to_span(&self) -> Span {
		(**self).to_span()
	}
}

impl<T: ToSpan> ToSpan for &mut T {
	fn to_span(&self) -> Span {
		(**self).to_span()
	}
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_span_vec() {
		let mut vec = vec![];
		vec.push(Span::new(SourceOffset(3), SourceOffset(10)));
		vec.push(Span::new(SourceOffset(13), SourceOffset(15)));
		assert_eq!(vec.to_span(), Span::new(SourceOffset(3), SourceOffset(15)));
	}

	#[test]
	fn line_index_matches_scan() {
		// Includes a multibyte char (é = 2 bytes) so column-by-char and byte offsets diverge.
		let source = "a\nbc\n\ndéf\nghi";
		let index = LineIndex::new(source);
		for offset in 0..=source.len() as u32 {
			// Only test offsets on char boundaries, as spans always land on them.
			if !source.is_char_boundary(offset as usize) {
				continue;
			}
			let span = Span::new(SourceOffset(offset), SourceOffset(offset));
			assert_eq!(index.line_and_column(span), span.line_and_column(source), "offset {offset}");
		}
	}

	#[test]
	fn line_index_empty_source() {
		let index = LineIndex::new("");
		let span = Span::new(SourceOffset(0), SourceOffset(0));
		assert_eq!(index.line_and_column(span), (0, 0));
	}

	#[test]
	fn line_index_new_in_matches_new() {
		let source = "a\nbc\n\ndéf\nghi";
		let index = LineIndex::new_in(source, Global);
		for offset in 0..=source.len() as u32 {
			if !source.is_char_boundary(offset as usize) {
				continue;
			}
			let span = Span::new(SourceOffset(offset), SourceOffset(offset));
			assert_eq!(index.line_and_column(span), span.line_and_column(source), "offset {offset}");
		}
	}
}
