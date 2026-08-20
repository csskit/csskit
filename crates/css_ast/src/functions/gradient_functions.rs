use css_parse::Box;

use super::prelude::*;
use crate::{CalcableValue, LengthPercentage};

/// <https://drafts.csswg.org/css-images-3/#typedef-gradient>
/// ```text-ignore,
/// <gradient> = <linear-gradient()> | <repeating-linear-gradient()> | <radial-gradient()> | <repeating-radial-gradient()>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Gradient<'a> {
	#[atom(CssAtomSet::LinearGradient)]
	LinearGradientFunction(Box<'a, LinearGradientFunction<'a>>),
	#[atom(CssAtomSet::RepeatingLinearGradient)]
	RepeatingLinearGradientFunction(Box<'a, RepeatingLinearGradientFunction<'a>>),
	#[atom(CssAtomSet::RadialGradient)]
	RadialGradientFunction(Box<'a, RadialGradientFunction<'a>>),
	#[atom(CssAtomSet::RepeatingRadialGradient)]
	RepeatingRadialGradientFunction(Box<'a, RepeatingRadialGradientFunction<'a>>),
}

/// <https://drafts.csswg.org/css-images-3/#funcdef-linear-gradient>
/// ```text,ignore
/// <linear-gradient()> = linear-gradient( [ <linear-gradient-syntax> ] )
/// <linear-gradient-syntax> = [ <angle> | <zero> | to <side-or-corner> ]? , <color-stop-list>
/// <side-or-corner> = [left | right] || [top | bottom]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LinearGradientFunction<'a> {
	#[atom(CssAtomSet::LinearGradient)]
	pub name: T![Function],
	pub params: LinearGradientFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LinearGradientFunctionParams<'a>(
	Option<LinearDirection<'a>>,
	#[semantic_eq(skip)] Option<T![,]>,
	CommaSeparated<'a, ColorStopOrHint<'a>>,
);

/// <https://drafts.csswg.org/css-images-3/#funcdef-repeating-linear-gradient>
/// ```text,ignore
/// <repeating-linear-gradient()> = repeating-linear-gradient( [ <linear-gradient-syntax> ] )
/// <linear-gradient-syntax> = [ <angle> | <zero> | to <side-or-corner> ]? , <color-stop-list>
/// <side-or-corner> = [left | right] || [top | bottom]
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RepeatingLinearGradientFunction<'a> {
	#[atom(CssAtomSet::RepeatingLinearGradient)]
	pub name: T![Function],
	pub params: RepeatingLinearGradientFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RepeatingLinearGradientFunctionParams<'a>(
	Option<LinearDirection<'a>>,
	#[semantic_eq(skip)] Option<T![,]>,
	CommaSeparated<'a, ColorStopOrHint<'a>>,
);

/// <https://drafts.csswg.org/css-images-3/#funcdef-radial-gradient>
/// ```text,ignore
/// <radial-gradient()> = radial-gradient( [ <radial-gradient-syntax> ] )
/// <radial-gradient-syntax> = [ <radial-shape> || <radial-size> ]? [ at <position> ]? , <color-stop-list>
/// <radial-size> = <radial-extent> | <length [0,∞]> | <length-percentage [0,∞]>{2}
/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
/// <radial-shape> = circle | ellipse
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RadialGradientFunction<'a> {
	#[atom(CssAtomSet::RadialGradient)]
	pub name: T![Function],
	pub params: RadialGradientFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RadialGradientFunctionParams<'a>(
	Option<RadialSize<'a>>,
	Option<RadialShape>,
	Option<T![Ident]>,
	Option<Position<'a>>,
	#[semantic_eq(skip)] Option<T![,]>,
	CommaSeparated<'a, ColorStopOrHint<'a>>,
);

/// <https://drafts.csswg.org/css-images-3/#funcdef-repeating-radial-gradient>
/// ```text,ignore
/// <repeating-radial-gradient()> = repeating-radial-gradient( [ <radial-gradient-syntax> ] )
/// <radial-gradient-syntax> = [ <radial-shape> || <radial-size> ]? [ at <position> ]? , <color-stop-list>
/// <radial-size> = <radial-extent> | <length [0,∞]> | <length-percentage [0,∞]>{2}
/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
/// <radial-shape> = circle | ellipse
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RepeatingRadialGradientFunction<'a> {
	#[atom(CssAtomSet::RepeatingRadialGradient)]
	pub name: T![Function],
	pub params: RepeatingRadialGradientFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RepeatingRadialGradientFunctionParams<'a>(
	Option<RadialSize<'a>>,
	Option<RadialShape>,
	Option<T![Ident]>,
	Option<Position<'a>>,
	#[semantic_eq(skip)] Option<T![,]>,
	CommaSeparated<'a, ColorStopOrHint<'a>>,
);

#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
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

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum LinearDirection<'a> {
	Angle(CalcableValue<'a, Angle>),
	Named(#[atom(CssAtomSet::To)] T![Ident], NamedDirection, Option<NamedDirection>),
}

/// <https://drafts.csswg.org/css-images-4/#typedef-radial-size>
/// <https://drafts.csswg.org/css-images-4/#radial-size>
///
/// ```text,ignore
/// <radial-size> = <radial-extent>{1,2} | <length-percentage [0,∞]>{1,2}
/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
/// ```
#[node]
#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RadialSize<'a> {
	Extent(RadialExtent, Option<RadialExtent>),
	Circular(CalcableValue<'a, LengthPercentage>),
	Elliptical(CalcableValue<'a, LengthPercentage>, CalcableValue<'a, LengthPercentage>),
}

/// <https://drafts.csswg.org/css-images-3/#typedef-radial-extent>
///
/// ```text,ignore
/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
/// ```
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
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

impl<'a> Parse<'a> for RadialSize<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if let Some(extent) = p.parse_if_peek::<RadialExtent>()? {
			let second = p.parse_if_peek::<RadialExtent>()?;
			return Ok(RadialSize::Extent(extent, second));
		}
		let first = p.parse::<CalcableValue<LengthPercentage>>()?;
		if let Some(second) = p.parse_if_peek::<CalcableValue<LengthPercentage>>()? {
			return Ok(Self::Elliptical(first, second));
		}
		Ok(Self::Circular(first))
	}
}

/// <https://drafts.csswg.org/css-images-3/#typedef-radial-shape>
///
/// ```text,ignore
/// <radial-shape> = circle | ellipse
/// ```
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RadialShape {
	#[atom(CssAtomSet::Circle)]
	Circle(T![Ident]),
	#[atom(CssAtomSet::Ellipse)]
	Ellipse(T![Ident]),
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ColorStopOrHint<'a> {
	Hint(CalcableValue<'a, LengthPercentage>),
	Stop(Color<'a>, Option<CalcableValue<'a, LengthPercentage>>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

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

	#[test]
	fn test_substitution() {
		// calc()/var() permitted in angle, stop-position, and radial-size slots.
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(calc(45deg),red,blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "linear-gradient(red calc(20% + 5px),blue)");
		assert_parse!(CssAtomSet::ATOMS, Gradient, "radial-gradient(calc(50px) circle,red,blue)");
	}
}
