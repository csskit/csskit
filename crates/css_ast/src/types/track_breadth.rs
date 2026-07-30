use super::prelude::*;

/// <https://drafts.csswg.org/css-grid-2/#typedef-track-breadth>
///
/// ```text,ignore
/// <track-breadth> = <length-percentage [0,∞]> | <flex [0,∞]> | min-content | max-content | auto
/// ```
#[syntax(" <length-percentage [0,∞]> | <flex [0,∞]> | min-content | max-content | auto ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TrackBreadth<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TrackBreadth, "10px");
		assert_parse!(CssAtomSet::ATOMS, TrackBreadth, "50%");
		assert_parse!(CssAtomSet::ATOMS, TrackBreadth, "1fr");
		assert_parse!(CssAtomSet::ATOMS, TrackBreadth, "min-content");
		assert_parse!(CssAtomSet::ATOMS, TrackBreadth, "max-content");
		assert_parse!(CssAtomSet::ATOMS, TrackBreadth, "auto");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TrackBreadth, "-10px");
		assert_peek_false!(CssAtomSet::ATOMS, TrackBreadth, "none");
	}
}
