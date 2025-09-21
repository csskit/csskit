use super::prelude::*;
use crate::CSSInt;

#[derive(ToSpan, Peek, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PositiveNonZeroInt(pub CSSInt);

impl<'a> Parse<'a> for PositiveNonZeroInt {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let num = p.parse::<CSSInt>()?;
		if 0.0f32 >= num.into() {
			Err(Diagnostic::new(num.into(), Diagnostic::number_too_small))?
		}

		Ok(Self(num))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PositiveNonZeroInt>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PositiveNonZeroInt, "1");
		assert_parse!(CssAtomSet::ATOMS, PositiveNonZeroInt, "100");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PositiveNonZeroInt, "0");
		assert_parse_error!(CssAtomSet::ATOMS, PositiveNonZeroInt, "0.0");
		assert_parse_error!(CssAtomSet::ATOMS, PositiveNonZeroInt, "-1");
		assert_parse_error!(CssAtomSet::ATOMS, PositiveNonZeroInt, "1.2");
		assert_parse_error!(CssAtomSet::ATOMS, PositiveNonZeroInt, "-1.2");
	}
}
