use crate::{Cursor, SourceToken, Span, ToSpan};

/// A cursor paired with its source text.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SourceCursor<'a, T> {
	cursor: Cursor<T>,
	source: &'a str,
}

impl<'a, T> SourceCursor<'a, T> {
	/// Creates a source cursor without checking source length.
	#[inline(always)]
	pub const fn new(cursor: Cursor<T>, source: &'a str) -> Self {
		Self { cursor, source }
	}

	/// Returns cursor by reference.
	#[inline(always)]
	pub const fn cursor_ref(&self) -> &Cursor<T> {
		&self.cursor
	}

	/// Returns source text.
	#[inline(always)]
	pub const fn source(&self) -> &'a str {
		self.source
	}
}

impl<'a, T: Copy> SourceCursor<'a, T> {
	/// Returns cursor.
	#[inline(always)]
	pub const fn cursor(&self) -> Cursor<T> {
		self.cursor
	}

	/// Returns token.
	#[inline(always)]
	pub const fn token(&self) -> T {
		self.cursor.token()
	}
}

impl<'a, T: SourceToken> SourceCursor<'a, T> {
	/// Creates a source cursor and checks source length in debug builds.
	#[inline(always)]
	pub fn from(cursor: Cursor<T>, source: &'a str) -> Self {
		debug_assert_eq!(cursor.len() as usize, source.len(), "source length must match cursor length");
		Self::new(cursor, source)
	}

	/// Returns token value with leading and trailing syntax removed.
	pub fn value(&self) -> &'a str {
		let leading = self.token().leading_len() as usize;
		let trailing = self.token().trailing_len() as usize;
		&self.source[leading..self.source.len() - trailing]
	}
}

impl<T: SourceToken> ToSpan for SourceCursor<'_, T> {
	fn to_span(&self) -> Span {
		self.cursor.to_span()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::SourceOffset;

	#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	struct Token(u32);

	impl SourceToken for Token {
		type Kind = ();
		const EMPTY: Self = Token(0);

		fn kind(self) -> Self::Kind {}

		fn len(self) -> u32 {
			self.0
		}

		fn kind_name(self) -> &'static str {
			"test"
		}
	}

	#[test]
	fn binds_cursor_to_source() {
		let cursor = Cursor::new(SourceOffset(2), Token(3));
		let sourced = SourceCursor::from(cursor, "abc");
		assert_eq!(sourced.cursor(), cursor);
		assert_eq!(sourced.source(), "abc");
		assert_eq!(sourced.to_span(), Span::new(SourceOffset(2), SourceOffset(5)));
	}
}
