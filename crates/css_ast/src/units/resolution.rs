use css_parse::{Build, Cursor, DimensionUnit, Parser, T};
use csskit_derives::{IntoCursor, Peek, ToCursors};

// const DPPX_IN: f32 = 96.0;
// const DPPX_CM: f32 = DPPX_IN / 2.54;

// https://drafts.csswg.org/css-values/#resolution
#[derive(Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Resolution {
	Dpi(T![Dimension::Dpi]),
	Dpcm(T![Dimension::Dpcm]),
	Dppx(T![Dimension::Dppx]),
	X(T![Dimension::X]),
}

impl From<Resolution> for f32 {
	fn from(res: Resolution) -> Self {
		match res {
			Resolution::Dpi(r) => r.into(),
			Resolution::Dpcm(r) => r.into(),
			Resolution::Dppx(r) => r.into(),
			Resolution::X(r) => r.into(),
		}
	}
}

impl<'a> Build<'a> for Resolution {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		match c.token().dimension_unit() {
			DimensionUnit::Dpi => Self::Dpi(<T![Dimension::Dpi]>::build(p, c)),
			DimensionUnit::Dpcm => Self::Dpcm(<T![Dimension::Dpcm]>::build(p, c)),
			DimensionUnit::Dppx => Self::Dppx(<T![Dimension::Dppx]>::build(p, c)),
			DimensionUnit::X => Self::X(<T![Dimension::X]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Resolution>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Resolution, "1dppx");
		assert_parse!(Resolution, "1x");
	}
}
