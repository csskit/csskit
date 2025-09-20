use super::prelude::*;

// const DPPX_IN: f32 = 96.0;
// const DPPX_CM: f32 = DPPX_IN / 2.54;

// https://drafts.csswg.org/css-values/#resolution
#[derive(Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Resolution {
	Dpi(T![Dimension]),
	Dpcm(T![Dimension]),
	Dppx(T![Dimension]),
	X(T![Dimension]),
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

impl<'a> Parse<'a> for Resolution {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let c = p.peek_n(1);
		match c.token().dimension_unit() {
			DimensionUnit::Dpi => p.parse::<T![Dimension]>().map(Self::Dpi),
			DimensionUnit::Dpcm => p.parse::<T![Dimension]>().map(Self::Dpcm),
			DimensionUnit::Dppx => p.parse::<T![Dimension]>().map(Self::Dppx),
			DimensionUnit::X => p.parse::<T![Dimension]>().map(Self::X),
			_ => Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?,
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
