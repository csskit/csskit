use super::prelude::*;
use crate::{LineStyleOrRepeat, NonEmpty};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-line-style-list>
///
/// ```text,ignore
/// <line-style-list> = [ <line-style-or-repeat> ]+
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LineStyleList<'a>(pub NonEmpty<Vec<'a, LineStyleOrRepeat<'a>>>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineStyleList, "solid");
		assert_parse!(CssAtomSet::ATOMS, LineStyleList, "solid dashed dotted");
		assert_parse!(CssAtomSet::ATOMS, LineStyleList, "solid repeat(2,dashed dotted) none");
		assert_parse!(CssAtomSet::ATOMS, LineStyleList, "solid repeat(auto,dashed) none");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, LineStyleList, "");
		assert_peek_false!(CssAtomSet::ATOMS, LineStyleList, "florp");
		assert_parse_error!(CssAtomSet::ATOMS, LineStyleList, "repeat(auto,)");
	}
}
