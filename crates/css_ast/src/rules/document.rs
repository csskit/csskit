use super::prelude::*;

// https://www.w3.org/TR/2012/WD-css3-conditional-20120911/#at-document
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct DocumentRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Document)]
	pub name: T![AtKeyword],
	pub prelude: DocumentMatcherList<'a>,
	pub block: DocumentRuleBlock<'a>,
}

impl<'a> NodeWithMetadata<CssMetadata> for DocumentRule<'a> {
	fn metadata(&self) -> CssMetadata {
		let mut meta = self.block.0.metadata();
		meta.used_at_rules |= AtRuleId::Document;
		meta.node_kinds |= NodeKinds::AtRule;
		meta
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct DocumentMatcherList<'a>(pub CommaSeparated<'a, DocumentMatcher>);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum DocumentMatcher {
	Url(T![Url]),
	#[atom(CssAtomSet::Url)]
	UrlFunction(T![Function], T![String], T![')']),
	#[atom(CssAtomSet::UrlPrefix)]
	UrlPrefix(T![Function], T![String], T![')']),
	#[atom(CssAtomSet::Domain)]
	Domain(T![Function], T![String], T![')']),
	#[atom(CssAtomSet::MediaDocument)]
	MediaDocument(T![Function], T![String], T![')']),
	#[atom(CssAtomSet::Regexp)]
	Regexp(T![Function], T![String], T![')']),
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct DocumentRuleBlock<'a>(pub RuleList<'a, Rule<'a>, CssMetadata>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DocumentRule>(), 136);
		assert_eq!(std::mem::size_of::<DocumentMatcher>(), 40);
		assert_eq!(std::mem::size_of::<DocumentRuleBlock>(), 88);
	}

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
