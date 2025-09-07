use crate::{AngleOrNumber, NoneOr, NumberOrPercentage};
use css_parse::{Build, Cursor, Function, Parser, Peek, T, function_set, keyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

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

function_set!(pub struct ColorFunctionName "color");
function_set!(pub struct RgbFunctionName "rgb");
function_set!(pub struct RgbaFunctionName "rgba");
function_set!(pub struct HslFunctionName "hsl");
function_set!(pub struct HslaFunctionName "hsla");
function_set!(pub struct HwbFunctionName "hwb");
function_set!(pub struct LabFunctionName "lab");
function_set!(pub struct LchFunctionName "lch");
function_set!(pub struct OklabFunctionName "oklab");
function_set!(pub struct OklchFunctionName "oklch");

#[derive(ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct CommaOrSlash(T![Delim]);

impl<'a> Peek<'a> for CommaOrSlash {
	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		c == ',' || c == '/'
	}
}

impl<'a> Build<'a> for CommaOrSlash {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		debug_assert!(Self::peek(p, c));
		Self(<T![Delim]>::build(p, c))
	}
}

/// <https://drafts.csswg.org/css-color/#typedef-color-function>
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum ColorFunction {
	Color(ColorFunctionColor),
	Rgb(RgbFunction),
	Rgba(RgbaFunction),
	Hsl(HslFunction),
	Hsla(HslaFunction),
	Hwb(HwbFunction),
	Lab(LabFunction),
	Lch(LchFunction),
	Oklab(OklabFunction),
	Oklch(OklchFunction),
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for ColorFunction {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		match self {
			Self::Color(c) => c.to_chromashift(),
			Self::Rgb(c) => c.to_chromashift(),
			Self::Rgba(c) => c.to_chromashift(),
			Self::Hsl(c) => c.to_chromashift(),
			Self::Hsla(c) => c.to_chromashift(),
			Self::Hwb(c) => c.to_chromashift(),
			Self::Lab(c) => c.to_chromashift(),
			Self::Lch(c) => c.to_chromashift(),
			Self::Oklab(c) => c.to_chromashift(),
			Self::Oklch(c) => c.to_chromashift(),
		}
	}
}

/// <https://drafts.csswg.org/css-color/#funcdef-color>
///
/// ```text,ignore
/// color() = color( <colorspace-params> [ / [ <alpha-value> | none ] ]? )
/// <colorspace-params> = [ <predefined-rgb-params> | <xyz-params>]
/// <predefined-rgb-params> = <predefined-rgb> [ <number> | <percentage> | none ]{3}
/// <predefined-rgb> = srgb | srgb-linear | display-p3 | a98-rgb | prophoto-rgb | rec2020
/// <xyz-params> = <xyz-space> [ <number> | <percentage> | none ]{3}
/// <xyz-space> = xyz | xyz-d50 | xyz-d65
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ColorFunctionColor(Function<ColorFunctionName, ColorFunctionColorParams>);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for ColorFunctionColor {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		todo!();
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ColorFunctionColorParams(
	pub ColorSpace,
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![/]>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-rgb>
///
/// ```text,ignore
/// rgb() = [ <legacy-rgb-syntax> | <modern-rgb-syntax> ]
/// rgba() = [ <legacy-rgba-syntax> | <modern-rgba-syntax> ]
/// <legacy-rgb-syntax> =   rgb( <percentage>#{3} , <alpha-value>? ) |
///                   rgb( <number>#{3} , <alpha-value>? )
/// <legacy-rgba-syntax> = rgba( <percentage>#{3} , <alpha-value>? ) |
///                   rgba( <number>#{3} , <alpha-value>? )
/// <modern-rgb-syntax> = rgb(
///   [ <number> | <percentage> | none]{3}
///   [ / [<alpha-value> | none] ]?  )
/// <modern-rgba-syntax> = rgba(
///   [ <number> | <percentage> | none]{3}
///   [ / [<alpha-value> | none] ]?  )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RgbFunction(Function<RgbFunctionName, RgbFunctionParams>);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for RgbFunction {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		self.0.parameters.to_chromashift()
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RgbaFunction(Function<RgbaFunctionName, RgbFunctionParams>);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for RgbaFunction {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		self.0.parameters.to_chromashift()
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct RgbFunctionParams(
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![,]>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![,]>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<CommaOrSlash>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for RgbFunctionParams {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Srgb;
		let Self(red, _, green, _, blue, _, alpha) = &self;
		let alpha = match alpha {
			Some(NoneOr::None(_)) => 0.0,
			Some(NoneOr::Some(NumberOrPercentage::Number(t))) => t.value() * 100.0,
			Some(NoneOr::Some(NumberOrPercentage::Percentage(t))) => t.value(),
			None => 100.0,
		};
		let red = match red {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(red)) => red.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(red)) => red.value() / 100.0 * 255.0,
		} as u8;
		let green = match green {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(green)) => green.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(green)) => green.value() / 100.0 * 255.0,
		} as u8;
		let blue = match blue {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(blue)) => blue.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(blue)) => blue.value() / 100.0 * 255.0,
		} as u8;
		Some(chromashift::Color::Srgb(Srgb::new(red, green, blue, alpha)))
	}
}

