use crate::{SourceOffset, Span, ToSpan};

/// A token that occupies bytes in source text.
pub trait SourceToken: Copy {
	/// Token category.
	type Kind: Copy;

	/// Token occupying no source bytes.
	const EMPTY: Self;

	/// Returns token category.
	fn kind(self) -> Self::Kind;

	/// Returns token length in bytes.
	fn len(self) -> u32;

	/// Returns token category name.
	fn kind_name(self) -> &'static str;

	/// Returns bytes before token value.
	fn leading_len(self) -> u32 {
		0
	}

	/// Returns bytes after token value.
	fn trailing_len(self) -> u32 {
		0
	}

	/// Returns whether token occupies no bytes.
	fn is_empty(self) -> bool {
		self.len() == 0
	}
}

/// A token and its byte offset in source text.
#[repr(C)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cursor<T>(SourceOffset, T);

impl<T> Cursor<T> {
	/// Places `token` at `offset`.
	#[inline(always)]
	pub const fn new(offset: SourceOffset, token: T) -> Self {
		Self(offset, token)
	}

	/// Places a synthetic token at [SourceOffset::DUMMY].
	#[inline(always)]
	pub const fn dummy(token: T) -> Self {
		Self(SourceOffset::DUMMY, token)
	}

	/// Returns token by reference.
	#[inline(always)]
	pub const fn token_ref(&self) -> &T {
		&self.1
	}

	/// Returns starting offset.
	#[inline(always)]
	pub const fn offset(&self) -> SourceOffset {
		self.0
	}

	/// Replaces token while preserving offset.
	#[inline(always)]
	pub fn with_token<U>(self, token: U) -> Cursor<U> {
		Cursor(self.0, token)
	}

	/// Maps token while preserving offset.
	#[inline(always)]
	pub fn map_token<U>(self, map: impl FnOnce(T) -> U) -> Cursor<U> {
		Cursor(self.0, map(self.1))
	}
}

impl<T: Copy> Cursor<T> {
	/// Returns token.
	#[inline(always)]
	pub const fn token(&self) -> T {
		self.1
	}
}

impl<T: SourceToken> Cursor<T> {
	/// Empty cursor at start of source text.
	pub const EMPTY: Self = Self(SourceOffset::ZERO, T::EMPTY);

	/// Returns token category.
	#[inline(always)]
	pub fn kind(&self) -> T::Kind {
		self.1.kind()
	}

	/// Returns offset immediately after token.
	#[inline(always)]
	pub fn end_offset(&self) -> SourceOffset {
		if self.0 == SourceOffset::DUMMY {
			return self.0;
		}
		SourceOffset(self.0.0 + self.len())
	}

	/// Returns token length in bytes.
	#[inline(always)]
	pub fn len(&self) -> u32 {
		self.1.len()
	}

	/// Returns whether token occupies no bytes.
	#[inline(always)]
	pub fn is_empty(&self) -> bool {
		self.1.is_empty()
	}

	/// Returns token source span.
	#[inline(always)]
	pub fn span(&self) -> Span {
		Span::new(self.0, self.end_offset())
	}

	/// Returns source text covered by token.
	#[inline(always)]
	pub fn str_slice<'a>(&self, source: &'a str) -> &'a str {
		let start = self.0.0 as usize;
		let end = self.end_offset().0 as usize;
		debug_assert!(source.len() >= end, "attempted to index out of bounds ({} < {})", source.len(), end);
		&source[start..end]
	}

	/// Returns token value with leading and trailing syntax removed.
	pub fn value_slice<'a>(&self, source: &'a str) -> &'a str {
		let start = (self.0.0 + self.1.leading_len()) as usize;
		let end = (self.0.0 + self.len() - self.1.trailing_len()) as usize;
		&source[start..end]
	}
}

impl<T: SourceToken> ToSpan for Cursor<T> {
	fn to_span(&self) -> Span {
		self.span()
	}
}

impl<T: SourceToken> From<Cursor<T>> for Span {
	fn from(cursor: Cursor<T>) -> Self {
		cursor.span()
	}
}

impl<T: SourceToken> PartialEq<Span> for Cursor<T> {
	fn eq(&self, other: &Span) -> bool {
		self.span() == *other
	}
}

impl<T: PartialEq<char>> PartialEq<char> for Cursor<T> {
	fn eq(&self, other: &char) -> bool {
		self.1 == *other
	}
}

impl<T: PartialEq<char>> PartialEq<char> for &Cursor<T> {
	fn eq(&self, other: &char) -> bool {
		self.token_ref() == other
	}
}

#[cfg(feature = "miette")]
impl<T: SourceToken> From<Cursor<T>> for miette::SourceSpan {
	fn from(cursor: Cursor<T>) -> Self {
		cursor.span().into()
	}
}

#[cfg(feature = "serde")]
impl<T: SourceToken> serde::Serialize for Cursor<T> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::Serializer,
	{
		use serde::ser::SerializeStruct;
		if self.is_empty() {
			return serializer.serialize_none();
		}
		let mut state = serializer.serialize_struct("Cursor", 3)?;
		state.serialize_field("kind", self.1.kind_name())?;
		state.serialize_field("offset", &self.0)?;
		state.serialize_field("len", &self.len())?;
		state.end()
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[repr(C)]
	struct Token(u32, u32);

	impl SourceToken for Token {
		type Kind = u32;
		const EMPTY: Self = Token(0, 0);

		fn kind(self) -> Self::Kind {
			self.0
		}

		fn len(self) -> u32 {
			self.1
		}

		fn kind_name(self) -> &'static str {
			"test"
		}
	}

	#[test]
	fn cursor_preserves_layout_and_span() {
		let cursor = Cursor::new(SourceOffset(3), Token(1, 4));
		assert_eq!(size_of::<Cursor<Token>>(), 12);
		assert_eq!(cursor.span(), Span::new(SourceOffset(3), SourceOffset(7)));
	}

	#[test]
	fn dummy_cursor_keeps_dummy_end_offset() {
		let cursor = Cursor::dummy(Token(1, 4));
		assert_eq!(cursor.end_offset(), SourceOffset::DUMMY);
	}
}
