use super::prelude::*;
use crate::{LineColorOrRepeat, NonEmpty};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-line-color-list>
///
/// ```text,ignore
/// <line-color-list> = [ <line-color-or-repeat> ]+
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LineColorList<'a>(pub NonEmpty<Vec<'a, LineColorOrRepeat<'a>>>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineColorList, "red");
		assert_parse!(CssAtomSet::ATOMS, LineColorList, "red green blue");
		assert_parse!(CssAtomSet::ATOMS, LineColorList, "red repeat(2,green blue) currentcolor");
		assert_parse!(CssAtomSet::ATOMS, LineColorList, "red repeat(auto,green) currentcolor");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, LineColorList, "");
		assert_peek_false!(CssAtomSet::ATOMS, LineColorList, "florp");
		assert_parse_error!(CssAtomSet::ATOMS, LineColorList, "repeat(auto,)");
	}
}
