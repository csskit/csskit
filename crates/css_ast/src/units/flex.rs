use super::prelude::*;

// https://www.w3.org/TR/css-grid-2/#typedef-flex
#[derive(IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Flex(T![Dimension]);

impl From<Flex> for f32 {
	fn from(flex: Flex) -> Self {
		flex.0.into()
	}
}

impl ToNumberValue for Flex {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl<'a> Peek<'a> for Flex {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::peek(p, c) && c.token().dimension_unit() == DimensionUnit::Fr
	}
}

impl<'a> Parse<'a> for Flex {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		p.parse::<T![Dimension]>().map(Self)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Flex>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Flex, "1fr");
	}
}
