use super::prelude::*;
use crate::{AngleOrZero, Color, Length, NonNegative, NumberOrPercentage, Url};

/// <https://drafts.csswg.org/filter-effects-1/#typedef-filter-function>
///
/// ```text,ignore
/// <filter-function> = <blur()> | <brightness()> | <contrast()> | <drop-shadow()>
///                   | <grayscale()> | <hue-rotate()> | <invert()> | <opacity()>
///                   | <saturate()> | <sepia()>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FilterFunction<'a> {
	#[atom(CssAtomSet::Blur)]
	Blur(BlurFunction),
	#[atom(CssAtomSet::Brightness)]
	Brightness(BrightnessFunction),
	#[atom(CssAtomSet::Contrast)]
	Contrast(ContrastFunction),
	#[atom(CssAtomSet::DropShadow)]
	DropShadow(DropShadowFunction<'a>),
	#[atom(CssAtomSet::Grayscale)]
	Grayscale(GrayscaleFunction),
	#[atom(CssAtomSet::HueRotate)]
	HueRotate(HueRotateFunction),
	#[atom(CssAtomSet::Invert)]
	Invert(InvertFunction),
	#[atom(CssAtomSet::Opacity)]
	Opacity(OpacityFunction),
	#[atom(CssAtomSet::Saturate)]
	Saturate(SaturateFunction),
	#[atom(CssAtomSet::Sepia)]
	Sepia(SepiaFunction),
}

/// <https://drafts.csswg.org/filter-effects-1/#typedef-filter-value-list>
///
/// ```text,ignore
/// <filter-value-list> = [ <filter-function> | <url> ]+
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FilterValueList<'a>(pub Vec<'a, FilterValue<'a>>);

/// A single item in a `<filter-value-list>`: either a filter function or a URL.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FilterValue<'a> {
	FilterFunction(FilterFunction<'a>),
	Url(Url),
}

/// `blur( <length [0,∞]>? )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct BlurFunction {
	#[atom(CssAtomSet::Blur)]
	pub name: T![Function],
	pub radius: Option<NonNegative<Length>>,
	pub close: T![')'],
}

/// `brightness( <number [0,∞]> | <percentage [0,∞]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct BrightnessFunction {
	#[atom(CssAtomSet::Brightness)]
	pub name: T![Function],
	pub value: Option<NumberOrPercentage>,
	pub close: T![')'],
}

/// `contrast( <number [0,∞]> | <percentage [0,∞]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ContrastFunction {
	#[atom(CssAtomSet::Contrast)]
	pub name: T![Function],
	pub value: Option<NumberOrPercentage>,
	pub close: T![')'],
}

/// `drop-shadow( <color>? && <length>{2,3} )`
///
/// Note: we parse color first (before offsets) as a simplification of the `&&` grammar.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct DropShadowFunction<'a> {
	#[atom(CssAtomSet::DropShadow)]
	pub name: T![Function],
	pub color: Option<Color<'a>>,
	pub offset_x: Length,
	pub offset_y: Length,
	pub blur_radius: Option<NonNegative<Length>>,
	pub close: T![')'],
}

/// `grayscale( <number [0,1]> | <percentage [0,100]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GrayscaleFunction {
	#[atom(CssAtomSet::Grayscale)]
	pub name: T![Function],
	pub value: Option<NumberOrPercentage>,
	pub close: T![')'],
}

/// `hue-rotate( <angle> | <zero> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HueRotateFunction {
	#[atom(CssAtomSet::HueRotate)]
	pub name: T![Function],
	pub angle: Option<AngleOrZero>,
	pub close: T![')'],
}

/// `invert( <number [0,1]> | <percentage [0,100]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct InvertFunction {
	#[atom(CssAtomSet::Invert)]
	pub name: T![Function],
	pub value: Option<NumberOrPercentage>,
	pub close: T![')'],
}

/// `opacity( <number [0,1]> | <percentage [0,100]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OpacityFunction {
	#[atom(CssAtomSet::Opacity)]
	pub name: T![Function],
	pub value: Option<NumberOrPercentage>,
	pub close: T![')'],
}

/// `saturate( <number [0,∞]> | <percentage [0,∞]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SaturateFunction {
	#[atom(CssAtomSet::Saturate)]
	pub name: T![Function],
	pub value: Option<NumberOrPercentage>,
	pub close: T![')'],
}

/// `sepia( <number [0,1]> | <percentage [0,100]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SepiaFunction {
	#[atom(CssAtomSet::Sepia)]
	pub name: T![Function],
	pub value: Option<NumberOrPercentage>,
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FilterFunction>(), 96);
	}

	#[test]
	fn test_filter_function_parses() {
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "blur()");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "blur(5px)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "brightness(0.5)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "brightness(50%)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "contrast(2)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "contrast(200%)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "drop-shadow(2px 4px)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "drop-shadow(2px 4px 3px)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "drop-shadow(red 2px 4px)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "drop-shadow(red 2px 4px 5px)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "grayscale(1)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "grayscale(100%)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "hue-rotate(90deg)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "hue-rotate(0)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "invert(0.5)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "invert(50%)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "opacity(0.5)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "opacity(50%)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "saturate(2)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "saturate(200%)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "sepia(0.5)");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "sepia(50%)");
	}

	#[test]
	fn test_filter_function_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FilterFunction, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FilterFunction, "foo()");
		assert_parse_error!(CssAtomSet::ATOMS, FilterFunction, "blur(-5px)");
	}

	#[test]
	fn test_filter_value_list_parses() {
		assert_parse!(CssAtomSet::ATOMS, FilterValueList, "blur(5px)");
		assert_parse!(CssAtomSet::ATOMS, FilterValueList, "blur(5px)brightness(0.5)");
		assert_parse!(CssAtomSet::ATOMS, FilterValueList, "blur(5px)contrast(200%)grayscale(0.5)");
		assert_parse!(CssAtomSet::ATOMS, FilterValueList, "url(\"filter.svg\")");
	}

	#[test]
	fn test_filter_value_list_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FilterValueList, "none");
	}
}
