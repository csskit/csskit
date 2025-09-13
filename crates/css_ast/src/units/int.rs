use super::prelude::*;

#[derive(IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
#[visit(skip)]
pub struct CSSInt(T![Number]);

impl CSSInt {
	#[allow(non_upper_case_globals)]
	pub const Zero: CSSInt = CSSInt(<T![Number]>::ZERO);
}

impl From<CSSInt> for i32 {
	fn from(value: CSSInt) -> Self {
		value.0.into()
	}
}

impl From<CSSInt> for f32 {
	fn from(value: CSSInt) -> Self {
		value.0.into()
	}
}

impl ToNumberValue for CSSInt {
	fn to_number_value(&self) -> Option<f32> {
		Some(self.0.into())
	}
}

impl<'a> Peek<'a> for CSSInt {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Number]>::peek(p, c) && c.token().is_int()
	}
}

impl<'a> Parse<'a> for CSSInt {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<Self>() {
			p.parse::<T![Number]>().map(Self)
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
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
		assert_eq!(std::mem::size_of::<CSSInt>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CSSInt, "0");
		assert_parse!(CssAtomSet::ATOMS, CSSInt, "999999");
	}
}
