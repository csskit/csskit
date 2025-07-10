#![allow(warnings)]
use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{Build, Parse, Parser, Peek, Result as ParserResult, T, diagnostics, function_set, keyword_set};
use csskit_derives::{IntoSpan, Parse, Peek, ToCursors};

use crate::types::{Image, Quote};

// https://drafts.csswg.org/css-content-3/#leader-function
type LeaderFunction = crate::Todo;
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
#[derive(IntoSpan, ToCursors, Peek, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ContentList<'a>(pub Vec<'a, ContentListItem<'a>>);

keyword_set!(ContentsKeyword, "contents");
function_set!(ContentListFunctionKeyword { String: "string" });

#[derive(IntoSpan, ToCursors, Parse, Peek, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ContentListItem<'a> {
	String(T![String]),
	Image(Image<'a>),
	AttrFunction(AttrFunction),
	Contents(ContentsKeyword),
	Quote(Quote),
	LeaderFunction(LeaderFunction),
	Target(Target),
	StringFunction(StringFunction),
	ContentFunction(ContentFunction),
	Counter(Counter),
}

impl<'a> Parse<'a> for ContentList<'a> {
	fn parse(_p: &mut Parser<'a>) -> ParserResult<Self> {
		todo!()
	}
}

/// string()

keyword_set!(NamePresencece { First: "first", Start: "start", Last: "last", FirstExcept: "first-except" });

// https://drafts.csswg.org/css-content-3/#string-function
// string() = string( <custom-ident> , [ first | start | last | first-except ]? )
#[derive(IntoSpan, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct StringFunction(
	pub T![Function],
	pub T![Ident],
	pub Option<T![,]>,
	pub Option<NamePresencece>,
	pub Option<T![')']>,
);

impl<'a> Peek<'a> for StringFunction {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		// TODO? is this string eq needed?
		ContentListFunctionKeyword::peek(p, c) && p.eq_ignore_ascii_case(c, "string")
	}
}

impl<'a> Parse<'a> for StringFunction {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let func = p.parse::<ContentListFunctionKeyword>()?;
		if let ContentListFunctionKeyword::String(cursor) = func {
			let ident = p.parse::<T![Ident]>()?;
			let comma = p.parse_if_peek::<T![,]>()?;
			let presence = p.parse_if_peek::<NamePresencece>()?;
			let close = p.parse_if_peek::<T![')']>()?;
			return Ok(StringFunction(<T![Function]>::build(p, cursor), ident, comma, presence, close));
		};

		let c: Cursor = func.into();
		Err(diagnostics::UnexpectedFunction(func.into(), c.into()))?
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
		assert_parse!(StringFunction, "string(heading)");
		assert_parse!(StringFunction, "string(heading,first)");
		assert_parse!(StringFunction, "string(heading,first)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(StringFunction, "string()");
		assert_parse_error!(StringFunction, "foo()");
	}
}
