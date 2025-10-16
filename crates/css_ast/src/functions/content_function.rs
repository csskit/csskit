use super::prelude::*;

/// <https://drafts.csswg.org/css-content-3/#funcdef-content>
///
/// ```text,ignore
/// content() = content( [ text | before | after | first-letter | marker ]? )
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct ContentFunction {
	#[atom(CssAtomSet::Content)]
	pub name: T![Function],
	pub params: Option<ContentKeyword>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum ContentKeyword {
	#[atom(CssAtomSet::Text)]
	Text(T![Ident]),
	#[atom(CssAtomSet::Before)]
	Before(T![Ident]),
	#[atom(CssAtomSet::After)]
	After(T![Ident]),
	#[atom(CssAtomSet::FirstLetter)]
	FirstLetter(T![Ident]),
	#[atom(CssAtomSet::Marker)]
	Marker(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ContentFunction>(), 40);
		assert_eq!(std::mem::size_of::<ContentKeyword>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ContentFunction, "content(text)");
		assert_parse!(CssAtomSet::ATOMS, ContentFunction, "content(before)");
		assert_parse!(CssAtomSet::ATOMS, ContentFunction, "content()");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ContentFunction, "content(text before)");
	}
}
