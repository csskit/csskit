use super::prelude::*;

#[derive(IntoCursor, Parse, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Decibel(T![Dimension]);

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

impl<'a> Peek<'a> for Decibel {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::peek(p, c) && c.token().dimension_unit() == DimensionUnit::Db
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Decibel>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Decibel, "1db");
	}
}
