use super::prelude::*;
use crate::{KeyframesName, KeyframesRuleBlock};

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct WebkitKeyframesRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::_WebkitKeyframes)]
	pub name: T![AtKeyword],
	pub prelude: KeyframesName,
	pub block: KeyframesRuleBlock<'a>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WebkitKeyframesRule>(), 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WebkitKeyframesRule, "@-webkit-keyframes foo{}");
		assert_parse!(CssAtomSet::ATOMS, WebkitKeyframesRule, "@-webkit-keyframes\"include\"{}");
		assert_parse!(CssAtomSet::ATOMS, WebkitKeyframesRule, "@-webkit-keyframes spin{to{rotate:360deg}}");
	}
}
