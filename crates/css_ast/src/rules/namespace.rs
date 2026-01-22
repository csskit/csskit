use super::prelude::*;
use crate::StringOrUrl;

/// <https://drafts.csswg.org/css-namespaces/#at-ruledef-namespace>
///
/// ```text,ignore
/// <namespace-prefix>? [ <string> | <url> ] ;
/// ```
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.page"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Namespace)]
pub struct NamespaceRule {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Namespace)]
	pub name: T![AtKeyword],
	pub prefix: Option<T![Ident]>,
	pub resource: StringOrUrl,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub semicolon: Option<T![;]>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NamespaceRule>(), 84);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, NamespaceRule, "@namespace html 'http://www.w3.org/1999/xhtml';");
		assert_parse!(
			CssAtomSet::ATOMS,
			NamespaceRule,
			"@namespace xul url(\"http://www.mozilla.org/keymaster/gatekeeper/there.is.only.xul\");"
		);
	}
}
