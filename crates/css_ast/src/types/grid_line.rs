#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, keyword_set};
use csskit_derives::{IntoSpan, Parse, Peek, ToCursors};

keyword_set!(GridLineKeywords { Auto: "auto", Span: "span" });

// https://drafts.csswg.org/css-grid-2/#typedef-grid-row-start-grid-line
// <grid-line> = auto | <custom-ident> | [ [ <integer [-∞,-1]> | <integer [1,∞]> ] && <custom-ident>? ] | [ span && [ <integer [1,∞]> || <custom-ident> ] ]
#[derive(IntoSpan, ToCursors, Peek, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum GridLine {
	Auto(GridLineKeywords),
}

impl<'a> Parse<'a> for GridLine {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(keyword) = p.parse_if_peek::<GridLineKeywords>()? {
			match keyword {
				GridLineKeywords::Auto(_) => return Ok(GridLine::Auto(keyword)),
				_ => todo!(),
			}
		}

		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<GridLine>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(GridLine, "auto");
	}
}
