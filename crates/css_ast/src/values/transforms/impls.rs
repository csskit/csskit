use super::{RotateStyleValue, TransformOriginStyleValue};
use crate::{Angle, CalcableValue, CssAtomSet, KeywordValue, Length, Number, NumericValue, PositionOne, PositionTwo};
use css_parse::{Cursor, Diagnostic, Parse, Parser, Peek, Result as ParseResult, T};

impl<'a> Parse<'a> for TransformOriginStyleValue<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let first = p.parse::<PositionOne>()?;
		if !p.peek::<PositionOne>() {
			return Ok(Self::PositionOne(crate::Value::Literal(first)));
		}
		let second_c = p.peek_n(1);
		let second = p.parse::<PositionOne>()?;
		let two = PositionTwo::from_two(p, first, second, second_c)?;
		Ok(Self::PositionTwo(crate::Value::Literal(two), p.parse_if_peek::<CalcableValue<Length>>()?))
	}
}

type Vector<'a> = (NumericValue<'a, Number>, NumericValue<'a, Number>, NumericValue<'a, Number>);

impl<'a> Parse<'a> for RotateStyleValue<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if <T![Ident]>::peek(p, c) {
			if p.equals_atom(c, &CssAtomSet::None) {
				return p.parse::<KeywordValue<'a, T![Ident]>>().map(Self::None);
			}
			let variant = match p.to_atom::<CssAtomSet>(c) {
				CssAtomSet::X => RotateStyleValue::XAngle,
				CssAtomSet::Y => RotateStyleValue::YAngle,
				CssAtomSet::Z => RotateStyleValue::ZAngle,
				_ => Err(Diagnostic::new(c, Diagnostic::unexpected))?,
			};
			let keyword = p.parse::<KeywordValue<'a, T![Ident]>>()?;
			return Ok(variant(keyword, p.parse::<CalcableValue<'a, Angle>>()?));
		}
		if <T![Number]>::peek(p, c) {
			let vector = p.parse::<Vector<'a>>()?;
			return Ok(Self::NumberAngle(vector, p.parse::<CalcableValue<'a, Angle>>()?));
		}
		let angle = p.parse::<CalcableValue<'a, Angle>>()?;
		let c = p.peek_n(1);
		if <T![Ident]>::peek(p, c) {
			let variant = match p.to_atom::<CssAtomSet>(c) {
				CssAtomSet::X => RotateStyleValue::XAngle,
				CssAtomSet::Y => RotateStyleValue::YAngle,
				CssAtomSet::Z => RotateStyleValue::ZAngle,
				_ => return Ok(Self::Angle(angle)),
			};
			let keyword = p.parse::<KeywordValue<'a, T![Ident]>>()?;
			return Ok(variant(keyword, angle));
		} else if <T![Number]>::peek(p, c) {
			return Ok(Self::NumberAngle(p.parse::<Vector<'a>>()?, angle));
		}
		Ok(Self::Angle(angle))
	}
}

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

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
	fn test_rotate() {
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "45deg");
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "x 45deg");
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "y 45deg");
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "z 45deg");
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "45deg x");
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "1 2 3 45deg");
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "45deg 1 2 3");
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "z calc(45deg * 2)");
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "x var(--angle)");
		assert_parse!(CssAtomSet::ATOMS, RotateStyleValue, "var(--angle) y");
	}

	#[test]
	fn test_rotate_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, RotateStyleValue, "x");
		assert_parse_error!(CssAtomSet::ATOMS, RotateStyleValue, "1 2 45deg");
		assert_parse_error!(CssAtomSet::ATOMS, RotateStyleValue, "none 45deg");
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
