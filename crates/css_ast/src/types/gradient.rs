use bumpalo::collections::Vec;
use css_lexer::{Cursor, Kind, ToSpan};
use css_parse::{Build, Parse, Parser, Peek, Result as ParserResult, T, diagnostics, function_set, keyword_set};
use csskit_derives::{ToCursors, ToSpan};

use crate::{
	types::Position,
	units::{Angle, Length, LengthPercentage},
};

use super::Color;

function_set!(
	pub enum GradientFunctionName {
		LinearGradient: "linear-gradient",
		RadialGradient: "radial-gradient",
		RepeatingLinearGradient: "repeating-linear-gradient",
		RepeatingRadialGradient: "repeating-radial-gradient",
	}
);

// https://drafts.csswg.org/css-images-3/#typedef-gradient
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Gradient<'a> {
	Linear(T![Function], Option<LinearDirection>, Option<T![,]>, Vec<'a, ColorStopOrHint>, Option<T![')']>),
	RepeatingLinear(T![Function], Option<LinearDirection>, Option<T![,]>, Vec<'a, ColorStopOrHint>, Option<T![')']>),
	Radial(
		T![Function],
		Option<RadialSize>,
		Option<RadialShape>,
		Option<T![Ident]>,
		Option<Position>,
		Option<T![,]>,
		Vec<'a, ColorStopOrHint>,
		Option<T![')']>,
	),
	RepeatingRadial(
		T![Function],
		Option<RadialSize>,
		Option<RadialShape>,
		Option<T![Ident]>,
		Option<Position>,
		Option<T![,]>,
		Vec<'a, ColorStopOrHint>,
		Option<T![')']>,
	),
}

impl<'a> Gradient<'a> {
	fn parse_stops(p: &mut Parser<'a>) -> ParserResult<Vec<'a, ColorStopOrHint>> {
		let mut stops = Vec::new_in(p.bump());
		let mut allow_hint = false;
		loop {
			if allow_hint && p.peek::<LengthPercentage>() {
				let hint = p.parse::<LengthPercentage>()?;
				let comma = p.parse::<T![,]>()?;
				stops.push(ColorStopOrHint::Hint(hint, comma));
			}
			let color = p.parse::<Color>()?;
			let hint = if p.peek::<LengthPercentage>() {
				let hint = p.parse::<LengthPercentage>()?;
				allow_hint = true;
				Some(hint)
			} else {
				None
			};
			let comma = p.parse_if_peek::<T![,]>()?;
			stops.push(ColorStopOrHint::Stop(color, hint, comma));
			if comma.is_none() {
				break;
			}
		}
		Ok(stops)
	}
}

impl<'a> Peek<'a> for Gradient<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		GradientFunctionName::peek(p, c)
	}
}

impl<'a> Parse<'a> for Gradient<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<GradientFunctionName>()?;
		match function {
			GradientFunctionName::LinearGradient(c) => {
				let dir = p.parse_if_peek::<LinearDirection>()?;
				let comma = if dir.is_some() { p.parse_if_peek::<T![,]>()? } else { None };
				let stops = Self::parse_stops(p)?;
				Ok(Self::Linear(<T![Function]>::build(p, c), dir, comma, stops, p.parse_if_peek::<T![')']>()?))
			}
			GradientFunctionName::RepeatingLinearGradient(c) => {
				let dir = p.parse_if_peek::<LinearDirection>()?;
				let comma = if dir.is_some() { p.parse_if_peek::<T![,]>()? } else { None };
				let stops = Self::parse_stops(p)?;
				Ok(Self::RepeatingLinear(<T![Function]>::build(p, c), dir, comma, stops, p.parse_if_peek::<T![')']>()?))
			}
			GradientFunctionName::RadialGradient(c) => {
				let mut size = p.parse_if_peek::<RadialSize>()?;
				let shape = p.parse_if_peek::<RadialShape>()?;
				if size.is_none() && shape.is_none() {
					size = Some(p.parse::<RadialSize>()?);
				}
				let maybe_at = p.peek_n(1);
				let at = if maybe_at == Kind::Ident && p.eq_ignore_ascii_case(maybe_at, "at") {
					Some(p.parse::<T![Ident]>()?)
				} else {
					None
				};
				let position = if at.is_some() { p.parse_if_peek::<Position>()? } else { None };
				let comma = if size.is_some() || shape.is_some() { p.parse_if_peek::<T![,]>()? } else { None };
				let stops = Self::parse_stops(p)?;
				Ok(Self::Radial(
					<T![Function]>::build(p, c),
					size,
					shape,
					at,
					position,
					comma,
					stops,
					p.parse_if_peek::<T![')']>()?,
				))
			}
			GradientFunctionName::RepeatingRadialGradient(c) => {
				let mut size = p.parse_if_peek::<RadialSize>()?;
				let shape = p.parse_if_peek::<RadialShape>()?;
				if size.is_none() && shape.is_none() {
					size = Some(p.parse::<RadialSize>()?);
				}
				let at = if c == Kind::Ident && p.eq_ignore_ascii_case(c, "at") {
					Some(p.parse::<T![Ident]>()?)
				} else {
					None
				};
				let position = if at.is_some() { p.parse_if_peek::<Position>()? } else { None };
				let comma = if size.is_some() || shape.is_some() { p.parse_if_peek::<T![,]>()? } else { None };
				let stops = Self::parse_stops(p)?;
				Ok(Self::RepeatingRadial(
					<T![Function]>::build(p, c),
					size,
					shape,
					at,
					position,
					comma,
					stops,
					p.parse_if_peek::<T![')']>()?,
				))
			}
		}
	}
}

