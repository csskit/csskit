use super::prelude::*;
use crate::{DocumentMatcherList, DocumentRuleBlock};

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct MozDocumentRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::_MozDocument)]
	pub name: T![AtKeyword],
	pub prelude: DocumentMatcherList<'a>,
	pub block: DocumentRuleBlock<'a>,
}

impl<'a> NodeWithMetadata<CssMetadata> for MozDocumentRule<'a> {
	fn metadata(&self) -> CssMetadata {
		let mut meta = self.block.0.metadata();
		meta.used_at_rules |= AtRuleId::MozDocument;
		meta.vendor_prefixes |= VendorPrefixes::Moz;
		meta.node_kinds |= NodeKinds::AtRule;
		meta
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<MozDocumentRule>(), 136);
	}

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
