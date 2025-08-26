use crate::units::Angle;
use css_parse::{Build, Cursor, Parse, Parser, Peek, Result as ParserResult, T, function_set, keyword_set};
use csskit_derives::{IntoCursor, ToCursors, ToSpan, Visitable};

function_set!(
	pub enum ColorFunctionName {
		Color: "color",
		Rgb: "rgb",
		Rgba: "rgba",
		Hsl: "hsl",
		Hsla: "hsla",
		Hwb: "hwb",
		Lab: "lab",
		Lch: "lch",
		Oklab: "oklab",
		Oklch: "oklch",
	}
);

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Hue {
	None(T![Ident]),
	Number(T![Number]),
	Angle(Angle),
}

impl<'a> Peek<'a> for Hue {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Number]>::peek(p, c) || Angle::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "none"))
	}
}

impl<'a> Build<'a> for Hue {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![Number]>::peek(p, c) {
			Self::Number(<T![Number]>::build(p, c))
		} else if Angle::peek(p, c) {
			Self::Angle(Angle::build(p, c))
		} else {
			Self::None(<T![Ident]>::build(p, c))
		}
	}
}

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Channel {
	None(T![Ident]),
	Number(T![Number]),
	Percent(T![Dimension::%]),
}

impl<'a> Peek<'a> for Channel {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Number]>::peek(p, c)
			|| <T![Dimension::%]>::peek(p, c)
			|| (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "none"))
	}
}

impl<'a> Build<'a> for Channel {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![Number]>::peek(p, c) {
			Self::Number(<T![Number]>::build(p, c))
		} else if <T![Dimension::%]>::peek(p, c) {
			Self::Percent(<T![Dimension::%]>::build(p, c))
		} else {
			Self::None(<T![Ident]>::build(p, c))
		}
	}
}

keyword_set!(pub enum ColorSpace {
	Srgb: "srgb",
	SrgbLinear: "srgb-linear",
	DisplayP3: "display-p3",
	A98Rgb: "a98-rgb",
	ProphotoRgb: "prophoto-rgb",
	Rec2020: "rec2020",
	Xyz: "xyz",
	XyzD50: "xyz-d50",
	XyzD65: "xyz-d65",
});

