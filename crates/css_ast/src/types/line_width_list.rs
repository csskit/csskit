use super::prelude::*;
use crate::{LineWidthOrRepeat, NonEmpty};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-line-width-list>
///
/// ```text,ignore
/// <line-width-list> = [ <line-width-or-repeat> ]+
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LineWidthList<'a>(pub NonEmpty<Vec<'a, LineWidthOrRepeat<'a>>>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineWidthList, "thin");
		assert_parse!(CssAtomSet::ATOMS, LineWidthList, "thin 2px thick");
		assert_parse!(CssAtomSet::ATOMS, LineWidthList, "thin repeat(2,2px 3px) medium");
		assert_parse!(CssAtomSet::ATOMS, LineWidthList, "thin repeat(auto,2px) medium");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, LineWidthList, "");
		assert_peek_false!(CssAtomSet::ATOMS, LineWidthList, "florp");
		assert_parse_error!(CssAtomSet::ATOMS, LineWidthList, "repeat(auto,)");
	}
}
