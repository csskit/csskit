use css_parse::{Build, Cursor, DimensionUnit, Parser, T, ToNumberValue};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, Visitable};

// https://drafts.csswg.org/css-values/#angles
#[derive(IntoCursor, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.types.angle"))]
#[visit(self)]
pub enum Angle {
	Grad(T![Dimension::Grad]),
	Rad(T![Dimension::Rad]),
	Turn(T![Dimension::Turn]),
	Deg(T![Dimension::Deg]),
}

impl From<Angle> for f32 {
	fn from(val: Angle) -> Self {
		match val {
			Angle::Grad(f) => f.into(),
			Angle::Rad(f) => f.into(),
			Angle::Turn(f) => f.into(),
			Angle::Deg(f) => f.into(),
		}
	}
}

impl ToNumberValue for Angle {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl Angle {
	const DEG_GRAD: f32 = 0.9;
	const DEG_RAD: f32 = 57.295_78;
	const DEG_TURN: f32 = 360.0;

	pub fn as_degrees(&self) -> f32 {
		match self {
			Self::Grad(d) => Into::<f32>::into(*d) * Self::DEG_GRAD,
			Self::Rad(d) => Into::<f32>::into(*d) * Self::DEG_RAD,
			Self::Turn(d) => Into::<f32>::into(*d) * Self::DEG_TURN,
			Self::Deg(d) => (*d).into(),
		}
	}
}

impl<'a> Build<'a> for Angle {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		match c.token().dimension_unit() {
			DimensionUnit::Grad => Self::Grad(<T![Dimension::Grad]>::build(p, c)),
			DimensionUnit::Rad => Self::Rad(<T![Dimension::Rad]>::build(p, c)),
			DimensionUnit::Turn => Self::Turn(<T![Dimension::Turn]>::build(p, c)),
			DimensionUnit::Deg => Self::Deg(<T![Dimension::Deg]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

#[derive(IntoCursor, Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub enum AngleOrZero {
	Angle(Angle),
	#[visit(skip)]
	#[parse(in_range = 0..0)]
	Zero(T![Number]),
}

#[derive(IntoCursor, Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub enum AngleOrNumber {
	Angle(Angle),
	#[visit(skip)]
	Zero(T![Number]),
}

impl From<AngleOrZero> for f32 {
	fn from(val: AngleOrZero) -> Self {
		match val {
			AngleOrZero::Angle(f) => f.into(),
			AngleOrZero::Zero(f) => f.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Angle>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Angle, "0grad");
		assert_parse!(Angle, "0deg");
		assert_parse!(AngleOrZero, "0", AngleOrZero::Zero(_));
	}
}
