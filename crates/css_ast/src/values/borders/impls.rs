pub(crate) use crate::traits::StyleValue;
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::assert_parse;

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<BorderTopColorStyleValue>(), 160);
		assert_eq!(std::mem::size_of::<BorderRightColorStyleValue>(), 160);
		assert_eq!(std::mem::size_of::<BorderBottomColorStyleValue>(), 160);
		assert_eq!(std::mem::size_of::<BorderLeftColorStyleValue>(), 160);
		assert_eq!(std::mem::size_of::<BorderBlockStartColorStyleValue>(), 160);
		assert_eq!(std::mem::size_of::<BorderBlockEndColorStyleValue>(), 160);
		assert_eq!(std::mem::size_of::<BorderInlineStartColorStyleValue>(), 160);
		assert_eq!(std::mem::size_of::<BorderInlineEndColorStyleValue>(), 160);
		// assert_eq!(std::mem::size_of::<BorderColorStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<BorderBlockColorStyleValue>(), 320);
		assert_eq!(std::mem::size_of::<BorderInlineColorStyleValue>(), 320);
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
		assert_eq!(std::mem::size_of::<BorderTopStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<BorderRightStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<BorderBottomStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<BorderLeftStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<BorderBlockStartStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<BorderBlockEndStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<BorderInlineStartStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<BorderInlineEndStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<BorderBlockStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<BorderInlineStyleValue>(), 192);
		assert_eq!(std::mem::size_of::<BorderTopLeftRadiusStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderTopRightRadiusStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderBottomRightRadiusStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderBottomLeftRadiusStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderStartStartRadiusStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderStartEndRadiusStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderEndStartRadiusStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<BorderEndEndRadiusStyleValue>(), 32);
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
		assert_eq!(std::mem::size_of::<CornerTopLeftShapeStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<CornerTopRightShapeStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<CornerBottomRightShapeStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<CornerBottomLeftShapeStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<CornerStartStartShapeStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<CornerStartEndShapeStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<CornerEndStartShapeStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<CornerEndEndShapeStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<CornerTopShapeStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerRightShapeStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerBottomShapeStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerLeftShapeStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerBlockStartShapeStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerBlockEndShapeStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerInlineStartShapeStyleValue>(), 88);
		assert_eq!(std::mem::size_of::<CornerInlineEndShapeStyleValue>(), 88);
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
	}

	#[test]
	fn test_writes() {
		assert_parse!(BorderTopColorStyleValue, "red");
		assert_parse!(BorderClipStyleValue, "1fr");
		assert_parse!(BorderClipStyleValue, "1fr 1fr 1fr");
		assert_parse!(BorderClipStyleValue, "1fr 20px 2fr 40rem");
		assert_parse!(BoxShadowPositionStyleValue, "outset");
		assert_parse!(BoxShadowPositionStyleValue, "inset,inset");
		assert_parse!(BoxShadowPositionStyleValue, "inset,inset,inset,outset,inset");
	}
}
