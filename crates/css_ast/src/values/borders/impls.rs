#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

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
