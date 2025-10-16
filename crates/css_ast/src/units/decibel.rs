use super::prelude::*;

#[derive(IntoCursor, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct Decibel(#[atom(CssAtomSet::Db)] T![Dimension]);

impl From<Decibel> for f32 {
	fn from(percentage: Decibel) -> Self {
		percentage.0.into()
	}
}

impl ToNumberValue for Decibel {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Decibel>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Decibel, "1db");
	}
}
