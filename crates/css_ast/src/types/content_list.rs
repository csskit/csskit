use super::prelude::*;
use crate::{AttrFunction, ContentFunction, Counter, Image, LeaderFunction, Quote, StringFunction, Target};

/// <https://drafts.csswg.org/css-content-3/#content-values>
///
/// ```text,ignore
/// <content-list> = [ <string> | <image> | <attr()> | contents | <quote> | <leader()> | <target> | <string()> | <content()> | <counter> ]+
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct ContentList<'a>(pub Vec<'a, ContentListItem<'a>>);

/// <https://drafts.csswg.org/css-content-3/#content-values>
///
/// ```text,ignore
/// <content-list> = [ <string> | <image> | <attr()> | contents | <quote> | <leader()> | <target> | <string()> | <content()> | <counter> ]+
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum ContentListItem<'a> {
	String(T![String]),
	Image(Image<'a>),
	AttrFunction(AttrFunction<'a>),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Contents)]
	Contents(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
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
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ContentList>(), 32);
		assert_eq!(std::mem::size_of::<ContentListItem>(), 208);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ContentList, "'some string'");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "url(dot.gif)");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "contents");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "open-quote");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "string(heading)");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "string(heading,first)");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "string(heading,first)");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "leader('.')");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "leader('.')target-counter('foo',bar,decimal)");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "content()");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "content(marker)");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "counter(foo,decimal)");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "counters(foo,'bar',decimal)");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "leader('.')'foo'counter(section,decimal)");
		assert_parse!(CssAtomSet::ATOMS, ContentList, "attr(foo)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("'some string'", ContentList, ContentListItem);
		assert_visits!("url(dot.gif)", ContentList, ContentListItem, Image, Url);
		assert_visits!("counter(foo,decimal)", ContentList, ContentListItem, CounterFunction);
		assert_visits!("'foo' url(bar.gif)", ContentList, ContentListItem, ContentListItem, Image, Url);
		assert_visits!("string(heading)", ContentList, ContentListItem, StringFunction);
		assert_visits!("attr(foo)", ContentList, ContentListItem, AttrFunction);
	}
}
