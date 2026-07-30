use super::prelude::*;

/// <https://www.w3.org/TR/2012/WD-css3-conditional-20120911/#at-document>
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule | Deprecated, used_at_rules = Document)]
pub struct DocumentRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Document)]
	pub name: T![AtKeyword],
	pub prelude: DocumentMatcherList<'a>,
	#[metadata(delegate)]
	pub block: DocumentRuleBlock<'a>,
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct DocumentMatcherList<'a>(pub CommaSeparated<'a, DocumentMatcher>);

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum DocumentMatcher {
	Url(T![Url]),
	#[atom(CssAtomSet::Url)]
	UrlFunction(T![Function], T![String], #[semantic_eq(skip)] T![')']),
	#[atom(CssAtomSet::UrlPrefix)]
	UrlPrefix(T![Function], T![String], #[semantic_eq(skip)] T![')']),
	#[atom(CssAtomSet::Domain)]
	Domain(T![Function], T![String], #[semantic_eq(skip)] T![')']),
	#[atom(CssAtomSet::MediaDocument)]
	MediaDocument(T![Function], T![String], #[semantic_eq(skip)] T![')']),
	#[atom(CssAtomSet::Regexp)]
	Regexp(T![Function], T![String], #[semantic_eq(skip)] T![')']),
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct DocumentRuleBlock<'a>(#[metadata(delegate)] pub RuleList<'a, Rule<'a>, CssMetadata>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DocumentRule, r#"@document url("http://www.w3.org"){}"#);
		assert_parse!(CssAtomSet::ATOMS, DocumentRule, r#"@document domain("mozilla.org"){}"#);
		assert_parse!(CssAtomSet::ATOMS, DocumentRule, r#"@document url-prefix("http://www.w3.org/Style/"){}"#);
		assert_parse!(CssAtomSet::ATOMS, DocumentRule, r#"@document media-document("video"){}"#);
		assert_parse!(CssAtomSet::ATOMS, DocumentRule, r#"@document regexp("https:.*"){}"#);
		assert_parse!(
			CssAtomSet::ATOMS,
			DocumentRule,
			r#"@document url(http://www.w3.org),url-prefix("http://www.w3.org/Style/"),domain("mozilla.org"){}"#
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			DocumentRule,
			r#"@document url(http://www.w3.org),url-prefix("http://www.w3.org/Style/"),domain("mozilla.org"){body{color:black}}"#
		);
	}
}
