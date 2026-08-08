use super::prelude::*;
/// <https://drafts.csswg.org/css-inline-3/#typedef-text-edge>
///
/// ```text,ignore
/// <text-edge> = [ text | ideographic | ideographic-ink ] | [ text | ideographic | ideographic-ink | cap | ex ] [ text | ideographic | ideographic-ink | alphabetic ]
/// ```
#[syntax(
	" [ text | ideographic | ideographic-ink | cap | ex ] [ text | ideographic | ideographic-ink | alphabetic ] | [ text | ideographic | ideographic-ink ] "
)]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TextEdge<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TextEdge, "text");
		assert_parse!(CssAtomSet::ATOMS, TextEdge, "ideographic");
		assert_parse!(CssAtomSet::ATOMS, TextEdge, "ideographic-ink");
		assert_parse!(CssAtomSet::ATOMS, TextEdge, "cap alphabetic");
		assert_parse!(CssAtomSet::ATOMS, TextEdge, "ex alphabetic");
		assert_parse!(CssAtomSet::ATOMS, TextEdge, "text alphabetic");
		assert_parse!(CssAtomSet::ATOMS, TextEdge, "cap text");
		assert_parse!(CssAtomSet::ATOMS, TextEdge, "ideographic ideographic-ink");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, TextEdge, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextEdge, "cap");
		assert_parse_error!(CssAtomSet::ATOMS, TextEdge, "ex");
		assert_peek_false!(CssAtomSet::ATOMS, TextEdge, "alphabetic");
		assert_parse_error!(CssAtomSet::ATOMS, TextEdge, "text cap");
		assert_peek_false!(CssAtomSet::ATOMS, TextEdge, "leading");
	}
}
