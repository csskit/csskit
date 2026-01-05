use super::prelude::*;

/// <https://drafts.csswg.org/css-values-4/#url-value>
///
/// ```text
/// <url> = <url()> | <src()>
/// <url()> = url( <string> <url-modifier>* ) | <url-token>
/// <src()> = src( <string> <url-modifier>* )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum Url {
	Url(T![Url]),
	#[atom(CssAtomSet::Url)]
	UrlFunction(T![Function], T![String], T![')']),
	#[atom(CssAtomSet::Src)]
	SrcFunction(T![Function], T![String], T![')']),
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum UrlOrString {
	Url(Url),
	String(T![String]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Url>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Url, "url('foo')");
		assert_parse!(CssAtomSet::ATOMS, Url, "url(\"foo\")");
		assert_parse!(CssAtomSet::ATOMS, Url, "url(foo)");
		assert_parse!(CssAtomSet::ATOMS, Url, "src('foo')");
	}
}
