use crate::{DocumentMatcherList, DocumentRuleBlock};
use css_parse::{AtRule, atkeyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

atkeyword_set!(pub struct AtMozDocumentKeyword "-moz-document");

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct MozDocumentRule<'a>(pub AtRule<AtMozDocumentKeyword, DocumentMatcherList<'a>, DocumentRuleBlock<'a>>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<MozDocumentRule>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MozDocumentRule, r#"@-moz-document url(http://www.w3.org){}"#);
		assert_parse!(MozDocumentRule, r#"@-moz-document url(http://www.w3.org),domain("mozilla.org"){}"#);
		assert_parse!(
			MozDocumentRule,
			r#"@-moz-document url(http://www.w3.org),url-prefix("http://www.w3.org/Style/"),domain("mozilla.org"){body{color:black}}"#
		);
	}
}
