use super::TransformOriginStyleValue;
use crate::{Length, PositionOne, PositionTwo};
use css_parse::{Cursor, Diagnostic, Parse, Parser, Peek, Result as ParseResult, T};

impl<'a> Parse<'a> for TransformOriginStyleValue {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let first = p.parse::<PositionOne>()?;
		let Some(second) = p.parse_if_peek::<PositionOne>()? else { return Ok(Self::PositionOne(first)) };
		let two = PositionTwo::from_two(p, first, second)?;
		Ok(Self::PositionTwo(two, p.parse_if_peek::<Length>()?))
	}
}

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TransformStyleValue>(), 24);
		// assert_eq!(std::mem::size_of::<TransformOriginStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TransformBoxStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<TranslateStyleValue>(), 48);
		// assert_eq!(std::mem::size_of::<RotateStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<ScaleStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<TransformStyleStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PerspectiveStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PerspectiveOriginStyleValue>(), 68);
		assert_eq!(std::mem::size_of::<BackfaceVisibilityStyleValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TransformStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, TransformStyleValue, "scale(1)");
		assert_parse!(CssAtomSet::ATOMS, TransformBoxStyleValue, "fill-box");
		assert_parse!(CssAtomSet::ATOMS, ScaleStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, ScaleStyleValue, "1%");
		assert_parse!(CssAtomSet::ATOMS, ScaleStyleValue, "1 2 3");
		assert_parse!(CssAtomSet::ATOMS, ScaleStyleValue, "1.7 50%");
		assert_parse!(CssAtomSet::ATOMS, TransformStyleStyleValue, "flat");
		assert_parse!(CssAtomSet::ATOMS, PerspectiveOriginStyleValue, "1px");
		assert_parse!(CssAtomSet::ATOMS, BackfaceVisibilityStyleValue, "visible");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, TransformStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, ScaleStyleValue, "none none");
	}

	#[test]
	fn test_transform_origin() {
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "left");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "center");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "right");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "top");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "bottom");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "left top");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "left center");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "left bottom");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "center top");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "center center");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "center bottom");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "right top");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "right center");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "right bottom");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "50% 50%");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "10px 20px");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "0 0");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "left 50%");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "right 25%");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "center 10px");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "50% top");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "75% bottom");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "10px center");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "left top 5px");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "center center 0px");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "50% 50% 10px");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "right bottom 100px");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "10px 20px 30px");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "top left");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "top center");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "top right");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "bottom left");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "bottom center");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "bottom right");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "top left 5px");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "bottom right 20px");
		assert_parse!(CssAtomSet::ATOMS, TransformOriginStyleValue, "top center 0px");
	}

	#[test]
	fn test_transform_origin_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, TransformOriginStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, TransformOriginStyleValue, "top top");
		assert_parse_error!(CssAtomSet::ATOMS, TransformOriginStyleValue, "left left");
		assert_parse_error!(CssAtomSet::ATOMS, TransformOriginStyleValue, "left top 50%");
		assert_parse_error!(CssAtomSet::ATOMS, TransformOriginStyleValue, "left top 0px extra");
	}
}