/// <https://drafts.csswg.org/css-color/#funcdef-hsl>
///
/// ```text,ignore
/// hsl() = [ <legacy-hsl-syntax> | <modern-hsl-syntax> ]
/// hsla() = [ <legacy-hsla-syntax> | <modern-hsla-syntax> ]
/// <modern-hsl-syntax> = hsl(
///     [<hue> | none]
///     [<percentage> | <number> | none]
///     [<percentage> | <number> | none]
///     [ / [<alpha-value> | none] ]? )
/// <modern-hsla-syntax> = hsla(
///     [<hue> | none]
///     [<percentage> | <number> | none]
///     [<percentage> | <number> | none]
///     [ / [<alpha-value> | none] ]? )
/// <legacy-hsl-syntax> = hsl( <hue>, <percentage>, <percentage>, <alpha-value>? )
/// <legacy-hsla-syntax> = hsla( <hue>, <percentage>, <percentage>, <alpha-value>? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HslFunction(Function<HslFunctionName, HslFunctionParams>);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for HslFunction {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		self.0.parameters.to_chromashift()
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HslaFunction(Function<HslaFunctionName, HslFunctionParams>);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for HslaFunction {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		self.0.parameters.to_chromashift()
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HslFunctionParams(
	pub NoneOr<AngleOrNumber>,
	pub Option<T![,]>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![,]>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<CommaOrSlash>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for HslFunctionParams {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Hsl;
		let Self(hue, _, saturation, _, lightness, _, alpha) = &self;
		let hue = match hue {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(AngleOrNumber::Number(hue)) => hue.value(),
			NoneOr::Some(AngleOrNumber::Angle(d)) => d.as_degrees(),
		};
		let saturation = match saturation {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value(),
		};
		let lightness = match lightness {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value(),
		};
		let alpha = match alpha {
			Some(NoneOr::None(_)) => 0.0,
			Some(NoneOr::Some(NumberOrPercentage::Number(t))) => t.value() * 100.0,
			Some(NoneOr::Some(NumberOrPercentage::Percentage(t))) => t.value(),
			None => 100.0,
		};
		Some(chromashift::Color::Hsl(Hsl::new(hue, saturation, lightness, alpha)))
	}
}

// https://drafts.csswg.org/css-color/#funcdef-hwb
// hwb() = hwb(
//  [<hue> | none]
//  [<percentage> | <number> | none]
//  [<percentage> | <number> | none]
//  [ / [<alpha-value> | none] ]? )
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HwbFunction(Function<HwbFunctionName, HwbFunctionParams>);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for HwbFunction {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Hwb;
		let HwbFunctionParams(hue, whiteness, blackness, _, alpha) = &self.0.parameters;
		let hue = match hue {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(AngleOrNumber::Number(hue)) => hue.value(),
			NoneOr::Some(AngleOrNumber::Angle(d)) => d.as_degrees(),
		};
		let whiteness = match whiteness {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value(),
		};
		let blackness = match blackness {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value(),
		};
		let alpha = match alpha {
			Some(NoneOr::None(_)) => 0.0,
			Some(NoneOr::Some(NumberOrPercentage::Number(t))) => t.value() * 100.0,
			Some(NoneOr::Some(NumberOrPercentage::Percentage(t))) => t.value(),
			None => 100.0,
		};
		Some(chromashift::Color::Hwb(Hwb::new(hue, whiteness, blackness, alpha)))
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct HwbFunctionParams(
	pub NoneOr<AngleOrNumber>,
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![/]>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-lab>
///
/// ```text,ignore
/// lab() = lab( [<percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ / [<alpha-value> | none] ]? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LabFunction(Function<LabFunctionName, LabFunctionParams>);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for LabFunction {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Lab;
		let LabFunctionParams(l, a, b, _, alpha) = &self.0.parameters;
		let l = match l {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value(),
		} as f64;
		let a = match a {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value() / 100.0 * 125.0,
		} as f64;
		let b = match b {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value() / 100.0 * 125.0,
		} as f64;
		let alpha = match alpha {
			Some(NoneOr::None(_)) => 0.0,
			Some(NoneOr::Some(NumberOrPercentage::Number(t))) => t.value() * 100.0,
			Some(NoneOr::Some(NumberOrPercentage::Percentage(t))) => t.value(),
			None => 100.0,
		};
		Some(chromashift::Color::Lab(Lab::new(l, a, b, alpha)))
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LabFunctionParams(
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub Option<T![/]>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-lch>
///
/// ```text,ignore
/// lch() = lch( [<percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <hue> | none]
///  [ / [<alpha-value> | none] ]? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LchFunction(Function<LchFunctionName, LchFunctionParams>);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for LchFunction {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Lch;
		let LchFunctionParams(lightness, chroma, hue, _, alpha) = &self.0.parameters;
		let lightness = match lightness {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value(),
		} as f64;
		let chroma = match chroma {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value() / 100.0 * 150.0,
		} as f64;
		let hue = match hue {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(AngleOrNumber::Number(hue)) => hue.value(),
			NoneOr::Some(AngleOrNumber::Angle(d)) => d.as_degrees(),
		} as f64;
		let alpha = match alpha {
			Some(NoneOr::None(_)) => 0.0,
			Some(NoneOr::Some(NumberOrPercentage::Number(t))) => t.value() * 100.0,
			Some(NoneOr::Some(NumberOrPercentage::Percentage(t))) => t.value(),
			None => 100.0,
		};
		Some(chromashift::Color::Lch(Lch::new(lightness, chroma, hue, alpha)))
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LchFunctionParams(
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<NumberOrPercentage>,
	pub NoneOr<AngleOrNumber>,
	pub Option<T![/]>,
	pub Option<NoneOr<NumberOrPercentage>>,
);

/// <https://drafts.csswg.org/css-color/#funcdef-oklab>
///
/// ```text,ignore
/// oklab() = oklab( [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ / [<alpha-value> | none] ]? )
///  ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct OklabFunction(Function<OklabFunctionName, LabFunctionParams>);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for OklabFunction {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Oklab;
		let LabFunctionParams(l, a, b, _, alpha) = &self.0.parameters;
		let alpha = match alpha {
			Some(NoneOr::None(_)) => 0.0,
			Some(NoneOr::Some(NumberOrPercentage::Number(t))) => t.value() * 100.0,
			Some(NoneOr::Some(NumberOrPercentage::Percentage(t))) => t.value(),
			None => 100.0,
		};
		let l = match l {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value() / 100.0,
		} as f64;
		let a = match a {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value() / 100.0 * 0.4,
		} as f64;
		let b = match b {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value() / 100.0 * 0.4,
		} as f64;
		Some(chromashift::Color::Oklab(Oklab::new(l, a, b, alpha)))
	}
}

/// <https://drafts.csswg.org/css-color/#funcdef-oklch>
///
/// ```text,ignore
/// oklab() = oklab( [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ <percentage> | <number> | none]
///  [ / [<alpha-value> | none] ]? )
///  ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct OklchFunction(Function<OklchFunctionName, LchFunctionParams>);

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for OklchFunction {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Oklch;
		let LchFunctionParams(lightness, chroma, hue, _, alpha) = &self.0.parameters;
		let lightness = match lightness {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value(),
		} as f64;
		let chroma = match chroma {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(NumberOrPercentage::Number(n)) => n.value(),
			NoneOr::Some(NumberOrPercentage::Percentage(p)) => p.value() / 100.0 * 150.0,
		} as f64;
		let hue = match hue {
			NoneOr::None(_) => {
				return None;
			}
			NoneOr::Some(AngleOrNumber::Number(hue)) => hue.value(),
			NoneOr::Some(AngleOrNumber::Angle(d)) => d.as_degrees(),
		} as f64;
		let alpha = match alpha {
			Some(NoneOr::None(_)) => 0.0,
			Some(NoneOr::Some(NumberOrPercentage::Number(t))) => t.value() * 100.0,
			Some(NoneOr::Some(NumberOrPercentage::Percentage(t))) => t.value(),
			None => 100.0,
		};
		Some(chromashift::Color::Oklch(Oklch::new(lightness, chroma, hue, alpha)))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorFunction>(), 144);
		assert_eq!(std::mem::size_of::<ColorFunctionColor>(), 124);
		assert_eq!(std::mem::size_of::<RgbFunction>(), 140);
		assert_eq!(std::mem::size_of::<RgbaFunction>(), 140);
		assert_eq!(std::mem::size_of::<HslFunction>(), 140);
		assert_eq!(std::mem::size_of::<HslaFunction>(), 140);
		assert_eq!(std::mem::size_of::<HwbFunction>(), 108);
		assert_eq!(std::mem::size_of::<LabFunction>(), 108);
		assert_eq!(std::mem::size_of::<LchFunction>(), 108);
		assert_eq!(std::mem::size_of::<OklabFunction>(), 108);
		assert_eq!(std::mem::size_of::<OklchFunction>(), 108);
	}
}
