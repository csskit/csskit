#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<BorderTopColorStyleValue>(), 144);
		assert_eq!(std::mem::size_of::<BorderRightColorStyleValue>(), 144);
		assert_eq!(std::mem::size_of::<BorderBottomColorStyleValue>(), 144);
		assert_eq!(std::mem::size_of::<BorderLeftColorStyleValue>(), 144);
		assert_eq!(std::mem::size_of::<BorderBlockStartColorStyleValue>(), 144);
		assert_eq!(std::mem::size_of::<BorderBlockEndColorStyleValue>(), 144);
		assert_eq!(std::mem::size_of::<BorderInlineStartColorStyleValue>(), 144);
		assert_eq!(std::mem::size_of::<BorderInlineEndColorStyleValue>(), 144);
		// assert_eq!(std::mem::size_of::<BorderColorStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BorderBlockColorStyleValue>(), 288);
		assert_eq!(std::mem::size_of::<BorderInlineColorStyleValue>(), 288);
		assert_eq!(std::mem::size_of::<BorderTopStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderRightStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderBottomStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderLeftStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderBlockStartStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderBlockEndStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderInlineEndStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderBlockStyleStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderInlineStyleStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderTopWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderRightWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderBottomWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderLeftWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderBlockStartWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderBlockEndWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderInlineStartWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderInlineEndWidthStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderBlockWidthStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderInlineWidthStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderTopStyleValue>(), 172);
		assert_eq!(std::mem::size_of::<BorderRightStyleValue>(), 172);
		assert_eq!(std::mem::size_of::<BorderBottomStyleValue>(), 172);
		assert_eq!(std::mem::size_of::<BorderLeftStyleValue>(), 172);
		assert_eq!(std::mem::size_of::<BorderBlockStartStyleValue>(), 172);
		assert_eq!(std::mem::size_of::<BorderBlockEndStyleValue>(), 172);
		assert_eq!(std::mem::size_of::<BorderInlineStartStyleValue>(), 172);
		assert_eq!(std::mem::size_of::<BorderInlineEndStyleValue>(), 172);
		assert_eq!(std::mem::size_of::<BorderBlockStyleValue>(), 172);
		assert_eq!(std::mem::size_of::<BorderInlineStyleValue>(), 172);
		assert_eq!(std::mem::size_of::<BorderTopLeftRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderTopRightRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderBottomRightRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderBottomLeftRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderStartStartRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderStartEndRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderEndStartRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderEndEndRadiusStyleValue>(), 48);
		// assert_eq!(std::mem::size_of::<BorderTopRadiusStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderRightRadiusStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderBottomRadiusStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderLeftRadiusStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderBlockStartRadiusStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderBlockEndRadiusStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderInlineStartRadiusStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderInlineEndRadiusStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderRadiusStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<CornerShapeStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<CornerTopLeftShapeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<CornerTopRightShapeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<CornerBottomRightShapeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<CornerBottomLeftShapeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<CornerStartStartShapeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<CornerStartEndShapeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<CornerEndStartShapeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<CornerEndEndShapeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<CornerTopShapeStyleValue>(), 80);
		assert_eq!(std::mem::size_of::<CornerRightShapeStyleValue>(), 80);
		assert_eq!(std::mem::size_of::<CornerBottomShapeStyleValue>(), 80);
		assert_eq!(std::mem::size_of::<CornerLeftShapeStyleValue>(), 80);
		assert_eq!(std::mem::size_of::<CornerBlockStartShapeStyleValue>(), 80);
		assert_eq!(std::mem::size_of::<CornerBlockEndShapeStyleValue>(), 80);
		assert_eq!(std::mem::size_of::<CornerInlineStartShapeStyleValue>(), 80);
		assert_eq!(std::mem::size_of::<CornerInlineEndShapeStyleValue>(), 80);
		// assert_eq!(std::mem::size_of::<BorderLimitStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderClipStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderClipTopStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderClipRightStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderClipBottomStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderClipLeftStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BoxShadowColorStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BoxShadowOffsetStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BoxShadowBlurStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BoxShadowSpreadStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BoxShadowPositionStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<BoxShadowStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BorderImageSourceStyleValue>(), 208);
		// assert_eq!(std::mem::size_of::<BorderImageSliceStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderImageWidthStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BorderImageOutsetStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<BorderImageRepeatStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<BorderImageStyleValue>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BorderTopColorStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, BorderClipStyleValue, "1fr");
		assert_parse!(CssAtomSet::ATOMS, BorderClipStyleValue, "1fr 1fr 1fr");
		assert_parse!(CssAtomSet::ATOMS, BorderClipStyleValue, "1fr 20px 2fr 40rem");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowPositionStyleValue, "outset");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowPositionStyleValue, "inset,inset");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowPositionStyleValue, "inset,inset,inset,outset,inset");
		assert_parse!(CssAtomSet::ATOMS, BorderImageOutsetStyleValue, "10");
		assert_parse!(CssAtomSet::ATOMS, BorderImageOutsetStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, BorderImageOutsetStyleValue, "10px 10rem 10q 10em");
		assert_parse!(CssAtomSet::ATOMS, BorderImageOutsetStyleValue, "10 1ric 10 10");
		assert_parse!(CssAtomSet::ATOMS, BorderImageRepeatStyleValue, "stretch");
		assert_parse!(CssAtomSet::ATOMS, BorderImageRepeatStyleValue, "stretch stretch");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BorderImageOutsetStyleValue, "-10");
	}
}
