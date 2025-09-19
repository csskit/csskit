use super::prelude::*;
use crate::{KeyframesName, KeyframesRuleBlock};

atkeyword_set!(pub struct AtWebkitKeyframesKeyword "-webkit-keyframes");

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct WebkitKeyframesRule<'a>(pub AtRule<AtWebkitKeyframesKeyword, KeyframesName, KeyframesRuleBlock<'a>>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WebkitKeyframesRule>(), 112);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WebkitKeyframesRule, "@-webkit-keyframes foo{}");
		assert_parse!(WebkitKeyframesRule, "@-webkit-keyframes\"include\"{}");
		assert_parse!(WebkitKeyframesRule, "@-webkit-keyframes spin{to{rotate:360deg}}");
	}
}
