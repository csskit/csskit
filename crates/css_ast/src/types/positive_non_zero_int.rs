use crate::{CSSInt, diagnostics};
use css_parse::{Parse, Parser, Result as ParserResult, ToSpan};
use csskit_derives::{Peek, ToCursors, ToSpan};

#[derive(ToSpan, Peek, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PositiveNonZeroInt(pub CSSInt);

impl<'a> Parse<'a> for PositiveNonZeroInt {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let num = p.parse::<CSSInt>()?;
		if 0.0f32 >= num.into() {
			Err(diagnostics::NumberTooSmall(0.0, num.to_span()))?
		}

		Ok(Self(num))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PositiveNonZeroInt>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PositiveNonZeroInt, "1");
		assert_parse!(PositiveNonZeroInt, "100");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PositiveNonZeroInt, "0");
		assert_parse_error!(PositiveNonZeroInt, "0.0");
		assert_parse_error!(PositiveNonZeroInt, "-1");
		assert_parse_error!(PositiveNonZeroInt, "1.2");
		assert_parse_error!(PositiveNonZeroInt, "-1.2");
	}
}
