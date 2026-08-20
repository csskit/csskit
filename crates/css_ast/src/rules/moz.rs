use super::prelude::*;
use crate::{DocumentMatcherList, DocumentRuleBlock};

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule | Deprecated | NonStandard, used_at_rules = MozDocument, vendor_prefixes = Moz)]
pub struct MozDocumentRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::_MozDocument)]
	pub name: T![AtKeyword],
	pub prelude: DocumentMatcherList<'a>,
	pub block: DocumentRuleBlock<'a>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, MozDocumentRule, r#"@-moz-document url(http://www.w3.org){}"#);
		assert_parse!(
			CssAtomSet::ATOMS,
			MozDocumentRule,
			r#"@-moz-document url(http://www.w3.org),domain("mozilla.org"){}"#
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			MozDocumentRule,
			r#"@-moz-document url(http://www.w3.org),url-prefix("http://www.w3.org/Style/"),domain("mozilla.org"){body{color:black}}"#
		);
	}
}
