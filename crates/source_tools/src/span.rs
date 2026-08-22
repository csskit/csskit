use crate::SourceOffset;
use allocator_api2::alloc::{Allocator, Global};
use allocator_api2::boxed::Box;
use core::{fmt::Display, hash::Hash, marker::PhantomData, ops::Add};

/// A half-open byte range in source text.
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct Span {
	start: SourceOffset,
	end: SourceOffset,
}

impl Span {
	/// A synthetic span.
	pub const DUMMY: Self = Self::new(SourceOffset::DUMMY, SourceOffset::DUMMY);

	/// An empty span at start of source text.
	pub const ZERO: Self = Self::new(SourceOffset::ZERO, SourceOffset::ZERO);

	/// Creates a span between two offsets.
	#[inline]
	pub const fn new(start: SourceOffset, end: SourceOffset) -> Self {
		debug_assert!(start.0 <= end.0);
		Self { start, end }
	}

	/// Returns starting offset.
	#[inline]
	pub const fn start(&self) -> SourceOffset {
		self.start
	}

	/// Returns ending offset.
	#[inline]
	pub const fn end(&self) -> SourceOffset {
		self.end
	}

	/// Returns a copy ending at `end`.
	#[inline]
	pub const fn with_end(self, end: SourceOffset) -> Self {
		debug_assert!(self.start.0 <= end.0);
		Self { start: self.start, end }
	}

	/// Returns whether `span` is entirely within this span.
	pub const fn contains(&self, span: Span) -> bool {
		self.start.0 <= span.start.0 && span.end.0 <= self.end.0
	}

	/// Returns whether spans share at least one byte.
	pub const fn overlaps(&self, span: Span) -> bool {
		self.start.0 < span.end.0 && span.start.0 < self.end.0
	}

	/// Returns whether span contains no bytes.
	pub const fn is_empty(&self) -> bool {
		self.start.0 == self.end.0
	}

	/// Returns length in bytes.
	pub const fn len(&self) -> u32 {
		debug_assert!(self.start.0 <= self.end.0);
		self.end.0 - self.start.0
	}

	/// Returns covered source text.
	pub fn str_slice<'a>(&self, source: &'a str) -> &'a str {
		&source[self.start.0 as usize..self.end.0 as usize]
	}

	/// Returns zero-based line and column of span start.
	pub fn line_and_column(self, source: &str) -> (u32, u32) {
		let mut line = 0;
		let mut column = 0;
		let mut offset = self.start.0;
		for character in source.chars() {
			if offset == 0 {
				break;
			}
			if character == '\n' {
				column = 0;
				line += 1;
			} else {
				column += 1;
			}
			offset -= character.len_utf8() as u32;
		}
		(line, column)
	}
}

/// Precomputed line starts for repeated line and column lookup.
#[derive(Debug, Clone)]
pub struct LineIndex<'a, A: Allocator = Global> {
	source: &'a str,
	line_starts: Box<[u32], A>,
}

impl<'a> LineIndex<'a, Global> {
	/// Builds an index for `source`.
	pub fn new(source: &'a str) -> Self {
		Self::new_in(source, Global)
	}
}

impl<'a, A: Allocator> LineIndex<'a, A> {
	/// Builds an index using `alloc`.
	pub fn new_in(source: &'a str, alloc: A) -> Self {
		let mut line_starts = allocator_api2::vec::Vec::with_capacity_in(source.len() / 32 + 1, alloc);
		line_starts.push(0);
		for (index, byte) in source.bytes().enumerate() {
			if byte == b'\n' {
				line_starts.push(index as u32 + 1);
			}
		}
		Self { source, line_starts: line_starts.into_boxed_slice() }
	}

