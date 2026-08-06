use super::prelude::*;
use crate::{CompositingOperator, GeometryBox, MaskReference, MaskingMode, PositionAndSize, RepeatStyle};

/// <https://drafts.csswg.org/css-masking-1/#typedef-mask-layer>
///
/// ```text,ignore
/// <mask-layer> = <mask-reference> || <position> [ / <bg-size> ]? || <repeat-style> ||
///   <geometry-box> || [ <geometry-box> | no-clip ] || <compositing-operator> ||
///   <masking-mode>
/// ```
///
/// The first `<geometry-box>` sets `mask-origin`, the second (or `no-clip`) sets
/// `mask-clip`.
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[parse(all_must_occur)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MaskLayer<'a> {
	pub image: Option<MaskReference<'a>>,
	pub position: Option<PositionAndSize<'a>>,
	pub repeat: Option<RepeatStyle>,
	pub origin: Option<GeometryBox>,
	pub clip: Option<MaskLayerClip>,
	pub composite: Option<CompositingOperator>,
	pub mode: Option<MaskingMode>,
}

/// The `[ <geometry-box> | no-clip ]` component of `<mask-layer>`, setting
/// `mask-clip`.
///
/// ```text,ignore
/// [ <geometry-box> | no-clip ]
/// ```
///
/// <https://drafts.csswg.org/css-masking-1/#typedef-mask-layer>
#[node]
#[derive(Parse, Peek, ToSpan, SemanticEq, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MaskLayerClip {
	GeometryBox(GeometryBox),
	#[atom(CssAtomSet::NoClip)]
	NoClip(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "none");
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "url(mask.svg)");
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "center");
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "center / cover");
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "no-repeat");
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "fill-box");
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "no-clip");
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "border-box no-clip");
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "content-box border-box");
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "add");
		assert_parse!(CssAtomSet::ATOMS, MaskLayer, "luminance");
		assert_parse!(
			CssAtomSet::ATOMS,
			MaskLayer,
			"url(mask.svg) center / cover no-repeat view-box no-clip add alpha"
		);
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MaskLayer, "");
	}
}
