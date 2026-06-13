#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

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
		assert_eq!(std::mem::size_of::<BorderColorStyleValue>(), 224);
		assert_eq!(std::mem::size_of::<BorderBlockColorStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderInlineColorStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderTopStyleStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderRightStyleStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBottomStyleStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderLeftStyleStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBlockStartStyleStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBlockEndStyleStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderInlineEndStyleStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBlockStyleStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderInlineStyleStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderTopWidthStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderRightWidthStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBottomWidthStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderLeftWidthStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBlockStartWidthStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBlockEndWidthStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderInlineStartWidthStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderInlineEndWidthStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBlockWidthStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderInlineWidthStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderTopStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderRightStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderBottomStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderLeftStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderBlockStartStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderBlockEndStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderInlineStartStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderInlineEndStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderBlockStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderInlineStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderTopLeftRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderTopRightRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderBottomRightRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderBottomLeftRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderStartStartRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderStartEndRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderEndStartRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderEndEndRadiusStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<BorderTopRadiusStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderRightRadiusStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderBottomRadiusStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderLeftRadiusStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderBlockStartRadiusStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderBlockEndRadiusStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderInlineStartRadiusStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderInlineEndRadiusStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderRadiusStyleValue>(), 208);
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
		assert_eq!(std::mem::size_of::<CornerTopStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<CornerLeftStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<CornerRightStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<CornerBottomStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<CornerBlockStartStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<CornerBlockEndStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<CornerInlineStartStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<CornerInlineEndStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<CornerStyleValue>(), 368);
		assert_eq!(std::mem::size_of::<BorderLimitStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<BorderClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderTopClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderRightClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBottomClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderLeftClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBlockClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBlockStartClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderBlockEndClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderInlineClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderInlineStartClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderInlineEndClipStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderInlineStartStyleStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderStyleStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<BorderStyleValue>(), 72);
		assert_eq!(std::mem::size_of::<BorderWidthStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<BoxShadowColorStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BoxShadowOffsetStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BoxShadowBlurStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BoxShadowSpreadStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BoxShadowPositionStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BoxShadowStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<BorderImageSourceStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<BorderImageSliceStyleValue>(), 112);
		assert_eq!(std::mem::size_of::<BorderImageWidthStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<BorderImageOutsetStyleValue>(), 96);
		assert_eq!(std::mem::size_of::<BorderImageRepeatStyleValue>(), 32);
		// assert_eq!(std::mem::size_of::<BorderImageStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BorderShapeStyleValue<'_>>(), 752);
	}

	#[test]
	fn test_box_shadow_offset() {
		assert_parse!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "10px 20px");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "none,10px");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "10px,20px 30px");
		assert_peek_false!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, BoxShadowOffsetStyleValue, "red");
	}

	#[test]
	fn test_box_shadow() {
		assert_parse!(CssAtomSet::ATOMS, BoxShadowStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowStyleValue, "0 0 0 .2rem rgba(0,123,255,.25)");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowStyleValue, "0 1px 1px rgba(0,0,0,.075)inset");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowStyleValue, "10px 20px 5px red");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowStyleValue, "10px 20px inset");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowStyleValue, "0 0 0 transparent,0 0 0 transparent");
		assert_parse!(CssAtomSet::ATOMS, BoxShadowStyleValue, "0 1px 1px rgba(0,0,0,.075),0 0 6px rgba(0,0,0,.05)");
		assert_peek_false!(CssAtomSet::ATOMS, BoxShadowStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, BoxShadowStyleValue, "red");
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
	fn test_border_color() {
		assert_parse!(CssAtomSet::ATOMS, BorderColorStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, BorderColorStyleValue, "red blue");
		assert_parse!(CssAtomSet::ATOMS, BorderColorStyleValue, "red blue green");
		assert_parse!(CssAtomSet::ATOMS, BorderColorStyleValue, "red blue green yellow");
		assert_parse!(CssAtomSet::ATOMS, BorderColorStyleValue, "stripes(red 1fr,blue 2fr)");
		assert_parse!(CssAtomSet::ATOMS, BorderColorStyleValue, "red stripes(red 1fr,blue 2fr)");
		assert_peek_false!(CssAtomSet::ATOMS, BorderColorStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, BorderColorStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, BorderColorStyleValue, "red blue green yellow purple");
	}

	#[test]
	fn test_border_image_slice() {
		assert_parse!(CssAtomSet::ATOMS, BorderImageSliceStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, BorderImageSliceStyleValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, BorderImageSliceStyleValue, "10 20");
		assert_parse!(CssAtomSet::ATOMS, BorderImageSliceStyleValue, "10 20 30 40");
		assert_parse!(CssAtomSet::ATOMS, BorderImageSliceStyleValue, "10% fill");
		assert_parse!(CssAtomSet::ATOMS, BorderImageSliceStyleValue, "fill 10%");
		assert_peek_false!(CssAtomSet::ATOMS, BorderImageSliceStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, BorderImageSliceStyleValue, "auto");
	}

	#[test]
	fn test_border_image_width() {
		assert_parse!(CssAtomSet::ATOMS, BorderImageWidthStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, BorderImageWidthStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, BorderImageWidthStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, BorderImageWidthStyleValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, BorderImageWidthStyleValue, "1 2");
		assert_parse!(CssAtomSet::ATOMS, BorderImageWidthStyleValue, "1 2 3 4");
		assert_parse!(CssAtomSet::ATOMS, BorderImageWidthStyleValue, "auto 10px 1 50%");
		assert_peek_false!(CssAtomSet::ATOMS, BorderImageWidthStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, BorderImageWidthStyleValue, "none");
	}

	#[test]
	fn test_border_shape() {
		assert_parse!(CssAtomSet::ATOMS, BorderShapeStyleValue, "none");
		assert_peek_false!(CssAtomSet::ATOMS, BorderShapeStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, BorderShapeStyleValue, "auto");
	}

	#[test]
	fn test_corner_shorthands() {
		assert_parse!(CssAtomSet::ATOMS, CornerTopLeftStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, CornerTopLeftStyleValue, "round");
		assert_parse!(CssAtomSet::ATOMS, CornerTopLeftStyleValue, "10px round");
		assert_parse!(CssAtomSet::ATOMS, CornerTopRightStyleValue, "5px squircle");
		assert_parse!(CssAtomSet::ATOMS, CornerTopStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, CornerTopStyleValue, "10px 20px");
		assert_peek_false!(CssAtomSet::ATOMS, CornerTopLeftStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, CornerTopLeftStyleValue, "red");
	}
}
