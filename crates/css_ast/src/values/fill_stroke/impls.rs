#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FillBreakStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FillColorStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<FillOpacityStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FillOriginStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FillPositionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<FillRepeatStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<FillRuleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<FillSizeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeAlignStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeBreakStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeColorStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeDashCornerStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeDasharrayStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeDashoffsetStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeLinecapStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeMiterlimitStyleValue>(), 12);
		assert_eq!(std::mem::size_of::<StrokeOpacityStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokeOriginStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<StrokePositionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeRepeatStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeSizeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<StrokeWidthStyleValue>(), 32);
	}

	#[test]
	fn test_writes() {
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
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FillBreakStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FillBreakStyleValue, "bounding-box slice");
		assert_parse_error!(CssAtomSet::ATOMS, FillRuleStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FillRuleStyleValue, "nonzero evenodd");
		assert_parse_error!(CssAtomSet::ATOMS, StrokeAlignStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, StrokeAlignStyleValue, "center inset");
		assert_parse_error!(CssAtomSet::ATOMS, StrokeBreakStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, StrokeLinecapStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, StrokeLinecapStyleValue, "butt round");
	}
}
