use super::prelude::*;

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
		<T![Dimension]>::peek(p, c)
			&& p.to_source_cursor(c).source()[c.token().numeric_len() as usize..].starts_with("--")
	}
}

impl<'a> Parse<'a> for CustomDimension {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<Self>() {
			p.parse::<T![Dimension]>().map(Self)
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
		assert_eq!(std::mem::size_of::<CustomDimension>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CustomDimension, "1--foo");
	}
}
