use super::prelude::*;
use crate::{AttrFunction, Counter};

/// One item in the alt-text portion of a `content` declaration.
///
/// ```text,ignore
/// <string> | <counter> | <attr()>
/// ```
///
/// Used in: `content: <content-list> / <content-alt-item>+`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ContentAltItem<'a> {
	String(T![String]),
	Counter(Counter<'a>),
	AttrFunction(AttrFunction<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ContentAltItem>(), 160);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ContentAltItem, "\"hello\"");
		assert_parse!(CssAtomSet::ATOMS, ContentAltItem, "counter(section)");
		assert_parse!(CssAtomSet::ATOMS, ContentAltItem, "attr(foo)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ContentAltItem, "open-quote");
	}
}
