#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		// FillStyleValue: <fill-layer>#
		assert_parse!(CssAtomSet::ATOMS, FillStyleValue, "currentcolor");
		assert_parse!(CssAtomSet::ATOMS, FillStyleValue, "url(sprite.svg#icon)");
		assert_parse!(CssAtomSet::ATOMS, FillStyleValue, "url(a.svg), url(b.svg) red");
		assert_parse!(CssAtomSet::ATOMS, FillStyleValue, "url(a.svg) center / cover no-repeat fill-box red");

		// StrokeStyleValue: <stroke-layer>#
		assert_parse!(CssAtomSet::ATOMS, StrokeStyleValue, "blue");
		assert_parse!(CssAtomSet::ATOMS, StrokeStyleValue, "url(x.svg) center / cover no-repeat stroke-box blue");

		// FillBreakStyleValue: bounding-box | slice | clone
		assert_parse!(CssAtomSet::ATOMS, FillBreakStyleValue, "bounding-box");
		assert_parse!(CssAtomSet::ATOMS, FillBreakStyleValue, "slice");
		assert_parse!(CssAtomSet::ATOMS, FillBreakStyleValue, "clone");

		// FillColorStyleValue: <color>
		assert_parse!(CssAtomSet::ATOMS, FillColorStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, FillColorStyleValue, "currentcolor");
		assert_parse!(CssAtomSet::ATOMS, FillColorStyleValue, "transparent");

		// FillOpacityStyleValue: <'opacity'>
		assert_parse!(CssAtomSet::ATOMS, FillOpacityStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, FillOpacityStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, FillOpacityStyleValue, "0.5");

		// FillOriginStyleValue: match-parent | fill-box | stroke-box | content-box | padding-box | border-box
		assert_parse!(CssAtomSet::ATOMS, FillOriginStyleValue, "match-parent");
		assert_parse!(CssAtomSet::ATOMS, FillOriginStyleValue, "fill-box");
		assert_parse!(CssAtomSet::ATOMS, FillOriginStyleValue, "stroke-box");
		assert_parse!(CssAtomSet::ATOMS, FillOriginStyleValue, "content-box");
		assert_parse!(CssAtomSet::ATOMS, FillOriginStyleValue, "padding-box");
		assert_parse!(CssAtomSet::ATOMS, FillOriginStyleValue, "border-box");

		// FillPositionStyleValue: <position>#
		assert_parse!(CssAtomSet::ATOMS, FillPositionStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, FillPositionStyleValue, "top left");
		assert_parse!(CssAtomSet::ATOMS, FillPositionStyleValue, "center,top left");

		// FillRuleStyleValue: nonzero | evenodd
		assert_parse!(CssAtomSet::ATOMS, FillRuleStyleValue, "nonzero");
		assert_parse!(CssAtomSet::ATOMS, FillRuleStyleValue, "evenodd");

		// StrokeAlignStyleValue: center | inset | outset
		assert_parse!(CssAtomSet::ATOMS, StrokeAlignStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, StrokeAlignStyleValue, "inset");
		assert_parse!(CssAtomSet::ATOMS, StrokeAlignStyleValue, "outset");

		// StrokeBreakStyleValue: bounding-box | slice | clone
		assert_parse!(CssAtomSet::ATOMS, StrokeBreakStyleValue, "bounding-box");
		assert_parse!(CssAtomSet::ATOMS, StrokeBreakStyleValue, "slice");
		assert_parse!(CssAtomSet::ATOMS, StrokeBreakStyleValue, "clone");

		// StrokeColorStyleValue: <color>#
		assert_parse!(CssAtomSet::ATOMS, StrokeColorStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, StrokeColorStyleValue, "blue");
		assert_parse!(CssAtomSet::ATOMS, StrokeColorStyleValue, "red, blue");

		// StrokeDashCornerStyleValue: none | <length>
		assert_parse!(CssAtomSet::ATOMS, StrokeDashCornerStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, StrokeDashCornerStyleValue, "10px");

		// StrokeDashoffsetStyleValue: <length-percentage>
		assert_parse!(CssAtomSet::ATOMS, StrokeDashoffsetStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, StrokeDashoffsetStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, StrokeDashoffsetStyleValue, "50%");

		// StrokeLinecapStyleValue: butt | round | square
		assert_parse!(CssAtomSet::ATOMS, StrokeLinecapStyleValue, "butt");
		assert_parse!(CssAtomSet::ATOMS, StrokeLinecapStyleValue, "round");
		assert_parse!(CssAtomSet::ATOMS, StrokeLinecapStyleValue, "square");

		// StrokeMiterlimitStyleValue: <number>
		assert_parse!(CssAtomSet::ATOMS, StrokeMiterlimitStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, StrokeMiterlimitStyleValue, "4");

		// StrokeOpacityStyleValue: <'opacity'>
		assert_parse!(CssAtomSet::ATOMS, StrokeOpacityStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, StrokeOpacityStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, StrokeOpacityStyleValue, "0.5");

		// StrokeOriginStyleValue: match-parent | fill-box | stroke-box | content-box | padding-box | border-box
		assert_parse!(CssAtomSet::ATOMS, StrokeOriginStyleValue, "match-parent");
		assert_parse!(CssAtomSet::ATOMS, StrokeOriginStyleValue, "fill-box");
		assert_parse!(CssAtomSet::ATOMS, StrokeOriginStyleValue, "border-box");

		// StrokePositionStyleValue: <position>#
		assert_parse!(CssAtomSet::ATOMS, StrokePositionStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, StrokePositionStyleValue, "top left");

		// StrokeWidthStyleValue: [ <length-percentage> | <line-width> ]#
		assert_parse!(CssAtomSet::ATOMS, StrokeWidthStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, StrokeWidthStyleValue, "thin");
		assert_parse!(CssAtomSet::ATOMS, StrokeWidthStyleValue, "medium");
		assert_parse!(CssAtomSet::ATOMS, StrokeWidthStyleValue, "thick");
		assert_parse!(CssAtomSet::ATOMS, StrokeWidthStyleValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, StrokeWidthStyleValue, "1px, 2px");
	}

	#[test]
	fn test_stroke_dash_justify() {
		assert_parse!(CssAtomSet::ATOMS, StrokeDashJustifyStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, StrokeDashJustifyStyleValue, "stretch");
		assert_parse!(CssAtomSet::ATOMS, StrokeDashJustifyStyleValue, "dashes");
		assert_parse!(CssAtomSet::ATOMS, StrokeDashJustifyStyleValue, "stretch dashes");
		assert_parse!(CssAtomSet::ATOMS, StrokeDashJustifyStyleValue, "stretch dashes gaps");
		assert_peek_false!(CssAtomSet::ATOMS, StrokeDashJustifyStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, StrokeDashJustifyStyleValue, "1px");
	}

	#[test]
	fn test_stroke_linejoin() {
		assert_parse!(CssAtomSet::ATOMS, StrokeLinejoinStyleValue, "crop");
		assert_parse!(CssAtomSet::ATOMS, StrokeLinejoinStyleValue, "arcs");
		assert_parse!(CssAtomSet::ATOMS, StrokeLinejoinStyleValue, "miter");
		assert_parse!(CssAtomSet::ATOMS, StrokeLinejoinStyleValue, "crop bevel");
		assert_parse!(CssAtomSet::ATOMS, StrokeLinejoinStyleValue, "bevel crop");
		assert_peek_false!(CssAtomSet::ATOMS, StrokeLinejoinStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, StrokeLinejoinStyleValue, "1px");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FillBreakStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FillBreakStyleValue, "bounding-box slice");
		assert_peek_false!(CssAtomSet::ATOMS, FillRuleStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FillRuleStyleValue, "nonzero evenodd");
		assert_peek_false!(CssAtomSet::ATOMS, StrokeAlignStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, StrokeAlignStyleValue, "center inset");
		assert_peek_false!(CssAtomSet::ATOMS, StrokeBreakStyleValue, "none");
		assert_peek_false!(CssAtomSet::ATOMS, StrokeLinecapStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, StrokeLinecapStyleValue, "butt round");
	}
}
