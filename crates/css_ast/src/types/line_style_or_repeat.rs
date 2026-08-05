use super::prelude::*;
use crate::RepeatLineStyle;

/// <https://drafts.csswg.org/css-gaps-1/#typedef-line-style-or-repeat>
///
/// ```text,ignore
/// <line-style-or-repeat> = [ <line-style> | <repeat-line-style> ]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum LineStyleOrRepeat<'a> {
	LineStyle(crate::Value<'a, crate::LineStyle>),
	RepeatFunction(RepeatLineStyle<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use crate::LineStyle;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineStyleOrRepeat, "repeat(2,solid)", LineStyleOrRepeat::RepeatFunction(_));
		assert_parse!(CssAtomSet::ATOMS, LineStyleOrRepeat, "repeat(auto,solid)", LineStyleOrRepeat::RepeatFunction(_));
		assert_parse!(
			CssAtomSet::ATOMS,
			LineStyleOrRepeat,
			"dashed",
			LineStyleOrRepeat::LineStyle(crate::Value::Literal(LineStyle::Dashed(_)))
		);
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, LineStyleOrRepeat, "repeat(none,solid)");
		assert_peek_false!(CssAtomSet::ATOMS, LineStyleOrRepeat, "florp");
	}
}
