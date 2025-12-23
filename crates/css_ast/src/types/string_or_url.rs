use super::prelude::*;

/// ```text
/// <string-or-url> = <url()> | string
/// <url()> = url( <string> <url-modifier>* ) | <url-token>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum StringOrUrl {
	Url(T![Url]),
	#[atom(CssAtomSet::Url)]
	UrlFunction(T![Function], T![String], T![')']),
	String(T![String]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StringOrUrl>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StringOrUrl, "url('foo')");
		assert_parse!(CssAtomSet::ATOMS, StringOrUrl, "url(\"foo\")");
		assert_parse!(CssAtomSet::ATOMS, StringOrUrl, "url(foo)");
		assert_parse!(CssAtomSet::ATOMS, StringOrUrl, "'foo'");
	}
}
