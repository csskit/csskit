use css_parse::{Build, CommaSeparated, Function, Parse, Parser, Result as ParserResult, T, function_set, keyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

use crate::{Angle, Color, Length, LengthPercentage, Position};

/// <https://drafts.csswg.org/css-images-3/#typedef-gradient>
/// ```text-ignore,
/// <gradient> = <linear-gradient()> | <repeating-linear-gradient()> | <radial-gradient()> | <repeating-radial-gradient()>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum Gradient<'a> {
	LinearGradientFunction(LinearGradientFunction<'a>),
	RepeatingLinearGradientFunction(RepeatingLinearGradientFunction<'a>),
	RadialGradientFunction(RadialGradientFunction<'a>),
	RepeatingRadialGradientFunction(RepeatingRadialGradientFunction<'a>),
}

function_set!(pub struct LinearGradientFunctionName "linear-gradient");

/// <https://drafts.csswg.org/css-images-3/#funcdef-linear-gradient>
/// ```text,ignore
/// <linear-gradient()> = linear-gradient( [ <linear-gradient-syntax> ] )
/// <linear-gradient-syntax> = [ <angle> | <zero> | to <side-or-corner> ]? , <color-stop-list>
/// <side-or-corner> = [left | right] || [top | bottom]
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LinearGradientFunction<'a>(Function<LinearGradientFunctionName, LinearGradientFunctionParams<'a>>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct LinearGradientFunctionParams<'a>(
	Option<LinearDirection>,
	Option<T![,]>,
	CommaSeparated<'a, ColorStopOrHint>,
);

function_set!(pub struct RepeatingLinearGradientFunctionName "repeating-linear-gradient");

/// <https://drafts.csswg.org/css-images-3/#funcdef-repeating-linear-gradient>
/// ```text,ignore
/// <repeating-linear-gradient()> = repeating-linear-gradient( [ <linear-gradient-syntax> ] )
/// <linear-gradient-syntax> = [ <angle> | <zero> | to <side-or-corner> ]? , <color-stop-list>
/// <side-or-corner> = [left | right] || [top | bottom]
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RepeatingLinearGradientFunction<'a>(
	Function<RepeatingLinearGradientFunctionName, RepeatingLinearGradientFunctionParams<'a>>,
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct RepeatingLinearGradientFunctionParams<'a>(
	Option<LinearDirection>,
	Option<T![,]>,
	CommaSeparated<'a, ColorStopOrHint>,
);

function_set!(pub struct RadialGradientFunctionName "radial-gradient");

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
pub struct RadialGradientFunction<'a>(Function<RadialGradientFunctionName, RadialGradientFunctionParams<'a>>);

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

function_set!(pub struct RepeatingRadialGradientFunctionName "repeating-radial-gradient");

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
pub struct RepeatingRadialGradientFunction<'a>(
	Function<RepeatingRadialGradientFunctionName, RepeatingRadialGradientFunctionParams<'a>>,
);

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

keyword_set!(pub struct AtKeyword "at");
keyword_set!(pub struct ToKeyword "to");

keyword_set!(pub enum NamedDirection { Bottom: "bottom", Top: "top", Left: "left", Right: "right" });

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LinearDirection {
	Angle(Angle),
	Named(ToKeyword, NamedDirection, Option<NamedDirection>),
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

keyword_set!(
	/// <https://drafts.csswg.org/css-images-3/#typedef-radial-extent>
	///
	/// ```text,ignore
	/// <radial-extent> = closest-corner | closest-side | farthest-corner | farthest-side
	/// ```
	pub enum RadialExtent {
		ClosestCorner: "closest-corner",
		ClosestSide: "closest-side",
		FarthestCorner: "farthest-corner",
		FarthestSide: "farthest-side",
	}
);

impl<'a> Parse<'a> for RadialSize {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(extent) = p.parse_if_peek::<RadialExtent>()? {
			return Ok(RadialSize::Extent(extent));
		}
		if p.peek::<Length>() {
			let first_len = p.parse::<Length>()?;
			if !p.peek::<Length>() {
				return p.parse::<Length>().map(Self::Circular);
			}
			let second_len = p.parse::<LengthPercentage>()?;
			return Ok(Self::Elliptical(LengthPercentage::build(p, first_len.into()), second_len));
		}
		let first = p.parse::<LengthPercentage>()?;
		let second = p.parse::<LengthPercentage>()?;
		Ok(Self::Elliptical(first, second))
	}
}

keyword_set!(
	/// <https://drafts.csswg.org/css-images-3/#typedef-radial-shape>
	///
	/// ```text,ignore
	/// <radial-shape> = circle | ellipse
	/// ```
	pub enum RadialShape {
		Circle: "circle",
		Ellipse: "ellipse"
	}
);

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ColorStopOrHint {
	Hint(LengthPercentage),
	Stop(Color, Option<LengthPercentage>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Gradient>(), 216);
		assert_eq!(std::mem::size_of::<LinearDirection>(), 44);
		assert_eq!(std::mem::size_of::<RadialSize>(), 32);
		assert_eq!(std::mem::size_of::<ColorStopOrHint>(), 160);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Gradient, "linear-gradient(to bottom,yellow,blue)");
		assert_parse!(Gradient, "linear-gradient(yellow,blue)");
		assert_parse!(Gradient, "linear-gradient(to bottom,#fff,#fff 85%,#e6e6e6)");
		assert_parse!(Gradient, "linear-gradient(45deg,#808080 25%,transparent 25%)");
		assert_parse!(Gradient, "linear-gradient(to right,transparent,red 20%,red 80%,transparent)");
		assert_parse!(Gradient, "radial-gradient(closest-corner circle,rgba(1,65,255,0.4),rgba(1,65,255,0))");
	}
}
