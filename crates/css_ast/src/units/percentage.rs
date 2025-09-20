use super::prelude::*;

#[derive(IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Percentage(T![Dimension]);

impl Percentage {
	pub fn value(&self) -> f32 {
		self.0.into()
	}
}

impl From<Percentage> for f32 {
	fn from(percentage: Percentage) -> Self {
		percentage.0.into()
	}
}

impl From<Percentage> for i32 {
	fn from(percentage: Percentage) -> Self {
		f32::from(percentage) as i32
	}
}

impl ToNumberValue for Percentage {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl<'a> Peek<'a> for Percentage {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::peek(p, c) && c.token().dimension_unit() == DimensionUnit::Percent
	}
}

impl<'a> Parse<'a> for Percentage {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if !p.peek::<Self>() {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		} else {
			p.parse::<T![Dimension]>().map(Self)
		}
	}
}

#[derive(Peek, Parse, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum NumberPercentage {
	Number(T![Number]),
	Percentage(Percentage),
}

impl From<NumberPercentage> for f32 {
	fn from(val: NumberPercentage) -> Self {
		match val {
			NumberPercentage::Number(n) => n.into(),
			NumberPercentage::Percentage(n) => n.into(),
		}
	}
}

impl ToNumberValue for NumberPercentage {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Percentage>(), 12);
		assert_eq!(std::mem::size_of::<NumberPercentage>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Percentage, "1%");
	}
}
