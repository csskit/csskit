use super::prelude::*;
use crate::{Color, FillOrigin, Paint, PositionAndSize, RepeatStyle};

/// Represents a single layer of the `fill` shorthand.
///
/// The `fill` property is a shorthand for `fill-image`, `fill-position`,
/// `fill-size`, `fill-repeat`, `fill-origin`, and `fill-color`, modelled on
/// `background`'s `<bg-layer>#` grammar (fill has no `<bg-clip>`/`<attachment>`
/// equivalent). The `color` field is only meaningful on the final layer, but
/// we parse it permissively on every layer, matching `BgLayer`'s approach.
///
/// <https://drafts.csswg.org/fill-stroke-3/#fill>
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[parse(all_must_occur)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FillLayer<'a> {
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
		assert_eq!(std::mem::size_of::<FillLayer>(), 256);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FillLayer, "none");
		assert_parse!(CssAtomSet::ATOMS, FillLayer, "url(foo.svg)");
		assert_parse!(CssAtomSet::ATOMS, FillLayer, "currentcolor");
		assert_parse!(CssAtomSet::ATOMS, FillLayer, "center");
		assert_parse!(CssAtomSet::ATOMS, FillLayer, "center / cover");
		assert_parse!(CssAtomSet::ATOMS, FillLayer, "repeat-x");
		assert_parse!(CssAtomSet::ATOMS, FillLayer, "fill-box");
		assert_parse!(CssAtomSet::ATOMS, FillLayer, "url(foo.svg) center no-repeat fill-box red");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FillLayer, "");
	}
}
