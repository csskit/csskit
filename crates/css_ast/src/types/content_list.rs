#![allow(warnings)]
use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	Build, Function, Parse, Parser, Peek, Result as ParserResult, T, diagnostics, function_set, keyword_set,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::{AttrFunction, ContentFunction, Counter, Image, LeaderFunction, Quote, StringFunction, Target};

/// <https://drafts.csswg.org/css-content-3/#content-values>
///
/// ```text,ignore
/// <content-list> = [ <string> | <image> | <attr()> | contents | <quote> | <leader()> | <target> | <string()> | <content()> | <counter> ]+
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ContentList<'a>(pub Vec<'a, ContentListItem<'a>>);

keyword_set!(pub struct ContentsKeyword "contents");

/// <https://drafts.csswg.org/css-content-3/#content-values>
///
/// ```text,ignore
/// <content-list> = [ <string> | <image> | <attr()> | contents | <quote> | <leader()> | <target> | <string()> | <content()> | <counter> ]+
/// ```
#[derive(Peek, Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ContentListItem<'a> {
	String(T![String]),
	Image(Image<'a>),
	AttrFunction(AttrFunction<'a>),
	Contents(ContentsKeyword),
	Quote(Quote),
	// https://drafts.csswg.org/css-content-3/#leader-function
	// leader() = leader( <leader-type> )
	LeaderFunction(LeaderFunction),
	Target(Target<'a>),
	// https://drafts.csswg.org/css-content-3/#string-function
	// string() = string( <custom-ident> , [ first | start | last | first-except ]? )
	StringFunction(StringFunction),
	// https://drafts.csswg.org/css-content-3/#funcdef-content
	// content() = content( [ text | before | after | first-letter | marker ]? )
	ContentFunction(ContentFunction),
	Counter(Counter<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ContentList>(), 32);
		assert_eq!(std::mem::size_of::<ContentListItem>(), 216);
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
		assert_parse!(ContentList, "leader('.')target-counter('foo',bar,decimal)");
		assert_parse!(ContentList, "content()");
		assert_parse!(ContentList, "content(marker)");
		assert_parse!(ContentList, "counter(foo,decimal)");
		assert_parse!(ContentList, "counters(foo,'bar',decimal)");
		assert_parse!(ContentList, "leader('.')'foo'counter(section,decimal)");
		assert_parse!(ContentList, "attr(foo)");
	}
}
