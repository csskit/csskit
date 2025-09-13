use super::prelude::*;

// const DPPX_IN: f32 = 96.0;
// const DPPX_CM: f32 = DPPX_IN / 2.54;

// https://drafts.csswg.org/css-values/#resolution
#[derive(ToCursors, Parse, Peek, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Resolution {
	#[atom(CssAtomSet::Dpi)]
	Dpi(T![Dimension]),
	#[atom(CssAtomSet::Dpcm)]
	Dpcm(T![Dimension]),
	#[atom(CssAtomSet::Dppx)]
	Dppx(T![Dimension]),
	#[atom(CssAtomSet::X)]
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Resolution>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Resolution, "1dppx");
		assert_parse!(CssAtomSet::ATOMS, Resolution, "1x");
	}
}
