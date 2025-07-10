#![allow(warnings)]
use css_lexer::Cursor;
use css_parse::{Build, Parse, Parser, Peek, Result as ParserResult, T, diagnostics, function_set, keyword_set};
use csskit_derives::{IntoSpan, Parse, Peek, ToCursors};

keyword_set!(LeaderTypeKeywords { Dotted: "dotted", Solid: "solid", Space: "space" });

// https://drafts.csswg.org/css-content-3/#typedef-leader-type
// <leader-type> = dotted | solid | space | <string>
// dotted - Equivalent to leader(".")
// solid - Equivalent to leader("_")
// space - Equivalent to leader(" ")
#[derive(IntoSpan, ToCursors, Peek, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LeaderType {
	Keyword(LeaderTypeKeywords),
	String(T![String]),
}

impl<'a> Parse<'a> for LeaderType {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(keyword) = p.parse_if_peek::<LeaderTypeKeywords>()? {
			return Ok(Self::Keyword(keyword));
		}

		let str = p.parse::<T![String]>()?;
		let c: Cursor = str.into();
		match p.parse_str(c) {
			"." => Ok(Self::Keyword(LeaderTypeKeywords::Dotted(c))),
			"_" => Ok(Self::Keyword(LeaderTypeKeywords::Solid(c))),
			" " => Ok(Self::Keyword(LeaderTypeKeywords::Space(c))),
			_ => Ok(Self::String(str)),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LeaderType>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(LeaderType, "dotted");
		// assert_parse!(LeaderType, "'.'", "dotted"); Maybe one day we can do things like this
		assert_parse!(LeaderType, "'.'");
		assert_parse!(LeaderType, "'abc'");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(LeaderType, "foo");
	}
}
