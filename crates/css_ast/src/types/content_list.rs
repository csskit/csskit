#![allow(warnings)]
use bumpalo::collections::Vec;
use css_parse::{Parse, Parser, Result as ParserResult, T, keyword_set};
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
	StringFunction(T![Function]),
	ContentFunction(ContentFunction),
	Counter(Counter),
}

impl<'a> Parse<'a> for ContentList<'a> {
	fn parse(_p: &mut Parser<'a>) -> ParserResult<Self> {
		todo!()
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
		// assert_parse!(ContentList, "contents");
	}

	#[test]
	fn test_errors() {
		// assert_parse_error!(ContentList, "foo");
	}
}
