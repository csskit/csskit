use css_parse::{Build, Cursor, Parser, Peek, T};
use csskit_derives::{IntoCursor, ToCursors, Visitable};

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

impl<'a> Peek<'a> for CSSInt {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Number]>::peek(p, c) && c.token().is_int()
	}
}

impl<'a> Build<'a> for CSSInt {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(<T![Number]>::build(p, c))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CSSInt>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CSSInt, "0");
		assert_parse!(CSSInt, "999999");
	}
}
