use super::prelude::*;
use crate::{Color, FillOrigin, Paint, PositionAndSize, RepeatStyle};

/// Represents a single layer of the `stroke` shorthand.
///
/// The `stroke` property is a shorthand for `stroke-image`, `stroke-position`,
/// `stroke-size`, `stroke-repeat`, `stroke-origin`, and `stroke-color`,
/// modelled on `background`'s `<bg-layer>#` grammar the same way `fill` is
/// (see `FillLayer`). The `color` field is only meaningful on the final
/// layer, but we parse it permissively on every layer, matching `BgLayer`'s
/// approach.
///
/// <https://drafts.csswg.org/fill-stroke-3/#stroke>
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[parse(all_must_occur)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct StrokeLayer<'a> {
	pub image: Option<Paint<'a>>,
	pub position: Option<PositionAndSize<'a>>,
	pub repeat: Option<RepeatStyle>,
	pub origin: Option<FillOrigin>,
	pub color: Option<Color<'a>>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StrokeLayer>(), 272);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StrokeLayer, "none");
		assert_parse!(CssAtomSet::ATOMS, StrokeLayer, "url(foo.svg)");
		assert_parse!(CssAtomSet::ATOMS, StrokeLayer, "currentcolor");
		assert_parse!(CssAtomSet::ATOMS, StrokeLayer, "center");
		assert_parse!(CssAtomSet::ATOMS, StrokeLayer, "center / cover");
		assert_parse!(CssAtomSet::ATOMS, StrokeLayer, "repeat-x");
		assert_parse!(CssAtomSet::ATOMS, StrokeLayer, "stroke-box");
		assert_parse!(CssAtomSet::ATOMS, StrokeLayer, "url(foo.svg) center no-repeat stroke-box red");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, StrokeLayer, "");
	}
}
