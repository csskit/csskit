use super::prelude::*;
use crate::{KeyframesName, KeyframesRuleBlock};

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct WebkitKeyframesRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::_WebkitKeyframes)]
	pub name: T![AtKeyword],
	pub prelude: KeyframesName,
	pub block: KeyframesRuleBlock<'a>,
}

impl<'a> NodeWithMetadata<CssMetadata> for WebkitKeyframesRule<'a> {
	fn metadata(&self) -> CssMetadata {
		let mut meta = self.block.0.metadata();
		meta.used_at_rules |= AtRuleId::WebkitKeyframes;
		meta.vendor_prefixes |= VendorPrefixes::WebKit;
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
		assert_eq!(std::mem::size_of::<WebkitKeyframesRule>(), 120);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WebkitKeyframesRule, "@-webkit-keyframes foo{}");
		assert_parse!(CssAtomSet::ATOMS, WebkitKeyframesRule, "@-webkit-keyframes\"include\"{}");
		assert_parse!(CssAtomSet::ATOMS, WebkitKeyframesRule, "@-webkit-keyframes spin{to{rotate:360deg}}");
	}
}
