use super::prelude::*;

use crate::{types::Color, units::LengthPercentageOrFlex};

function_set!(pub struct StripesFunctionName "stripes");

/// <https://drafts.csswg.org/css-images-4/#typedef-image-1d>
///
/// ```text,ignore
/// <stripes()> = stripes( <color-stripe># )
/// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct StripesFunction<'a>(Function<StripesFunctionName, CommaSeparated<'a, ColorStripe>>);

/// <https://drafts.csswg.org/css-images-4/#typedef-color-stripe>
///
/// ```text,ignore
/// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
/// ```
#[derive(ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub struct ColorStripe {
	pub color: Color,
	pub thickness: Option<LengthPercentageOrFlex>,
}

impl<'a> Peek<'a> for ColorStripe {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		Color::peek(p, c) || LengthPercentageOrFlex::peek(p, c)
	}
}

impl<'a> Parse<'a> for ColorStripe {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut color = p.parse_if_peek::<Color>()?;
		let thickness = p.parse_if_peek::<LengthPercentageOrFlex>()?;
		if color.is_none() {
			color = Some(p.parse::<Color>()?);
		}
		Ok(Self { color: color.unwrap(), thickness })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StripesFunction>(), 64);
		assert_eq!(std::mem::size_of::<ColorStripe>(), 160);
	}

	#[test]
	fn test_writes() {
		assert_parse!(StripesFunction, "stripes(red 1fr,green 2fr,blue 100px)");
		assert_parse!(
			StripesFunction,
			"stripes(0.1fr red,0.2fr green,100px blue)",
			"stripes(red 0.1fr,green 0.2fr,blue 100px)"
		);
		assert_parse!(
			StripesFunction,
			"stripes(red 1fr,2fr green,blue 100px)",
			"stripes(red 1fr,green 2fr,blue 100px)"
		);
	}
}
