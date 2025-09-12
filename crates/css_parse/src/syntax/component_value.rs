use crate::{
	AssociatedWhitespaceRules, Cursor, CursorSink, FunctionBlock, Kind, KindSet, Parse, Parser, Peek,
	Result as ParserResult, SimpleBlock, Span, State, T, ToCursors, ToSpan, diagnostics,
};

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
// A compatible "Token" per CSS grammar, subsetted to the tokens possibly
// rendered by ComponentValue (so no pairwise, function tokens, etc).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum ComponentValue<'a> {
	SimpleBlock(SimpleBlock<'a>),
	Function(FunctionBlock<'a>),
	Whitespace(T![Whitespace]),
	Number(T![Number]),
	Dimension(T![Dimension]),
	Ident(T![Ident]),
	AtKeyword(T![AtKeyword]),
	Hash(T![Hash]),
	String(T![String]),
	Url(T![Url]),
	Delim(T![Delim]),
	Colon(T![:]),
	Semicolon(T![;]),
	Comma(T![,]),
}

impl<'a> Peek<'a> for ComponentValue<'a> {
	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		let kindset = KindSet::new(&[
			Kind::Whitespace,
			Kind::Number,
			Kind::Dimension,
			Kind::Ident,
			Kind::AtKeyword,
			Kind::Hash,
			Kind::String,
			Kind::Url,
			Kind::Delim,
			Kind::Colon,
			Kind::Semicolon,
			Kind::Comma,
			Kind::Function,
			Kind::LeftCurly,
			Kind::LeftParen,
			Kind::LeftSquare,
		]);
		c == kindset
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-component-value
impl<'a> Parse<'a> for ComponentValue<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![' ']>() {
			p.parse::<T![' ']>().map(Self::Whitespace)
		} else if p.peek::<T![PairWiseStart]>() {
			let old_state = p.set_state(State::Nested);
			let block = p.parse::<SimpleBlock>();
			p.set_state(old_state);
			Ok(Self::SimpleBlock(block?))
		} else if p.peek::<T![Function]>() {
			p.parse::<FunctionBlock>().map(Self::Function)
		} else if p.peek::<T![Number]>() {
			p.parse::<T![Number]>().map(Self::Number)
		} else if p.peek::<T![Dimension]>() {
			p.parse::<T![Dimension]>().map(Self::Dimension)
		} else if p.peek::<T![Ident]>() {
			p.parse::<T![Ident]>().map(Self::Ident)
		} else if p.peek::<T![AtKeyword]>() {
			p.parse::<T![AtKeyword]>().map(Self::AtKeyword)
		} else if p.peek::<T![Hash]>() {
			p.parse::<T![Hash]>().map(Self::Hash)
		} else if p.peek::<T![String]>() {
			p.parse::<T![String]>().map(Self::String)
		} else if p.peek::<T![Url]>() {
			p.parse::<T![Url]>().map(Self::Url)
		} else if p.peek::<T![Delim]>() {
			p.parse::<T![Delim]>().map(|delim| {
				// Carefully handle Whitespace rules to ensure whitespace isn't lost when re-serializing
				let mut rules = AssociatedWhitespaceRules::none();
				if p.peek_next_including_whitespace() == Kind::Whitespace {
					rules |= AssociatedWhitespaceRules::EnforceAfter;
				} else {
					rules |= AssociatedWhitespaceRules::BanAfter;
				}
				Self::Delim(delim.with_associated_whitespace(rules))
			})
		} else if p.peek::<T![:]>() {
			p.parse::<T![:]>().map(Self::Colon)
		} else if p.peek::<T![;]>() {
			p.parse::<T![;]>().map(Self::Semicolon)
		} else if p.peek::<T![,]>() {
			p.parse::<T![,]>().map(Self::Comma)
		} else {
			Err(diagnostics::Unexpected(p.next()))?
		}
	}
}

impl<'a> ToCursors for ComponentValue<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::SimpleBlock(t) => ToCursors::to_cursors(t, s),
			Self::Function(t) => ToCursors::to_cursors(t, s),
			Self::Ident(t) => ToCursors::to_cursors(t, s),
			Self::AtKeyword(t) => ToCursors::to_cursors(t, s),
			Self::Hash(t) => ToCursors::to_cursors(t, s),
			Self::String(t) => ToCursors::to_cursors(t, s),
			Self::Url(t) => ToCursors::to_cursors(t, s),
			Self::Delim(t) => ToCursors::to_cursors(t, s),
			Self::Number(t) => ToCursors::to_cursors(t, s),
			Self::Dimension(t) => ToCursors::to_cursors(t, s),
			Self::Whitespace(t) => ToCursors::to_cursors(t, s),
			Self::Colon(t) => ToCursors::to_cursors(t, s),
			Self::Semicolon(t) => ToCursors::to_cursors(t, s),
			Self::Comma(t) => ToCursors::to_cursors(t, s),
		}
	}
}

impl<'a> ToSpan for ComponentValue<'a> {
	fn to_span(&self) -> Span {
		match self {
			Self::SimpleBlock(t) => t.to_span(),
			Self::Function(t) => t.to_span(),
			Self::Ident(t) => t.to_span(),
			Self::AtKeyword(t) => t.to_span(),
			Self::Hash(t) => t.to_span(),
			Self::String(t) => t.to_span(),
			Self::Url(t) => t.to_span(),
			Self::Delim(t) => t.to_span(),
			Self::Number(t) => t.to_span(),
			Self::Dimension(t) => t.to_span(),
			Self::Whitespace(t) => t.to_span(),
			Self::Colon(t) => t.to_span(),
			Self::Semicolon(t) => t.to_span(),
			Self::Comma(t) => t.to_span(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ComponentValue>(), 72);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ComponentValue, "foo");
		assert_parse!(ComponentValue, " ");
		assert_parse!(ComponentValue, "{block}");
	}
}
