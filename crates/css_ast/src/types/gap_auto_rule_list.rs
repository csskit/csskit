use crate::{GapRepeatRule, GapRuleList, GapRuleOrRepeat};

// https://drafts.csswg.org/css-gaps-1/#typedef-gap-auto-rule-list
// <gap-auto-rule-list> = <gap-rule-or-repeat>#? , <gap-auto-repeat-rule> , <gap-rule-or-repeat>#?
// We intentionally flatten this into <gap-rule-list> semantics so higher layers
// can decide whether stricter auto-repeat placement constraints should apply.
pub type GapAutoRuleList<'a> = GapRuleList<'a>;
pub type GapAutoRuleListItem<'a> = GapRuleOrRepeat<'a>;
pub type GapAutoRepeatRule<'a> = GapRepeatRule<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<GapAutoRuleList>(), std::mem::size_of::<GapRuleList>());
		assert_eq!(std::mem::size_of::<GapAutoRuleListItem>(), std::mem::size_of::<GapRuleOrRepeat>());
		assert_eq!(std::mem::size_of::<GapAutoRepeatRule>(), std::mem::size_of::<GapRepeatRule>());
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GapAutoRuleList, "repeat(auto, 1px solid red)");
		assert_parse!(CssAtomSet::ATOMS, GapAutoRuleList, "1px solid red, repeat(auto, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapAutoRuleList, "repeat(auto, 1px solid red), 2px dashed green");
		assert_parse!(
			CssAtomSet::ATOMS,
			GapAutoRuleList,
			"1px solid red, repeat(auto, 2px dashed green), 3px dotted blue"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			GapAutoRuleList,
			"repeat(auto, 1px solid red), repeat(auto, 2px dashed green)"
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, GapAutoRuleList, "repeat(none, 1px solid red)");
		assert_parse_error!(CssAtomSet::ATOMS, GapAutoRuleList, "repeat(0, 1px solid red)");
		assert_parse_error!(CssAtomSet::ATOMS, GapAutoRuleList, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, GapAutoRuleList, "1px solid red,");
	}
}
