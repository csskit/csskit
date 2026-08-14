use super::prelude::*;
use crate::{Percentage, Value};

/// `[ normal | from-font | <percentage> ]`, as used by the `@font-face` sub/superscript override
/// descriptors.
///
/// ```text,ignore
/// normal | from-font | <percentage>
/// ```
///
/// <https://drafts.csswg.org/css-fonts-5/#descdef-font-face-superscript-position-override>
// minimal: the size-override descriptors restrict the percentage to [0,∞], which this shared type
// does not enforce; split into a non-negative variant if that validation is needed.
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum NormalFromFontPercentage<'a> {
	#[atom(CssAtomSet::Normal)]
	Normal(T![Ident]),
	#[atom(CssAtomSet::FromFont)]
	FromFont(T![Ident]),
	Percentage(Value<'a, Percentage>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, NormalFromFontPercentage, "normal");
		assert_parse!(CssAtomSet::ATOMS, NormalFromFontPercentage, "from-font");
		assert_parse!(CssAtomSet::ATOMS, NormalFromFontPercentage, "20%");
		assert_parse!(CssAtomSet::ATOMS, NormalFromFontPercentage, "-20%");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, NormalFromFontPercentage, "20px");
	}
}
