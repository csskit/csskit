use super::prelude::*;

/// <https://drafts.csswg.org/css-grid-2/#typedef-track-size>
///
/// ```text,ignore
/// <track-size> = <track-breadth> | minmax( <inflexible-breadth> , <track-breadth> ) | fit-content( <length-percentage [0,∞]> )
/// ```
#[syntax(
	" <track-breadth> | minmax( <inflexible-breadth> , <track-breadth> ) | fit-content( <length-percentage [0,∞]> ) "
)]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TrackSize<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TrackSize, "10px");
		assert_parse!(CssAtomSet::ATOMS, TrackSize, "auto");
		assert_parse!(CssAtomSet::ATOMS, TrackSize, "minmax(100px,1fr)");
		assert_parse!(CssAtomSet::ATOMS, TrackSize, "fit-content(10%)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TrackSize, "minmax(1fr,10px)");
		assert_peek_false!(CssAtomSet::ATOMS, TrackSize, "none");
	}
}
