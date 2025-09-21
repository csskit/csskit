use super::prelude::*;
use crate::{Length, LengthPercentage};

/// <https://drafts.csswg.org/css-images-3/#typedef-gradient>
/// ```text-ignore,
/// <gradient> = <linear-gradient()> | <repeating-linear-gradient()> | <radial-gradient()> | <repeating-radial-gradient()>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum Gradient<'a> {
	#[atom(CssAtomSet::LinearGradient)]
	LinearGradientFunction(LinearGradientFunction<'a>),
	#[atom(CssAtomSet::RepeatingLinearGradient)]
	RepeatingLinearGradientFunction(RepeatingLinearGradientFunction<'a>),
	#[atom(CssAtomSet::RadialGradient)]
	RadialGradientFunction(RadialGradientFunction<'a>),
	#[atom(CssAtomSet::RepeatingRadialGradient)]
	RepeatingRadialGradientFunction(RepeatingRadialGradientFunction<'a>),
}

/// <https://drafts.csswg.org/css-images-3/#funcdef-linear-gradient>
/// ```text,ignore
/// <linear-gradient()> = linear-gradient( [ <linear-gradient-syntax> ] )
/// <linear-gradient-syntax> = [ <angle> | <zero> | to <side-or-corner> ]? , <color-stop-list>
/// <side-or-corner> = [left | right] || [top | bottom]
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LinearGradientFunction<'a> {
	#[atom(CssAtomSet::LinearGradient)]
	pub name: T![Function],
	pub params: LinearGradientFunctionParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LinearGradientFunctionParams<'a>(
	Option<LinearDirection>,
	Option<T![,]>,
	CommaSeparated<'a, ColorStopOrHint>,
);

/// <https://drafts.csswg.org/css-images-3/#funcdef-repeating-linear-gradient>
/// ```text,ignore
/// <repeating-linear-gradient()> = repeating-linear-gradient( [ <linear-gradient-syntax> ] )
/// <linear-gradient-syntax> = [ <angle> | <zero> | to <side-or-corner> ]? , <color-stop-list>
/// <side-or-corner> = [left | right] || [top | bottom]
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RepeatingLinearGradientFunction<'a> {
	#[atom(CssAtomSet::RepeatingLinearGradient)]
	pub name: T![Function],
	pub params: RepeatingLinearGradientFunctionParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct RepeatingLinearGradientFunctionParams<'a>(
	Option<LinearDirection>,
	Option<T![,]>,
	CommaSeparated<'a, ColorStopOrHint>,
);

/// <https://drafts.csswg.org/css-images-3/#funcdef-radial-gradient>
/// ```text,ignore
/// <radial-gradient()> = radial-gradient( [ <radial-gradient-syntax> ] )
/// <radial-gradient-syntax> = [ <radial-shape> || <radial-size> ]? [ at <position> ]? , <color-stop-list>
/// <radial-size> = <radial-extent> | <length [0,∞]> | <length-percentage [0,∞]>{2}
/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
/// <radial-shape> = circle | ellipse
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RadialGradientFunction<'a> {
	#[atom(CssAtomSet::RadialGradient)]
	pub name: T![Function],
	pub params: RadialGradientFunctionParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct RadialGradientFunctionParams<'a>(
	Option<RadialSize>,
	Option<RadialShape>,
	Option<T![Ident]>,
	Option<Position>,
	Option<T![,]>,
	CommaSeparated<'a, ColorStopOrHint>,
);

/// <https://drafts.csswg.org/css-images-3/#funcdef-repeating-radial-gradient>
/// ```text,ignore
/// <repeating-radial-gradient()> = repeating-radial-gradient( [ <radial-gradient-syntax> ] )
/// <radial-gradient-syntax> = [ <radial-shape> || <radial-size> ]? [ at <position> ]? , <color-stop-list>
/// <radial-size> = <radial-extent> | <length [0,∞]> | <length-percentage [0,∞]>{2}
/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
/// <radial-shape> = circle | ellipse
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RepeatingRadialGradientFunction<'a> {
	#[atom(CssAtomSet::RepeatingRadialGradient)]
	pub name: T![Function],
	pub params: RepeatingRadialGradientFunctionParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct RepeatingRadialGradientFunctionParams<'a>(
	Option<RadialSize>,
	Option<RadialShape>,
	Option<T![Ident]>,
	Option<Position>,
	Option<T![,]>,
	CommaSeparated<'a, ColorStopOrHint>,
);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum NamedDirection {
	#[atom(CssAtomSet::Bottom)]
	Bottom(T![Ident]),
	#[atom(CssAtomSet::Top)]
	Top(T![Ident]),
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
}

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LinearDirection {
	Angle(Angle),
	Named(#[atom(CssAtomSet::To)] T![Ident], NamedDirection, Option<NamedDirection>),
}

/// <https://drafts.csswg.org/css-images-3/#typedef-radial-size>
///
/// ```text,ignore
/// <radial-size> = <radial-extent> | <length [0,∞]> | <length-percentage [0,∞]>{2}
/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
/// ```
#[derive(Peek, ToSpan, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum RadialSize {
	Extent(RadialExtent),
	Circular(Length),
	Elliptical(LengthPercentage, LengthPercentage),
}

/// <https://drafts.csswg.org/css-images-3/#typedef-radial-extent>
///
/// ```text,ignore
/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum RadialExtent {
	#[atom(CssAtomSet::ClosestCorner)]
	ClosestCorner(T![Ident]),
	#[atom(CssAtomSet::ClosestSide)]
	ClosestSide(T![Ident]),
	#[atom(CssAtomSet::FarthestCorner)]
	FarthestCorner(T![Ident]),
	#[atom(CssAtomSet::FarthestSide)]
	FarthestSide(T![Ident]),
}

impl<'a> Parse<'a> for RadialSize {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(extent) = p.parse_if_peek::<RadialExtent>()? {
			return Ok(RadialSize::Extent(extent));
		}
		if p.peek::<Length>() {
			let first_len = p.parse::<LengthPercentage>()?;
			if !p.peek::<Length>()
				&& let LengthPercentage::Length(len) = first_len
			{
				return Ok(Self::Circular(len));
			}
			let second_len = p.parse::<LengthPercentage>()?;
			return Ok(Self::Elliptical(first_len, second_len));
		}
		let first = p.parse::<LengthPercentage>()?;
		let second = p.parse::<LengthPercentage>()?;
		Ok(Self::Elliptical(first, second))
	}
}

/// <https://drafts.csswg.org/css-images-3/#typedef-radial-shape>
///
/// ```text,ignore
/// <radial-shape> = circle | ellipse
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum RadialShape {
	#[atom(CssAtomSet::Circle)]
	Circle(T![Ident]),
	#[atom(CssAtomSet::Ellipse)]
	Ellipse(T![Ident]),
}

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ColorStopOrHint {
	Hint(LengthPercentage),
	Stop(Color, Option<LengthPercentage>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Gradient>(), 208);
		assert_eq!(std::mem::size_of::<LinearDirection>(), 44);
		assert_eq!(std::mem::size_of::<RadialSize>(), 32);
		assert_eq!(std::mem::size_of::<ColorStopOrHint>(), 156);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(to bottom,yellow,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(yellow,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(to bottom,#fff,#fff 85%,#e6e6e6)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(45deg,#808080 25%,transparent 25%)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(to right,transparent,red 20%,red 80%,transparent)");
		assert_parse!(
			CssAtomSet::ATOMS,
			Gradient,
			"radial-gradient(closest-corner circle,rgba(1,65,255,0.4),rgba(1,65,255,0))"
		);
	}
}
