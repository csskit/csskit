use super::prelude::*;
use crate::RepeatLineColor;

/// <https://drafts.csswg.org/css-gaps-1/#typedef-line-color-or-repeat>
///
/// ```text,ignore
/// <line-color-or-repeat> = [ <color> | <repeat-line-color> ]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum LineColorOrRepeat<'a> {
	Color(crate::Value<'a, crate::Color<'a>>),
	RepeatFunction(RepeatLineColor<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineColorOrRepeat, "repeat(2,red)", LineColorOrRepeat::RepeatFunction(_));
		assert_parse!(CssAtomSet::ATOMS, LineColorOrRepeat, "red", LineColorOrRepeat::Color(_));
		assert_parse!(CssAtomSet::ATOMS, LineColorOrRepeat, "repeat(auto,red)", LineColorOrRepeat::RepeatFunction(_));
		assert_parse!(CssAtomSet::ATOMS, LineColorOrRepeat, "var(--x)", LineColorOrRepeat::Color(_));
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, LineColorOrRepeat, "repeat(none,red)");
		assert_peek_false!(CssAtomSet::ATOMS, LineColorOrRepeat, "florp");
	}
}
