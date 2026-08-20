use super::prelude::*;
use css_parse::Box;

use crate::{AttrFunction, CounterFunction, CountersFunction};

/// <https://drafts.csswg.org/css-gcpm-4/#typedef-content-level>
///
/// ```text,ignore
/// <content-level> = element | content | text | attr(<custom-ident>) | <counter()> | <counters()>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ContentLevel<'a> {
	#[atom(CssAtomSet::Element)]
	Element(T![Ident]),
	#[atom(CssAtomSet::Content)]
	Content(T![Ident]),
	#[atom(CssAtomSet::Text)]
	Text(T![Ident]),
	Attr(Box<'a, AttrFunction<'a>>),
	Counter(Box<'a, CounterFunction<'a>>),
	Counters(Box<'a, CountersFunction<'a>>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ContentLevel, "element");
		assert_parse!(CssAtomSet::ATOMS, ContentLevel, "content");
		assert_parse!(CssAtomSet::ATOMS, ContentLevel, "text");
		assert_parse!(CssAtomSet::ATOMS, ContentLevel, "attr(foo)");
		assert_parse!(CssAtomSet::ATOMS, ContentLevel, "counter(section)");
		assert_parse!(CssAtomSet::ATOMS, ContentLevel, "counters(section,'.')");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ContentLevel, "");
		assert_peek_false!(CssAtomSet::ATOMS, ContentLevel, "foo");
	}
}
