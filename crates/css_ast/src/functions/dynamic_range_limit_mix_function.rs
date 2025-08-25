use css_parse::{
	CommaSeparated, Cursor, Function, Parse, Parser, Result as ParserResult, T, diagnostics, function_set,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

function_set!(pub struct DynamicRangeLimitMixFunctionName "dynamic-range-limit-mix");

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct DynamicRangeLimitMixFunction<'a>(
	Function<DynamicRangeLimitMixFunctionName, CommaSeparated<'a, DynamicRangeLimitMixFunctionParams>>,
);

#[derive(Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DynamicRangeLimitMixFunctionParams(T![Ident], T![Dimension::%]);

impl<'a> Parse<'a> for DynamicRangeLimitMixFunctionParams {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let ident = p.parse::<T![Ident]>()?;
		let length = p.parse::<T![Dimension::%]>()?;
		let c: Cursor = length.into();
		if !(0.0..=100.0).contains(&c.token().value()) {
			Err(diagnostics::NumberOutOfBounds(c.token().value(), format!("{:?}", 0.0..=100.0), c.into()))?
		}
		Ok(Self(ident, length))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DynamicRangeLimitMixFunction>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DynamicRangeLimitMixFunction, "dynamic-range-limit-mix(high 80%,standard 20%)");
		assert_parse!(DynamicRangeLimitMixFunction, "dynamic-range-limit-mix(high 8%,standard 2%)");
	}
}
