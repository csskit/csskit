use super::prelude::*;
use crate::Percentage;

/// <https://drafts.csswg.org/css-color-5/#color-mix>
///
/// ```text,ignore
/// color-mix() = color-mix( <color-interpolation-method> , [ <color> && <percentage [0,100]>? ]#{2} )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(all))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorMixFunction<'a> {
	#[atom(CssAtomSet::ColorMix)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub interpolation: ColorInterpolationMethod,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub comma: T![,],
	pub first: ColorMixPart<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub comma2: T![,],
	pub second: ColorMixPart<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-color-4/#color-interpolation-method>
///
/// ```text,ignore
/// <color-interpolation-method> = in [ <rectangular-color-space> | <polar-color-space> <hue-interpolation-method>? ]
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorInterpolationMethod {
	#[atom(CssAtomSet::In)]
	pub in_keyword: T![Ident],
	pub color_space: InterpolationColorSpace,
}

/// The color space for color interpolation, which can be rectangular or polar.
///
/// ```text,ignore
/// <rectangular-color-space> | <polar-color-space> <hue-interpolation-method>?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum InterpolationColorSpace {
	Rectangular(RectangularColorSpace),
	Polar(PolarColorSpace, Option<HueInterpolationMethod>),
}

/// <https://drafts.csswg.org/css-color-4/#typedef-rectangular-color-space>
///
/// ```text,ignore
/// <rectangular-color-space> = srgb | srgb-linear | display-p3 | a98-rgb |
///     prophoto-rgb | rec2020 | lab | oklab | xyz | xyz-d50 | xyz-d65
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RectangularColorSpace {
	#[atom(CssAtomSet::Srgb)]
	Srgb(T![Ident]),
	#[atom(CssAtomSet::SrgbLinear)]
	SrgbLinear(T![Ident]),
	#[atom(CssAtomSet::DisplayP3)]
	DisplayP3(T![Ident]),
	#[atom(CssAtomSet::A98Rgb)]
	A98Rgb(T![Ident]),
	#[atom(CssAtomSet::ProphotoRgb)]
	ProphotoRgb(T![Ident]),
	#[atom(CssAtomSet::Rec2020)]
	Rec2020(T![Ident]),
	#[atom(CssAtomSet::Lab)]
	Lab(T![Ident]),
	#[atom(CssAtomSet::Oklab)]
	Oklab(T![Ident]),
	#[atom(CssAtomSet::Xyz)]
	Xyz(T![Ident]),
	#[atom(CssAtomSet::XyzD50)]
	XyzD50(T![Ident]),
	#[atom(CssAtomSet::XyzD65)]
	XyzD65(T![Ident]),
}

/// <https://drafts.csswg.org/css-color-4/#typedef-polar-color-space>
///
/// ```text,ignore
/// <polar-color-space> = hsl | hwb | lch | oklch
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PolarColorSpace {
	#[atom(CssAtomSet::Hsl)]
	Hsl(T![Ident]),
	#[atom(CssAtomSet::Hwb)]
	Hwb(T![Ident]),
	#[atom(CssAtomSet::Lch)]
	Lch(T![Ident]),
	#[atom(CssAtomSet::Oklch)]
	Oklch(T![Ident]),
}

/// <https://drafts.csswg.org/css-color-4/#typedef-hue-interpolation-method>
///
/// ```text,ignore
/// <hue-interpolation-method> = [ shorter | longer | increasing | decreasing ] hue
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HueInterpolationMethod {
	pub direction: HueInterpolationDirection,
	#[atom(CssAtomSet::Hue)]
	pub hue_keyword: T![Ident],
}

/// The direction keyword for hue interpolation.
///
/// ```text,ignore
/// shorter | longer | increasing | decreasing
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum HueInterpolationDirection {
	#[atom(CssAtomSet::Shorter)]
	Shorter(T![Ident]),
	#[atom(CssAtomSet::Longer)]
	Longer(T![Ident]),
	#[atom(CssAtomSet::Increasing)]
	Increasing(T![Ident]),
	#[atom(CssAtomSet::Decreasing)]
	Decreasing(T![Ident]),
}

