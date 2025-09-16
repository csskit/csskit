use css_parse::{Cursor, Diagnostic, Parse, Parser, Peek, Result, T};
use csskit_derives::{IntoCursor, ToCursors};

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(transparent))]
pub struct CSSFloat(T![Number]);

impl CSSFloat {
	#[allow(non_upper_case_globals)]
	pub const Zero: CSSFloat = CSSFloat(<T![Number]>::ZERO);
}

impl From<CSSFloat> for i32 {
	fn from(value: CSSFloat) -> Self {
		value.0.into()
	}
}

impl From<CSSFloat> for f32 {
	fn from(value: CSSFloat) -> Self {
		value.0.into()
	}
}

impl<'a> Peek<'a> for CSSFloat {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Number]>::peek(p, c) && c.token().is_float()
	}
}

impl<'a> Parse<'a> for CSSFloat {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let c = p.next();
			Ok(Self(T![Number](c)))
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CSSFloat>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CSSFloat, "0.01");
		assert_parse!(CSSFloat, "3.141");
	}
}
