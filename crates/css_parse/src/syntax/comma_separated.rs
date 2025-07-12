use crate::{CursorSink, Parse, Parser, Peek, Result as ParserResult, ToCursors, token_macros};
use bumpalo::collections::Vec;
use css_lexer::{KindSet, Span, ToSpan};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct CommaSeparated<'a, T: Peek<'a> + Parse<'a> + ToCursors + ToSpan> {
	items: Vec<'a, (T, token_macros::Comma)>,
	last: Option<T>,
}

impl<'a, T: Peek<'a> + Parse<'a> + ToCursors + ToSpan> Peek<'a> for CommaSeparated<'a, T> {
	const PEEK_KINDSET: KindSet = T::PEEK_KINDSET;
	fn peek(p: &Parser<'a>, c: css_lexer::Cursor) -> bool {
		T::peek(p, c)
	}
}

impl<'a, T: Peek<'a> + Parse<'a> + ToCursors + ToSpan> Parse<'a> for CommaSeparated<'a, T> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut items = Vec::new_in(p.bump());
		loop {
			let last = p.parse::<T>()?;
			if let Some(punct) = p.parse_if_peek::<token_macros::Comma>()? {
				items.push((last, punct));
			} else {
				return Ok(Self { items, last: Some(last) });
			}
		}
	}
}

impl<'a, T: Peek<'a> + Parse<'a> + ToCursors + ToSpan> ToCursors for CommaSeparated<'a, T> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.items, s);
		ToCursors::to_cursors(&self.last, s);
	}
}

impl<'a, T: Peek<'a> + Parse<'a> + ToCursors + ToSpan> ToSpan for CommaSeparated<'a, T> {
	fn to_span(&self) -> Span {
		if self.items.is_empty() {
			debug_assert!(self.last.is_some(), "Cannot have an empty comma separated list!");
			return self.last.as_ref().unwrap().to_span();
		}
		let (ty, comma) = &self.items[0];
		let first = ty.to_span() + comma.to_span();
		if let Some(last) = &self.last {
			first + last.to_span()
		} else {
			first + self.items.last().map(|(_, p)| p.to_span()).unwrap_or(first)
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{T, test_helpers::*};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CommaSeparated<T![Ident]>>(), 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CommaSeparated<T![Ident]>, "foo");
		assert_parse!(CommaSeparated<T![Ident]>, "one,two");
		assert_parse!(CommaSeparated<T![Ident]>, "one,two,three");
	}

	#[test]
	fn test_spans() {
		assert_parse_span!(
			CommaSeparated<T![Ident]>,
			r#"
			foo bar
			^^^
		"#
		);
		assert_parse_span!(
			CommaSeparated<T![Ident]>,
			r#"
			foo, bar, baz 1
			^^^^^^^^^^^^^
		"#
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CommaSeparated<T![Ident]>, ",");
		assert_parse_error!(CommaSeparated<T![Ident]>, "one,two,three,");
		assert_parse_error!(CommaSeparated<T![Ident]>, "one two");
	}
}
