#![allow(unused)]
use super::prelude::*;

use crate::{AutoOr, Color, LineStyle, LineWidth, PositiveNonZeroInt};
use css_parse::{CommaSeparated, Optionals3};

// https://drafts.csswg.org/css-gaps-1/#typedef-gap-rule-list
// <gap-rule-list> = <gap-rule-or-repeat>#
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GapRuleList<'a>(pub CommaSeparated<'a, GapRuleOrRepeat<'a>>);

// <gap-rule-or-repeat> = <gap-rule> | <gap-repeat-rule>
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum GapRuleOrRepeat<'a> {
	GapRule(GapRule<'a>),
	GapRepeatRule(GapRepeatRule<'a>),
}

// <gap-repeat-rule> = repeat( <integer [1,∞]> , <gap-rule># )
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GapRepeatRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Repeat)]
	pub name: T![Function],
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub count: AutoOr<PositiveNonZeroInt>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub comma: T![,],
	pub rules: CommaSeparated<'a, GapRule<'a>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: T![')'],
}

// <gap-rule> = <line-width> || <line-style> || <color>
pub type GapRule<'a> = Optionals3<LineWidth, LineStyle, Color<'a>>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<GapRuleList>(), 32);
		assert_eq!(std::mem::size_of::<GapRuleOrRepeat>(), 176);
		assert_eq!(std::mem::size_of::<GapRepeatRule>(), 88);
		assert_eq!(std::mem::size_of::<GapRule>(), 176);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red, 2px dashed green, 3px dotted blue");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red, repeat(2, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "1px solid red, repeat(auto, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "repeat(2, 1px solid red, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "repeat(auto, 1px solid red, 2px dashed green)");
		assert_parse!(CssAtomSet::ATOMS, GapRuleList, "solid 1px red");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(none, 1px solid red)");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(0, 1px solid red)");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(2,)");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "repeat(2, repeat(2, 1px solid red))");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "1px solid red,");
		assert_parse_error!(CssAtomSet::ATOMS, GapRuleList, "1px solid red solid");
	}
}
