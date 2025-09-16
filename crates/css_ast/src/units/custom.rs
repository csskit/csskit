use css_parse::{Cursor, Diagnostic, DimensionUnit, Parse, Parser, Peek, Result, T};
use csskit_derives::{IntoCursor, ToCursors};

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct CustomDimension(T![Dimension]);

impl From<CustomDimension> for f32 {
	fn from(custom: CustomDimension) -> Self {
		custom.0.into()
	}
}

impl<'a> Peek<'a> for CustomDimension {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Dimension]>::peek(p, c) && c == DimensionUnit::Unknown && p.parse_str(c).starts_with("--")
	}
}

impl<'a> Parse<'a> for CustomDimension {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<Self>() {
			let c = p.next();
			Ok(Self(T![Dimension](c)))
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
		assert_eq!(std::mem::size_of::<CustomDimension>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CustomDimension, "1--foo");
	}
}
