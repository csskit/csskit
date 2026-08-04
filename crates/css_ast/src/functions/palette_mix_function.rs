use super::prelude::*;
use crate::{CalcableValue, ColorInterpolationMethod, PaletteIdentifier, Percentage, Ranged, Value};

/// <https://drafts.csswg.org/css-fonts-5/#typedef-palette-mix-function>
///
/// ```text,ignore
/// <palette-mix()> = palette-mix( <color-interpolation-method> , [ [ normal | light | dark | <palette-identifier> ] && <percentage [0%,100%]>? ]#{2} )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(all))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PaletteMixFunction<'a> {
	#[atom(CssAtomSet::PaletteMix)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub interpolation: ColorInterpolationMethod,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub interpolation_comma: Option<T![,]>,
	pub first: PaletteMixFunctionParams<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma: Option<T![,]>,
	pub second: PaletteMixFunctionParams<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// A palette with an optional percentage in a `palette-mix()` function.
///
/// ```text,ignore
/// [ normal | light | dark | <palette-identifier> ] && <percentage [0%,100%]>?
/// ```
///
/// The palette and percentage can appear in either order.
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PaletteMixFunctionParams<'a> {
	pub palette: PaletteMixPalette<'a>,
	pub percentage: Option<CalcableValue<'a, Ranged<Percentage, 0, 100>>>,
}

impl<'a> Peek<'a> for PaletteMixFunctionParams<'a> {
	const PEEK_KINDSET: KindSet =
		PaletteMixPalette::PEEK_KINDSET.combine(CalcableValue::<Ranged<Percentage, 0, 100>>::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		PaletteMixPalette::peek(p, c) || CalcableValue::<Ranged<Percentage, 0, 100>>::peek(p, c)
	}
}

impl<'a> Parse<'a> for PaletteMixFunctionParams<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut palette = p.parse_if_peek::<PaletteMixPalette>()?;
		let percentage = p.parse_if_peek::<CalcableValue<Ranged<Percentage, 0, 100>>>()?;
		if palette.is_none() {
			palette = Some(p.parse::<PaletteMixPalette>()?);
		}
		Ok(Self { palette: palette.unwrap(), percentage })
	}
}

/// The palette selected by one side of a `palette-mix()` function.
///
/// ```text,ignore
/// normal | light | dark | <palette-identifier>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PaletteMixPalette<'a> {
	#[atom(CssAtomSet::Normal)]
	Normal(T![Ident]),
	#[atom(CssAtomSet::Light)]
	Light(T![Ident]),
	#[atom(CssAtomSet::Dark)]
	Dark(T![Ident]),
	Identifier(Value<'a, PaletteIdentifier>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PaletteMixFunction, "palette-mix(in lch,normal,dark)");
		assert_parse!(CssAtomSet::ATOMS, PaletteMixFunction, "palette-mix(in oklab,light 40%,dark 60%)");
		assert_parse!(CssAtomSet::ATOMS, PaletteMixFunction, "palette-mix(in lch,40% light,60% dark)");
		assert_parse!(CssAtomSet::ATOMS, PaletteMixFunction, "palette-mix(in lch longer hue,--blues 30%,--reds)");
		assert_parse!(CssAtomSet::ATOMS, PaletteMixFunction, "palette-mix(in lch,--blues var(--pct),--reds)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PaletteMixFunction, "palette-mix(normal,dark)");
		assert_parse_error!(CssAtomSet::ATOMS, PaletteMixFunction, "palette-mix(in lch,normal)");
		assert_parse_error!(CssAtomSet::ATOMS, PaletteMixFunction, "palette-mix(in lch,normal,dark,light)");
		assert_parse_error!(CssAtomSet::ATOMS, PaletteMixFunction, "palette-mix(in lch,normal 120%,dark)");
		assert_parse_error!(CssAtomSet::ATOMS, PaletteMixFunction, "palette-mix(in lch,30%,dark)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!(
			"palette-mix(in lch,--blues 30%,--reds)",
			PaletteMixFunction,
			ColorInterpolationMethod,
			PaletteMixPalette,
			Percentage,
			PaletteMixPalette,
		);
	}
}
