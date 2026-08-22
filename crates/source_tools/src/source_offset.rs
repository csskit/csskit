use crate::{Cursor, SourceToken, Span};

/// A byte position in source text.
#[repr(transparent)]
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub struct SourceOffset(pub u32);

impl SourceOffset {
	/// A synthetic source position.
	pub const DUMMY: Self = Self(u32::MAX);

	/// Start of source text.
	pub const ZERO: Self = Self(0);

	/// Returns span occupied by `token` at this offset.
	pub fn as_span<T: SourceToken>(&self, token: T) -> Span {
		Span::new(*self, Self(self.0 + token.len()))
	}

	/// Places `token` at this offset.
	pub const fn as_cursor<T>(&self, token: T) -> Cursor<T> {
		Cursor::new(*self, token)
	}
}

#[cfg(feature = "miette")]
impl From<SourceOffset> for miette::SourceOffset {
	fn from(value: SourceOffset) -> Self {
		Self::from(value.0 as usize)
	}
}

impl PartialEq<u32> for SourceOffset {
	fn eq(&self, other: &u32) -> bool {
		self.0 == *other
	}
}

impl From<SourceOffset> for usize {
	fn from(value: SourceOffset) -> Self {
		value.0 as usize
	}
}
