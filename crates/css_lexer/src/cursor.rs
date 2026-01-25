use crate::{AssociatedWhitespaceRules, CommentStyle, Kind, KindSet, QuoteStyle, SourceOffset, Span, ToSpan, Token};

/// Wraps [Token] with a [SourceOffset], allows it to reason about the character data of the source text.
#[derive(Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cursor(SourceOffset, Token);

impl Cursor {
	pub const DUMMY_SITE_NUMBER_ZERO: Self = Self(SourceOffset::DUMMY, Token::NUMBER_ZERO);
	pub const EMPTY: Self = Self(SourceOffset::ZERO, Token::EMPTY);

	#[inline(always)]
	pub const fn new(offset: SourceOffset, token: Token) -> Self {
		Self(offset, token)
	}

	#[inline(always)]
	pub const fn dummy(token: Token) -> Self {
		Self(SourceOffset::DUMMY, token)
	}

	#[inline(always)]
	pub const fn token(&self) -> Token {
		self.1
	}

	#[inline(always)]
	pub const fn offset(&self) -> SourceOffset {
		self.0
	}

	#[inline(always)]
	pub fn end_offset(&self) -> SourceOffset {
		if self.offset() == SourceOffset::DUMMY {
			return self.offset();
		}
		SourceOffset(self.offset().0 + self.len())
	}

	#[inline(always)]
	pub const fn is_empty(&self) -> bool {
		self.token().is_empty()
	}

	#[inline(always)]
	pub const fn len(&self) -> u32 {
		self.token().len()
	}

	#[inline(always)]
	pub fn span(&self) -> Span {
		Span::new(self.offset(), self.end_offset())
	}

	#[inline(always)]
	pub fn str_slice<'a>(&self, str: &'a str) -> &'a str {
		debug_assert!(
			str.len() >= (self.end_offset().0 as usize),
			"attempted to index out of bounds ({} < {})",
			str.len(),
			self.end_offset().0
		);
		&str[(self.offset().0 as usize)..(self.end_offset().0 as usize)]
	}

	pub fn with_quotes(&self, quote_style: QuoteStyle) -> Self {
		if *self == quote_style || *self != Kind::String {
			return *self;
		}
		Self::new(self.offset(), self.token().with_quotes(quote_style))
	}

	pub fn with_associated_whitespace(&self, rules: AssociatedWhitespaceRules) -> Self {
		debug_assert!(self.1 == KindSet::DELIM_LIKE);
		if self.1.associated_whitespace().to_bits() == rules.to_bits() {
			return *self;
		}
		Self::new(self.offset(), self.token().with_associated_whitespace(rules))
	}

	/// Returns a new [Cursor] with the `sign_is_required` flag set on the token.
	/// This indicates that the `+` sign should be preserved during minification.
	///
	/// Asserts: the token `kind()` is [Kind::Number].
	pub fn with_sign_required(&self) -> Self {
		debug_assert!(self.1 == Kind::Number);
		Self::new(self.offset(), self.token().with_sign_required())
	}

	#[inline]
	pub fn atom_bits(&self) -> u32 {
		self.1.atom_bits()
	}
}

impl From<Cursor> for Token {
	fn from(cursor: Cursor) -> Self {
		cursor.token()
	}
}

impl PartialEq<Token> for Cursor {
	fn eq(&self, other: &Token) -> bool {
		self.1 == *other
	}
}

impl ToSpan for Cursor {
	fn to_span(&self) -> Span {
		self.span()
	}
}

impl From<Cursor> for Span {
	fn from(cursor: Cursor) -> Self {
		cursor.span()
	}
}

impl PartialEq<Span> for Cursor {
	fn eq(&self, other: &Span) -> bool {
		self.span() == *other
	}
}

impl From<Cursor> for Kind {
	fn from(cursor: Cursor) -> Self {
		cursor.token().kind()
	}
}

impl PartialEq<Kind> for Cursor {
	fn eq(&self, other: &Kind) -> bool {
		self.1 == *other
	}
}

impl PartialEq<CommentStyle> for Cursor {
	fn eq(&self, other: &CommentStyle) -> bool {
		self.1 == *other
	}
}

impl From<Cursor> for KindSet {
	fn from(cursor: Cursor) -> Self {
		cursor.token().into()
	}
}

impl PartialEq<KindSet> for Cursor {
	fn eq(&self, other: &KindSet) -> bool {
		self.1 == *other
	}
}

impl From<Cursor> for QuoteStyle {
	fn from(cursor: Cursor) -> Self {
		cursor.token().into()
	}
}

impl PartialEq<QuoteStyle> for Cursor {
	fn eq(&self, other: &QuoteStyle) -> bool {
		self.1 == *other
	}
}

impl PartialEq<AssociatedWhitespaceRules> for Cursor {
	fn eq(&self, other: &AssociatedWhitespaceRules) -> bool {
		self.1 == *other
	}
}

impl PartialEq<char> for Cursor {
	fn eq(&self, other: &char) -> bool {
		self.1 == *other
	}
}

impl PartialEq<CommentStyle> for &Cursor {
	fn eq(&self, other: &CommentStyle) -> bool {
		self.1 == *other
	}
}

impl PartialEq<Kind> for &Cursor {
	fn eq(&self, other: &Kind) -> bool {
		self.1 == *other
	}
}

impl PartialEq<KindSet> for &Cursor {
	fn eq(&self, other: &KindSet) -> bool {
		self.1 == *other
	}
}

impl PartialEq<QuoteStyle> for &Cursor {
	fn eq(&self, other: &QuoteStyle) -> bool {
		self.1 == *other
	}
}

impl PartialEq<char> for &Cursor {
	fn eq(&self, other: &char) -> bool {
		self.1 == *other
	}
}

#[cfg(feature = "miette")]
impl From<Cursor> for miette::SourceSpan {
	fn from(val: Cursor) -> Self {
		let span = val.span();
		span.into()
	}
}

#[cfg(feature = "serde")]
impl serde::ser::Serialize for Cursor {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: serde::ser::Serializer,
	{
		use serde::ser::SerializeStruct;
		if self.token() == Token::EMPTY {
			return serializer.serialize_none();
		}
		let mut state = serializer.serialize_struct("Cursor", 3)?;
		state.serialize_field("kind", self.token().kind().as_str())?;
		state.serialize_field("offset", &self.offset())?;
		state.serialize_field("len", &self.token().len())?;
		state.end()
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Cursor>(), 12);
}
