use super::prelude::*;
use crate::{AngleOrZero, CalcableValue, Color, Length, NonNegative, NumberOrPercentage, Url};

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
	Blur(BlurFunction<'a>),
	#[atom(CssAtomSet::Brightness)]
	Brightness(BrightnessFunction<'a>),
	#[atom(CssAtomSet::Contrast)]
	Contrast(ContrastFunction<'a>),
	#[atom(CssAtomSet::DropShadow)]
	DropShadow(DropShadowFunction<'a>),
	#[atom(CssAtomSet::Grayscale)]
	Grayscale(GrayscaleFunction<'a>),
	#[atom(CssAtomSet::HueRotate)]
	HueRotate(HueRotateFunction<'a>),
	#[atom(CssAtomSet::Invert)]
	Invert(InvertFunction<'a>),
	#[atom(CssAtomSet::Opacity)]
	Opacity(OpacityFunction<'a>),
	#[atom(CssAtomSet::Saturate)]
	Saturate(SaturateFunction<'a>),
	#[atom(CssAtomSet::Sepia)]
	Sepia(SepiaFunction<'a>),
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
pub struct BlurFunction<'a> {
	#[atom(CssAtomSet::Blur)]
	pub name: T![Function],
	pub radius: Option<CalcableValue<'a, NonNegative<Length>>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// `brightness( <number [0,∞]> | <percentage [0,∞]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct BrightnessFunction<'a> {
	#[atom(CssAtomSet::Brightness)]
	pub name: T![Function],
	pub value: Option<CalcableValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// `contrast( <number [0,∞]> | <percentage [0,∞]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ContrastFunction<'a> {
	#[atom(CssAtomSet::Contrast)]
	pub name: T![Function],
	pub value: Option<CalcableValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)]
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
	pub offset_x: CalcableValue<'a, Length>,
	pub offset_y: CalcableValue<'a, Length>,
	pub blur_radius: Option<CalcableValue<'a, NonNegative<Length>>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// `grayscale( <number [0,1]> | <percentage [0,100]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GrayscaleFunction<'a> {
	#[atom(CssAtomSet::Grayscale)]
	pub name: T![Function],
	pub value: Option<CalcableValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// `hue-rotate( <angle> | <zero> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HueRotateFunction<'a> {
	#[atom(CssAtomSet::HueRotate)]
	pub name: T![Function],
	pub angle: Option<CalcableValue<'a, AngleOrZero>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// `invert( <number [0,1]> | <percentage [0,100]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct InvertFunction<'a> {
	#[atom(CssAtomSet::Invert)]
	pub name: T![Function],
	pub value: Option<CalcableValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// `opacity( <number [0,1]> | <percentage [0,100]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OpacityFunction<'a> {
	#[atom(CssAtomSet::Opacity)]
	pub name: T![Function],
	pub value: Option<CalcableValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// `saturate( <number [0,∞]> | <percentage [0,∞]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SaturateFunction<'a> {
	#[atom(CssAtomSet::Saturate)]
	pub name: T![Function],
	pub value: Option<CalcableValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// `sepia( <number [0,1]> | <percentage [0,100]> )`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SepiaFunction<'a> {
	#[atom(CssAtomSet::Sepia)]
	pub name: T![Function],
	pub value: Option<CalcableValue<'a, NumberOrPercentage>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FilterFunction>(), 120);
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
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "drop-shadow(calc(2px) var(--y))");
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
		// Substitution/math in value/radius/angle slots.
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "blur(calc(5px))");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "brightness(var(--b))");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "hue-rotate(calc(90deg))");
		assert_parse!(CssAtomSet::ATOMS, FilterFunction, "opacity(var(--o))");
	}

	#[test]
	fn test_filter_function_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FilterFunction, "none");
		assert_peek_false!(CssAtomSet::ATOMS, FilterFunction, "foo()");
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
		assert_peek_false!(CssAtomSet::ATOMS, FilterValueList, "none");
	}
}