	/// Returns zero-based line and column of span start.
	pub fn line_and_column(&self, span: Span) -> (u32, u32) {
		let starts = &self.line_starts[..];
		let offset = span.start().0;
		let line = starts.partition_point(|&start| start <= offset) - 1;
		let line_start = starts[line] as usize;
		let end = (offset as usize).min(self.source.len());
		let column = self.source[line_start..end].chars().count() as u32;
		(line as u32, column)
	}
}

impl Add for Span {
	type Output = Self;

	fn add(self, rhs: Self) -> Self::Output {
		if rhs == Self::DUMMY {
			return self;
		}
		if self == Self::DUMMY {
			return rhs;
		}
		Self { start: SourceOffset(self.start.0.min(rhs.start.0)), end: SourceOffset(self.end.0.max(rhs.end.0)) }
	}
}

impl Display for Span {
	fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
		write!(f, "[{}..{})", self.start.0, self.end.0)
	}
}

#[cfg(feature = "miette")]
impl From<Span> for miette::SourceSpan {
	fn from(value: Span) -> Self {
		Self::new(miette::SourceOffset::from(value.start.0 as usize), value.len() as usize)
	}
}

/// A value that can return its source span.
pub trait ToSpan {
	/// Returns source span.
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

impl<T: ToSpan> ToSpan for Option<T> {
	fn to_span(&self) -> Span {
		self.as_ref().map_or(Span::DUMMY, ToSpan::to_span)
	}
}

impl<T> ToSpan for PhantomData<T> {
	fn to_span(&self) -> Span {
		Span::DUMMY
	}
}

impl<T: ToSpan> ToSpan for [T] {
	fn to_span(&self) -> Span {
		self.iter().fold(Span::DUMMY, |span, item| span + item.to_span())
	}
}

impl<T: ToSpan> ToSpan for Vec<T> {
	fn to_span(&self) -> Span {
		self.as_slice().to_span()
	}
}

impl<T: ToSpan, A: Allocator> ToSpan for allocator_api2::vec::Vec<T, A> {
	fn to_span(&self) -> Span {
		self.as_slice().to_span()
	}
}

macro_rules! impl_tuple {
	($($name:ident),+) => {
		impl<$($name: ToSpan),+> ToSpan for ($($name,)+) {
			#[allow(non_snake_case)]
			fn to_span(&self) -> Span {
				let ($($name,)+) = self;
				Span::DUMMY $(+ $name.to_span())+
			}
		}
	};
}

impl_tuple!(A, B);
impl_tuple!(A, B, C);
impl_tuple!(A, B, C, D);
impl_tuple!(A, B, C, D, E);
impl_tuple!(A, B, C, D, E, F);
impl_tuple!(A, B, C, D, E, F, G);
impl_tuple!(A, B, C, D, E, F, G, H);
impl_tuple!(A, B, C, D, E, F, G, H, I);
impl_tuple!(A, B, C, D, E, F, G, H, I, J);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K);
impl_tuple!(A, B, C, D, E, F, G, H, I, J, K, L);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn span_layout_and_ranges() {
		let span = Span::new(SourceOffset(2), SourceOffset(5));
		assert_eq!(size_of::<Span>(), 8);
		assert_eq!(span.len(), 3);
		assert_eq!(span.str_slice("abcdef"), "cde");
		assert!(span.contains(Span::new(SourceOffset(3), SourceOffset(4))));
		assert!(!span.overlaps(Span::new(SourceOffset(5), SourceOffset(6))));
	}

	#[test]
	fn empty_collections_have_dummy_span() {
		let spans: Vec<Span> = vec![];
		assert_eq!(spans.to_span(), Span::DUMMY);
	}

	#[test]
	fn line_index_matches_scan() {
		let source = "one\ntwø\nthree";
		let index = LineIndex::new(source);
		for offset in source.char_indices().map(|(offset, _)| offset).chain([source.len()]) {
			let offset = offset as u32;
			let span = Span::new(SourceOffset(offset), SourceOffset(offset));
			assert_eq!(index.line_and_column(span), span.line_and_column(source));
		}
	}
}
