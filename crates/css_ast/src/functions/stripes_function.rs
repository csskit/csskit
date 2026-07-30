use super::prelude::*;
use crate::{CalcableValue, types::Color, units::LengthPercentageOrFlex};

/// <https://drafts.csswg.org/css-images-4/#typedef-image-1d>
///
/// ```text,ignore
/// <stripes()> = stripes( <color-stripe># )
/// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct StripesFunction<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Stripes)]
	pub name: T![Function],
	pub params: CommaSeparated<'a, ColorStripe<'a>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-images-4/#typedef-color-stripe>
///
/// ```text,ignore
/// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
/// ```
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorStripe<'a> {
	pub color: Color<'a>,
	pub thickness: Option<CalcableValue<'a, LengthPercentageOrFlex>>,
}

impl<'a> Peek<'a> for ColorStripe<'a> {
	const PEEK_KINDSET: KindSet = Color::PEEK_KINDSET.combine(CalcableValue::<LengthPercentageOrFlex>::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Color::peek(p, c) || CalcableValue::<LengthPercentageOrFlex>::peek(p, c)
	}
}

impl<'a> Parse<'a> for ColorStripe<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut color = p.parse_if_peek::<Color>()?;
		let thickness = p.parse_if_peek::<CalcableValue<LengthPercentageOrFlex>>()?;
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
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StripesFunction, "stripes(red 1fr,green 2fr,blue 100px)");
		assert_parse!(CssAtomSet::ATOMS, StripesFunction, "stripes(0.1fr red,0.2fr green,100px blue)");
		assert_parse!(CssAtomSet::ATOMS, StripesFunction, "stripes(red 1fr,2fr green,blue 100px)");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, StripesFunction, "stripes(red var(--t),green 2fr)");
		assert_parse!(CssAtomSet::ATOMS, StripesFunction, "stripes(red calc(1px + 2px),blue 100px)");
	}
}