/// A color with an optional percentage in a color-mix() function.
///
/// ```text,ignore
/// [ <color> && <percentage [0,100]>? ]
/// ```
///
/// The color and percentage can appear in either order.
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorMixPart<'a> {
	pub color: Color<'a>,
	pub percentage: Option<Percentage>,
}

impl<'a> Peek<'a> for ColorMixPart<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Color::peek(p, c) || Percentage::peek(p, c)
	}
}

impl<'a> Parse<'a> for ColorMixPart<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Either order: <color> <percentage>? or <percentage> <color>
		let mut color = p.parse_if_peek::<Color>()?;
		let percentage = p.parse_if_peek::<Percentage>()?;
		if color.is_none() {
			color = Some(p.parse::<Color>()?);
		}
		Ok(Self { color: color.unwrap(), percentage })
	}
}

#[cfg(feature = "chromashift")]
impl HueInterpolationDirection {
	/// Converts this AST node to the corresponding chromashift hue interpolation direction.
	pub fn to_hue_interpolation(&self) -> chromashift::HueInterpolation {
		match self {
			Self::Shorter(_) => chromashift::HueInterpolation::Shorter,
			Self::Longer(_) => chromashift::HueInterpolation::Longer,
			Self::Increasing(_) => chromashift::HueInterpolation::Increasing,
			Self::Decreasing(_) => chromashift::HueInterpolation::Decreasing,
		}
	}
}

#[cfg(feature = "chromashift")]
impl InterpolationColorSpace {
	/// Mixes two colours in this interpolation colour space.
	///
	/// `percentage` is how much of the second colour to use (0.0 = all first, 100.0 = all second).
	pub fn mix(&self, first: chromashift::Color, second: chromashift::Color, percentage: f64) -> chromashift::Color {
		use chromashift::{
			A98Rgb, ColorMix, ColorMixPolar, DisplayP3, Hsl, Hwb, Lab, Lch, LinearRgb, Oklab, Oklch, ProphotoRgb,
			Rec2020, Srgb, XyzD50, XyzD65,
		};
		match self {
			Self::Rectangular(space) => match space {
				RectangularColorSpace::Srgb(_) => chromashift::Color::Srgb(Srgb::mix(first, second, percentage)),
				RectangularColorSpace::SrgbLinear(_) => {
					chromashift::Color::LinearRgb(LinearRgb::mix(first, second, percentage))
				}
				RectangularColorSpace::DisplayP3(_) => {
					chromashift::Color::DisplayP3(DisplayP3::mix(first, second, percentage))
				}
				RectangularColorSpace::A98Rgb(_) => chromashift::Color::A98Rgb(A98Rgb::mix(first, second, percentage)),
				RectangularColorSpace::ProphotoRgb(_) => {
					chromashift::Color::ProphotoRgb(ProphotoRgb::mix(first, second, percentage))
				}
				RectangularColorSpace::Rec2020(_) => {
					chromashift::Color::Rec2020(Rec2020::mix(first, second, percentage))
				}
				RectangularColorSpace::Lab(_) => chromashift::Color::Lab(Lab::mix(first, second, percentage)),
				RectangularColorSpace::Oklab(_) => chromashift::Color::Oklab(Oklab::mix(first, second, percentage)),
				RectangularColorSpace::XyzD50(_) => chromashift::Color::XyzD50(XyzD50::mix(first, second, percentage)),
				RectangularColorSpace::Xyz(_) | RectangularColorSpace::XyzD65(_) => {
					chromashift::Color::XyzD65(XyzD65::mix(first, second, percentage))
				}
			},
			Self::Polar(space, hue_method) => {
				let dir = match hue_method {
					None => chromashift::HueInterpolation::Shorter,
					Some(him) => him.direction.to_hue_interpolation(),
				};
				match space {
					PolarColorSpace::Hsl(_) => chromashift::Color::Hsl(Hsl::mix_polar(first, second, percentage, dir)),
					PolarColorSpace::Hwb(_) => chromashift::Color::Hwb(Hwb::mix_polar(first, second, percentage, dir)),
					PolarColorSpace::Lch(_) => chromashift::Color::Lch(Lch::mix_polar(first, second, percentage, dir)),
					PolarColorSpace::Oklch(_) => {
						chromashift::Color::Oklch(Oklch::mix_polar(first, second, percentage, dir))
					}
				}
			}
		}
	}
}

