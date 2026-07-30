use super::prelude::*;
use crate::RepeatLineWidth;

/// <https://drafts.csswg.org/css-gaps-1/#typedef-line-width-or-repeat>
///
/// ```text,ignore
/// <line-width-or-repeat> = [ <line-width> | <repeat-line-width> ]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum LineWidthOrRepeat<'a> {
	LineWidth(crate::Value<'a, crate::LineWidth<'a>>),
	RepeatFunction(RepeatLineWidth<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use crate::LineWidth;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineWidthOrRepeat, "repeat(2,12px)", LineWidthOrRepeat::RepeatFunction(_));
		assert_parse!(
			CssAtomSet::ATOMS,
			LineWidthOrRepeat,
			"thin",
			LineWidthOrRepeat::LineWidth(crate::Value::Literal(LineWidth::Thin(_)))
		);
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, LineWidthOrRepeat, "repeat(none, 12px)");
	}
}