keyword_set!(pub enum NamedDirection { Bottom: "bottom", Top: "top", Left: "left", Right: "right" });

#[derive(ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LinearDirection {
	Angle(Angle),
	Named(T![Ident], NamedDirection, Option<NamedDirection>),
}

impl<'a> Peek<'a> for LinearDirection {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		Angle::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "to"))
	}
}

impl<'a> Parse<'a> for LinearDirection {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<Angle>() {
			p.parse::<Angle>().map(Self::Angle)
		} else {
			let to = p.parse::<T![Ident]>()?;
			let c: Cursor = to.into();
			if !p.eq_ignore_ascii_case(c, "to") {
				Err(diagnostics::UnexpectedIdent(p.parse_str(c).into(), to.to_span()))?
			}
			let first = p.parse::<NamedDirection>()?;
			let second = p.parse_if_peek::<NamedDirection>()?;
			Ok(Self::Named(to, first, second))
		}
	}
}

// https://drafts.csswg.org/css-images-3/#typedef-rg-size
#[derive(ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum RadialSize {
	ClosestCorner(T![Ident]),
	ClosestSide(T![Ident]),
	FarthestCorner(T![Ident]),
	FarthestSide(T![Ident]),
	Circular(Length),
	Elliptical(LengthPercentage, LengthPercentage),
}

keyword_set!(pub enum RadialSizeKeyword {
	ClosestCorner: "closest-corner",
	ClosestSide: "closest-side",
	FarthestCorner: "farthest-corner",
	FarthestSide: "farthest-side",
});

impl<'a> Peek<'a> for RadialSize {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		LengthPercentage::peek(p, c) || RadialSizeKeyword::peek(p, c)
	}
}

impl<'a> Parse<'a> for RadialSize {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(keyword) = p.parse_if_peek::<RadialSizeKeyword>()? {
			let ident = <T![Ident]>::build(p, keyword.into());
			return Ok(match keyword {
				RadialSizeKeyword::ClosestCorner(_) => RadialSize::ClosestCorner(ident),
				RadialSizeKeyword::ClosestSide(_) => RadialSize::ClosestSide(ident),
				RadialSizeKeyword::FarthestCorner(_) => RadialSize::FarthestCorner(ident),
				RadialSizeKeyword::FarthestSide(_) => RadialSize::FarthestSide(ident),
			});
		}
		if p.peek::<T![Number]>() {
			let first_len = p.parse::<LengthPercentage>()?;
			if !p.peek::<T![Number]>() {
				return p.parse::<Length>().map(Self::Circular);
			}
			let second_len = p.parse::<LengthPercentage>()?;
			return Ok(Self::Elliptical(first_len, second_len));
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

#[derive(ToSpan, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum ColorStopOrHint {
	Stop(Color, Option<LengthPercentage>, Option<T![,]>),
	Hint(LengthPercentage, T![,]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Gradient>(), 208);
		assert_eq!(std::mem::size_of::<LinearDirection>(), 44);
		assert_eq!(std::mem::size_of::<RadialSize>(), 32);
		assert_eq!(std::mem::size_of::<ColorStopOrHint>(), 192);
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