#[cfg(feature = "chromashift")]
impl crate::ToChromashift for ColorMixFunction<'_> {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		let first_color = self.first.color.to_chromashift()?;
		let second_color = self.second.color.to_chromashift()?;

		// Resolve percentages per the spec:
		// - If both omitted: 50% / 50%
		// - If one omitted: other = 100% - given
		// - If both given: use as-is (may need normalization if they don't sum to 100%)
		let p1 = self.first.percentage.as_ref().map(|p| p.value() as f64);
		let p2 = self.second.percentage.as_ref().map(|p| p.value() as f64);

		let (p1, p2) = match (p1, p2) {
			(None, None) => (50.0, 50.0),
			(Some(a), None) => (a, 100.0 - a),
			(None, Some(b)) => (100.0 - b, b),
			(Some(a), Some(b)) => (a, b),
		};

		// Normalize so that p1 + p2 = 100
		let sum = p1 + p2;
		if sum == 0.0 {
			return None;
		}
		let p1 = p1 / sum * 100.0;

		// The percentage for mixing is "how much of the second color"
		let mix_percentage = 100.0 - p1;

		Some(self.interpolation.color_space.mix(first_color, second_color, mix_percentage))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorMixFunction>(), 424);
		assert_eq!(std::mem::size_of::<ColorInterpolationMethod>(), 56);
		assert_eq!(std::mem::size_of::<InterpolationColorSpace>(), 44);
		assert_eq!(std::mem::size_of::<RectangularColorSpace>(), 16);
		assert_eq!(std::mem::size_of::<PolarColorSpace>(), 16);
		assert_eq!(std::mem::size_of::<HueInterpolationMethod>(), 28);
		assert_eq!(std::mem::size_of::<HueInterpolationDirection>(), 16);
		assert_eq!(std::mem::size_of::<ColorMixPart>(), 160);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,red 50%,blue 50%)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in oklch,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in oklch longer hue,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in hsl shorter hue,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in hsl increasing hue,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in hsl decreasing hue,red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in lab,rgb(255 0 0),rgb(0 0 255))");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,50% red,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,red 50%,blue)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in oklab,#fff 30%,#000 70%)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in xyz-d50,red,green)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in xyz-d65,red,green)");
		assert_parse!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb-linear,red,green)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		// Named colors
		assert_visits!("color-mix(in srgb, red, blue)", ColorMixFunction, ColorInterpolationMethod, Color, Color,);
		// Function colors recurse into ColorFunction and its variant
		assert_visits!(
			"color-mix(in srgb, rgb(255, 0, 0), blue)",
			ColorMixFunction,
			ColorInterpolationMethod,
			Color,
			ColorFunction,
			RgbFunction,
			Color,
		);
		// Percentages are visited
		assert_visits!(
			"color-mix(in srgb, red 50%, blue 50%)",
			ColorMixFunction,
			ColorInterpolationMethod,
			Color,
			Percentage,
			Color,
			Percentage,
		);
		// Polar color space with hue interpolation
		assert_visits!(
			"color-mix(in oklch shorter hue, red, blue)",
			ColorMixFunction,
			ColorInterpolationMethod,
			Color,
			Color,
		);
	}

	#[test]
	fn test_errors() {
		// Missing interpolation method
		assert_parse_error!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(red,blue)");
		// Missing "in" keyword
		assert_parse_error!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(srgb,red,blue)");
		// Missing second color
		assert_parse_error!(CssAtomSet::ATOMS, ColorMixFunction, "color-mix(in srgb,red)");
	}
}
