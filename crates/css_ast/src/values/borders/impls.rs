#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<BorderTopColorStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderRightColorStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderBottomColorStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderLeftColorStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderBlockStartColorStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderBlockEndColorStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderInlineStartColorStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderInlineEndColorStyleValue>(), 56);
		// assert_eq!(std::mem::size_of::<BorderColorStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BorderBlockColorStyleValue>(), 120);
		assert_eq!(std::mem::size_of::<BorderInlineColorStyleValue>(), 120);
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
		assert_eq!(std::mem::size_of::<BorderTopStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderRightStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderBottomStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderLeftStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderBlockStartStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderBlockEndStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderInlineStartStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderInlineEndStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderBlockStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderInlineStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderTopLeftRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderTopRightRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderBottomRightRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderBottomLeftRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderStartStartRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderStartEndRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderEndStartRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderEndEndRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderTopRadiusStyleValue>(), 76);
		assert_eq!(std::mem::size_of::<BorderRightRadiusStyleValue>(), 76);
		assert_eq!(std::mem::size_of::<BorderBottomRadiusStyleValue>(), 76);
		assert_eq!(std::mem::size_of::<BorderLeftRadiusStyleValue>(), 76);
		assert_eq!(std::mem::size_of::<BorderBlockStartRadiusStyleValue>(), 76);
		assert_eq!(std::mem::size_of::<BorderBlockEndRadiusStyleValue>(), 76);
		assert_eq!(std::mem::size_of::<BorderInlineStartRadiusStyleValue>(), 76);
		assert_eq!(std::mem::size_of::<BorderInlineEndRadiusStyleValue>(), 76);
		assert_eq!(std::mem::size_of::<BorderRadiusStyleValue>(), 140);
		assert_eq!(std::mem::size_of::<CornerShapeStyleValue>(), 160);
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
		assert_eq!(std::mem::size_of::<CornerTopLeftStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerTopRightStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerBottomLeftStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerBottomRightStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerStartStartStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerStartEndStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerEndStartStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerEndEndStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerTopStyleValue>(), 156);
		assert_eq!(std::mem::size_of::<CornerLeftStyleValue>(), 156);
		assert_eq!(std::mem::size_of::<CornerRightStyleValue>(), 156);
		assert_eq!(std::mem::size_of::<CornerBottomStyleValue>(), 156);
		assert_eq!(std::mem::size_of::<CornerBlockStartStyleValue>(), 156);
		assert_eq!(std::mem::size_of::<CornerBlockEndStyleValue>(), 156);
		assert_eq!(std::mem::size_of::<CornerInlineStartStyleValue>(), 156);
		assert_eq!(std::mem::size_of::<CornerInlineEndStyleValue>(), 156);
		assert_eq!(std::mem::size_of::<CornerStyleValue>(), 300);
		assert_eq!(std::mem::size_of::<BorderLimitStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderTopClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderRightClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderBottomClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderLeftClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderBlockClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderBlockStartClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderBlockEndClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderInlineClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderInlineStartClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderInlineEndClipStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderInlineStartStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<BorderStyleStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<BorderStyleValue>(), 56);
		assert_eq!(std::mem::size_of::<BorderWidthStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<BoxShadowColorStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BoxShadowOffsetStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BoxShadowBlurStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BoxShadowSpreadStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BoxShadowPositionStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BoxShadowStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderImageSourceStyleValue>(), 128);
		// assert_eq!(std::mem::size_of::<BorderImageSliceStyleValue>(), 1);
		// assert_eq!(std::mem::size_of::<BorderImageWidthStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BorderImageOutsetStyleValue>(), 64);
		assert_eq!(std::mem::size_of::<BorderImageRepeatStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<BorderImageStyleValue>(), 1);
	}

	#[test]
	fn test_box_shadow_offset() {
		assert_parse!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "10px 20px");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "none,10px");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "10px,20px 30px");
		assert_parse_error!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "red");
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

	#[test]
	fn test_corner_shorthands() {
		assert_parse!(CssAtomSet::ATOMS, CornerTopLeftStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, CornerTopLeftStyleValue, "round");
		assert_parse!(CssAtomSet::ATOMS, CornerTopLeftStyleValue, "10px round");
		assert_parse!(CssAtomSet::ATOMS, CornerTopRightStyleValue, "5px squircle");
		assert_parse!(CssAtomSet::ATOMS, CornerTopStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, CornerTopStyleValue, "10px 20px");
		assert_parse_error!(CssAtomSet::ATOMS, CornerTopLeftStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, CornerTopLeftStyleValue, "red");
	}
}
