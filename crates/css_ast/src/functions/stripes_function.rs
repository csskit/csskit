use super::prelude::*;
use crate::{types::Color, units::LengthPercentageOrFlex};

/// <https://drafts.csswg.org/css-images-4/#typedef-image-1d>
///
/// ```text,ignore
/// <stripes()> = stripes( <color-stripe># )
/// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct StripesFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Stripes)]
	pub name: T![Function],
	pub params: CommaSeparated<'a, ColorStripe>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-images-4/#typedef-color-stripe>
///
/// ```text,ignore
/// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
/// ```
#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
pub struct ColorStripe {
	pub color: Color,
	pub thickness: Option<LengthPercentageOrFlex>,
}

impl<'a> Peek<'a> for ColorStripe {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Color::peek(p, c) || LengthPercentageOrFlex::peek(p, c)
	}
}

impl<'a> Parse<'a> for ColorStripe {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
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
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StripesFunction>(), 56);
		assert_eq!(std::mem::size_of::<ColorStripe>(), 156);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StripesFunction, "stripes(red 1fr,green 2fr,blue 100px)");
		assert_parse!(CssAtomSet::ATOMS, StripesFunction, "stripes(0.1fr red,0.2fr green,100px blue)");
		assert_parse!(CssAtomSet::ATOMS, StripesFunction, "stripes(red 1fr,2fr green,blue 100px)");
	}
}
