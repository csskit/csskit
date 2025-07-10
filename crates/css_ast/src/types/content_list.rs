#![allow(warnings)]
use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{Build, Parse, Parser, Peek, Result as ParserResult, T, diagnostics, function_set, keyword_set};
use csskit_derives::{IntoSpan, Parse, Peek, ToCursors};

use crate::types::{Image, LeaderType, Quote};

// https://drafts.csswg.org/css-content-3/#funcdef-content
type ContentFunction = crate::Todo;
// https://drafts.csswg.org/css-lists-3/#typedef-counter
type Counter = crate::Todo;
// https://drafts.csswg.org/css-content-3/#typedef-target
type Target = crate::Todo;
// https://drafts.csswg.org/css-values-5/#funcdef-attr
type AttrFunction = crate::Todo;

// https://drafts.csswg.org/css-content-3/#content-values
// <content-list> = [ <string> | <image> | <attr()> | contents | <quote> | <leader()> | <target> | <string()> | <content()> | <counter> ]+
#[derive(IntoSpan, Parse, ToCursors, Peek, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ContentList<'a>(pub Vec<'a, ContentListItem<'a>>);

keyword_set!(ContentsKeyword, "contents");
keyword_set!(StringFunctionNamePresencece {
	First: "first",
	Start: "start",
	Last: "last",
	FirstExcept: "first-except"
});
function_set!(ContentListFunctionNames { String: "string", Leader: "leader" });

#[derive(IntoSpan, ToCursors, Peek, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ContentListItem<'a> {
	String(T![String]),
	Image(Image<'a>),
	// AttrFunction(AttrFunction),
	Contents(ContentsKeyword),
	Quote(Quote),
	// https://drafts.csswg.org/css-content-3/#leader-function
	// leader() = leader( <leader-type> )
	LeaderFunction(T![Function], LeaderType, Option<T![')']>),
	// Target(Target),
	// https://drafts.csswg.org/css-content-3/#string-function
	// string() = string( <custom-ident> , [ first | start | last | first-except ]? )
	StringFunction(T![Function], T![Ident], Option<T![,]>, Option<StringFunctionNamePresencece>, Option<T![')']>),
	// ContentFunction(ContentFunction),
	// Counter(Counter),
}

impl<'a> Parse<'a> for ContentListItem<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(string) = p.parse_if_peek::<T![String]>()? {
			return Ok(Self::String(string));
		}

		if let Some(image) = p.parse_if_peek::<Image>()? {
			return Ok(Self::Image(image));
		}

		if let Some(contents) = p.parse_if_peek::<ContentsKeyword>()? {
			return Ok(Self::Contents(contents));
		}

		if let Some(quote) = p.parse_if_peek::<Quote>()? {
			return Ok(Self::Quote(quote));
		}

		match p.parse::<ContentListFunctionNames>()? {
			ContentListFunctionNames::String(cursor) => {
				return Ok(Self::StringFunction(
					<T![Function]>::build(p, cursor),
					p.parse::<T![Ident]>()?,
					p.parse_if_peek::<T![,]>()?,
					p.parse_if_peek::<StringFunctionNamePresencece>()?,
					p.parse_if_peek::<T![')']>()?,
				));
			}
			ContentListFunctionNames::Leader(cursor) => {
				return Ok(Self::LeaderFunction(
					<T![Function]>::build(p, cursor),
					p.parse::<LeaderType>()?,
					p.parse_if_peek::<T![')']>()?,
				));
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<ContentList>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ContentList, "'some string'");
		assert_parse!(ContentList, "url(dot.gif)");
		assert_parse!(ContentList, "contents");
		assert_parse!(ContentList, "open-quote");
		assert_parse!(ContentList, "string(heading)");
		assert_parse!(ContentList, "string(heading,first)");
		assert_parse!(ContentList, "string(heading,first)");
		assert_parse!(ContentList, "leader('.')");
	}

	#[test]
	fn test_errors() {
		// assert_parse_error!(ContentList, "string()");
		// assert_parse_error!(ContentList, "foo()");
	}
}
