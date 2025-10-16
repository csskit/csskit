use super::prelude::*;
use crate::{DocumentMatcherList, DocumentRuleBlock};

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
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
	fn size_test() {
		assert_eq!(std::mem::size_of::<MozDocumentRule>(), 112);
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