// https://drafts.csswg.org/css-color/#typedef-color-function
#[derive(ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum ColorFunction {
	// https://drafts.csswg.org/css-color/#funcdef-color
	// color() = color( <colorspace-params> [ / [ <alpha-value> | none ] ]? )
	// <colorspace-params> = [ <predefined-rgb-params> | <xyz-params>]
	// <predefined-rgb-params> = <predefined-rgb> [ <number> | <percentage> | none ]{3}
	// <predefined-rgb> = srgb | srgb-linear | display-p3 | a98-rgb | prophoto-rgb | rec2020
	// <xyz-params> = <xyz-space> [ <number> | <percentage> | none ]{3}
	// <xyz-space> = xyz | xyz-d50 | xyz-d65
	Color(T![Function], ColorSpace, Channel, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-rgb
	// rgb() = [ <legacy-rgb-syntax> | <modern-rgb-syntax> ]
	// rgba() = [ <legacy-rgba-syntax> | <modern-rgba-syntax> ]
	// <legacy-rgb-syntax> =   rgb( <percentage>#{3} , <alpha-value>? ) |
	//                   rgb( <number>#{3} , <alpha-value>? )
	// <legacy-rgba-syntax> = rgba( <percentage>#{3} , <alpha-value>? ) |
	//                   rgba( <number>#{3} , <alpha-value>? )
	// <modern-rgb-syntax> = rgb(
	//   [ <number> | <percentage> | none]{3}
	//   [ / [<alpha-value> | none] ]?  )
	// <modern-rgba-syntax> = rgba(
	//   [ <number> | <percentage> | none]{3}
	//   [ / [<alpha-value> | none] ]?  )
	Rgb(
		T![Function],
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
		Option<T![')']>,
	),
	Rgba(
		T![Function],
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
		Option<T![')']>,
	),

	// https://drafts.csswg.org/css-color/#funcdef-hsl
	// hsl() = [ <legacy-hsl-syntax> | <modern-hsl-syntax> ]
	// hsla() = [ <legacy-hsla-syntax> | <modern-hsla-syntax> ]
	// <modern-hsl-syntax> = hsl(
	//     [<hue> | none]
	//     [<percentage> | <number> | none]
	//     [<percentage> | <number> | none]
	//     [ / [<alpha-value> | none] ]? )
	// <modern-hsla-syntax> = hsla(
	//     [<hue> | none]
	//     [<percentage> | <number> | none]
	//     [<percentage> | <number> | none]
	//     [ / [<alpha-value> | none] ]? )
	// <legacy-hsl-syntax> = hsl( <hue>, <percentage>, <percentage>, <alpha-value>? )
	// <legacy-hsla-syntax> = hsla( <hue>, <percentage>, <percentage>, <alpha-value>? )
	Hsl(
		T![Function],
		Hue,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
		Option<T![')']>,
	),
	Hsla(
		T![Function],
		Hue,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
		Option<T![')']>,
	),

	// https://drafts.csswg.org/css-color/#funcdef-hwb
	// hwb() = hwb(
	//  [<hue> | none]
	//  [<percentage> | <number> | none]
	//  [<percentage> | <number> | none]
	//  [ / [<alpha-value> | none] ]? )
	Hwb(T![Function], Hue, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-lab
	// lab() = lab( [<percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ / [<alpha-value> | none] ]? )
	Lab(T![Function], Channel, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-lch
	// lch() = lch( [<percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <hue> | none]
	//  [ / [<alpha-value> | none] ]? )
	Lch(T![Function], Channel, Channel, Hue, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-oklab
	// oklab() = oklab( [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ / [<alpha-value> | none] ]? )
	Oklab(T![Function], Channel, Channel, Channel, Option<T![/]>, Option<Channel>, Option<T![')']>),

	// https://drafts.csswg.org/css-color/#funcdef-oklch
	// oklch() = oklch( [ <percentage> | <number> | none]
	//  [ <percentage> | <number> | none]
	//  [ <hue> | none]
	//  [ / [<alpha-value> | none] ]? )
	Oklch(T![Function], Channel, Channel, Hue, Option<T![/]>, Option<Channel>, Option<T![')']>),
}

impl<'a> Peek<'a> for ColorFunction {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		ColorFunctionName::peek(p, c)
	}
}

impl<'a> ColorFunction {
	#[allow(clippy::type_complexity)] // TODO: simplify types
	fn parse_rgb(
		p: &mut Parser<'a>,
	) -> ParserResult<(
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
	)> {
		let a = p.parse::<Channel>()?;
		let b = p.parse_if_peek::<T![,]>()?;
		let c = p.parse::<Channel>()?;
		let d = p.parse_if_peek::<T![,]>()?;
		let e = p.parse::<Channel>()?;
		let f = p.parse_if_peek::<T![,]>()?;
		let g = p.parse_if_peek::<T![/]>()?;
		let h = p.parse_if_peek::<Channel>()?;
		Ok((a, b, c, d, e, f, g, h))
	}

	#[allow(clippy::type_complexity)] // TODO: simplify types
	fn parse_hsl(
		p: &mut Parser<'a>,
	) -> ParserResult<(
		Hue,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Channel,
		Option<T![,]>,
		Option<T![/]>,
		Option<Channel>,
	)> {
		let a = p.parse::<Hue>()?;
		let b = p.parse_if_peek::<T![,]>()?;
		let c = p.parse::<Channel>()?;
		let d = p.parse_if_peek::<T![,]>()?;
		let e = p.parse::<Channel>()?;
		let f = p.parse_if_peek::<T![,]>()?;
		let g = p.parse_if_peek::<T![/]>()?;
		let h = p.parse_if_peek::<Channel>()?;
		Ok((a, b, c, d, e, f, g, h))
	}

	#[allow(clippy::type_complexity)] // TODO: simplify types
	fn parse_hwb(p: &mut Parser<'a>) -> ParserResult<(Hue, Channel, Channel, Option<T![/]>, Option<Channel>)> {
		let a = p.parse::<Hue>()?;
		let b = p.parse::<Channel>()?;
		let c = p.parse::<Channel>()?;
		let d = p.parse_if_peek::<T![/]>()?;
		let e = p.parse_if_peek::<Channel>()?;
		Ok((a, b, c, d, e))
	}

	#[allow(clippy::type_complexity)] // TODO: simplify types
	fn parse_lch(p: &mut Parser<'a>) -> ParserResult<(Channel, Channel, Hue, Option<T![/]>, Option<Channel>)> {
		let a = p.parse::<Channel>()?;
		let b = p.parse::<Channel>()?;
		let c = p.parse::<Hue>()?;
		let d = p.parse_if_peek::<T![/]>()?;
		let e = p.parse_if_peek::<Channel>()?;
		Ok((a, b, c, d, e))
	}

	#[allow(clippy::type_complexity)] // TODO: simplify types
	fn parse_three_channel(
		p: &mut Parser<'a>,
	) -> ParserResult<(Channel, Channel, Channel, Option<T![/]>, Option<Channel>)> {
		let a = p.parse::<Channel>()?;
		let b = p.parse::<Channel>()?;
		let c = p.parse::<Channel>()?;
		let d = p.parse_if_peek::<T![/]>()?;
		let e = p.parse_if_peek::<Channel>()?;
		Ok((a, b, c, d, e))
	}
}

impl<'a> Parse<'a> for ColorFunction {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = ColorFunctionName::parse(p)?;
		match function {
			ColorFunctionName::Color(function) => {
				let space = p.parse::<ColorSpace>()?;
				let (a, b, c, d, e) = Self::parse_three_channel(p)?;
				Ok(Self::Color(function, space, a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Rgb(function) => {
				let (a, b, c, d, e, f, g, h) = Self::parse_rgb(p)?;
				Ok(Self::Rgb(function, a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Rgba(function) => {
				let (a, b, c, d, e, f, g, h) = Self::parse_rgb(p)?;
				Ok(Self::Rgba(function, a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Hsl(function) => {
				let (a, b, c, d, e, f, g, h) = Self::parse_hsl(p)?;
				Ok(Self::Hsl(function, a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Hsla(function) => {
				let (a, b, c, d, e, f, g, h) = Self::parse_hsl(p)?;
				Ok(Self::Hsla(function, a, b, c, d, e, f, g, h, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Hwb(function) => {
				let (a, b, c, d, e) = Self::parse_hwb(p)?;
				Ok(Self::Hwb(function, a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Lab(function) => {
				let (a, b, c, d, e) = Self::parse_three_channel(p)?;
				Ok(Self::Lab(function, a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Lch(function) => {
				let (a, b, c, d, e) = Self::parse_lch(p)?;
				Ok(Self::Lch(function, a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Oklab(function) => {
				let (a, b, c, d, e) = Self::parse_three_channel(p)?;
				Ok(Self::Oklab(function, a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
			ColorFunctionName::Oklch(function) => {
				let (a, b, c, d, e) = Self::parse_lch(p)?;
				Ok(Self::Oklch(function, a, b, c, d, e, p.parse_if_peek::<T![')']>()?))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Channel>(), 16);
		assert_eq!(std::mem::size_of::<ColorFunction>(), 160);
	}
}
