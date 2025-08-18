use css_lexer::{Cursor, DimensionUnit};
use css_parse::{Build, Parser, T};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors};

// const DEG_GRAD: f32 = 0.9;
// const DEG_RAD: f32 = 57.295_78;
// const DEG_TURN: f32 = 360.0;

// https://drafts.csswg.org/css-values/#angles
#[derive(Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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

#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AngleOrZero {
	Angle(Angle),
	Zero(T![Number]),
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
	}
}
