use super::prelude::*;
#[cfg(feature = "visitable")]
use crate::visit::{NodeId, QueryableNode};
use crate::{KeyframesName, KeyframesRuleBlock};

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, queryable(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule | Deprecated | NonStandard, used_at_rules = WebkitKeyframes, vendor_prefixes = WebKit, property_kinds = Name)]
pub struct WebkitKeyframesRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::_WebkitKeyframes)]
	pub name: T![AtKeyword],
	pub prelude: KeyframesName,
	#[metadata(delegate)]
	pub block: KeyframesRuleBlock<'a>,
}

#[cfg(feature = "visitable")]
impl<'a> QueryableNode for WebkitKeyframesRule<'a> {
	const NODE_ID: NodeId = NodeId::WebkitKeyframesRule;

	fn get_property(&self, kind: PropertyKind) -> Option<Cursor> {
		match kind {
			PropertyKind::Name => Some(self.prelude.into()),
			_ => None,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WebkitKeyframesRule>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WebkitKeyframesRule, "@-webkit-keyframes foo{}");
		assert_parse!(CssAtomSet::ATOMS, WebkitKeyframesRule, "@-webkit-keyframes\"include\"{}");
		assert_parse!(CssAtomSet::ATOMS, WebkitKeyframesRule, "@-webkit-keyframes spin{to{rotate:360deg}}");
	}
}
