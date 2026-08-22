use crate::{AssociatedWhitespaceRules, CommentStyle, Cursor, Kind, KindSet, QuoteStyle, Token};

impl From<Cursor> for Token {
	fn from(cursor: Cursor) -> Self {
		cursor.token()
	}
}

impl PartialEq<Token> for Cursor {
	fn eq(&self, other: &Token) -> bool {
		self.token() == *other
	}
}

impl From<Cursor> for Kind {
	fn from(cursor: Cursor) -> Self {
		cursor.token().kind()
	}
}

impl PartialEq<Kind> for Cursor {
	fn eq(&self, other: &Kind) -> bool {
		self.token() == *other
	}
}

impl PartialEq<CommentStyle> for Cursor {
	fn eq(&self, other: &CommentStyle) -> bool {
		self.token() == *other
	}
}

impl From<Cursor> for KindSet {
	fn from(cursor: Cursor) -> Self {
		cursor.token().into()
	}
}

impl PartialEq<KindSet> for Cursor {
	fn eq(&self, other: &KindSet) -> bool {
		self.token() == *other
	}
}

impl From<Cursor> for QuoteStyle {
	fn from(cursor: Cursor) -> Self {
		cursor.token().into()
	}
}

impl PartialEq<QuoteStyle> for Cursor {
	fn eq(&self, other: &QuoteStyle) -> bool {
		self.token() == *other
	}
}

impl PartialEq<AssociatedWhitespaceRules> for Cursor {
	fn eq(&self, other: &AssociatedWhitespaceRules) -> bool {
		self.token() == *other
	}
}

impl PartialEq<CommentStyle> for &Cursor {
	fn eq(&self, other: &CommentStyle) -> bool {
		self.token() == *other
	}
}

impl PartialEq<Kind> for &Cursor {
	fn eq(&self, other: &Kind) -> bool {
		self.token() == *other
	}
}

impl PartialEq<KindSet> for &Cursor {
	fn eq(&self, other: &KindSet) -> bool {
		self.token() == *other
	}
}

impl PartialEq<QuoteStyle> for &Cursor {
	fn eq(&self, other: &QuoteStyle) -> bool {
		self.token() == *other
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Cursor>(), 12);
}
