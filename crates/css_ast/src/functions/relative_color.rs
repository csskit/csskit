use super::prelude::*;
use crate::{AngleOrNumber, NoneOr, NumberOrPercentage};
use css_parse::BumpBox;

/// The `from <color>` clause in relative color syntax.
///
/// <https://drafts.csswg.org/css-color-5/#relative-colors>
///
/// ```text,ignore
/// from <color>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RelativeColorOrigin<'a> {
	#[atom(CssAtomSet::From)]
	pub from_keyword: T![Ident],
	pub color: Color<'a>,
}

/// Channel keyword for `rgb()` / `rgba()` relative color syntax.
///
/// Valid keywords: `r`, `g`, `b`, `alpha`
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RgbChannelKeyword {
	#[atom(CssAtomSet::R)]
	R(T![Ident]),
	#[atom(CssAtomSet::G)]
	G(T![Ident]),
	#[atom(CssAtomSet::B)]
	B(T![Ident]),
	#[atom(CssAtomSet::Alpha)]
	Alpha(T![Ident]),
}

/// Channel keyword for `hsl()` / `hsla()` relative color syntax.
///
/// Valid keywords: `h`, `s`, `l`, `alpha`
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum HslChannelKeyword {
	#[atom(CssAtomSet::H)]
	H(T![Ident]),
	#[atom(CssAtomSet::S)]
	S(T![Ident]),
	#[atom(CssAtomSet::L)]
	L(T![Ident]),
	#[atom(CssAtomSet::Alpha)]
	Alpha(T![Ident]),
}

/// Channel keyword for `hwb()` relative color syntax.
///
/// Valid keywords: `h`, `w`, `b`, `alpha`
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum HwbChannelKeyword {
	#[atom(CssAtomSet::H)]
	H(T![Ident]),
	#[atom(CssAtomSet::W)]
	W(T![Ident]),
	#[atom(CssAtomSet::B)]
	B(T![Ident]),
	#[atom(CssAtomSet::Alpha)]
	Alpha(T![Ident]),
}

/// Channel keyword for `lab()` / `oklab()` relative color syntax.
///
/// Valid keywords: `l`, `a`, `b`, `alpha`
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum LabChannelKeyword {
	#[atom(CssAtomSet::L)]
	L(T![Ident]),
	#[atom(CssAtomSet::A)]
	A(T![Ident]),
	#[atom(CssAtomSet::B)]
	B(T![Ident]),
	#[atom(CssAtomSet::Alpha)]
	Alpha(T![Ident]),
}

/// Channel keyword for `lch()` / `oklch()` relative color syntax.
///
/// Valid keywords: `l`, `c`, `h`, `alpha`
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum LchChannelKeyword {
	#[atom(CssAtomSet::L)]
	L(T![Ident]),
	#[atom(CssAtomSet::C)]
	C(T![Ident]),
	#[atom(CssAtomSet::H)]
	H(T![Ident]),
	#[atom(CssAtomSet::Alpha)]
	Alpha(T![Ident]),
}

/// Channel keyword for `color()` relative color syntax with xyz spaces.
///
/// Valid keywords: `x`, `y`, `z`, `alpha`
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum XyzChannelKeyword {
	#[atom(CssAtomSet::X)]
	X(T![Ident]),
	#[atom(CssAtomSet::Y)]
	Y(T![Ident]),
	#[atom(CssAtomSet::Z)]
	Z(T![Ident]),
	#[atom(CssAtomSet::Alpha)]
	Alpha(T![Ident]),
}

/// Channel keyword for `color()` relative color syntax - union of predefined-rgb and xyz keywords.
///
/// Valid keywords: `r`, `g`, `b`, `x`, `y`, `z`, `alpha`
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ColorChannelKeyword {
	#[atom(CssAtomSet::R)]
	R(T![Ident]),
	#[atom(CssAtomSet::G)]
	G(T![Ident]),
	#[atom(CssAtomSet::B)]
	B(T![Ident]),
	#[atom(CssAtomSet::X)]
	X(T![Ident]),
	#[atom(CssAtomSet::Y)]
	Y(T![Ident]),
	#[atom(CssAtomSet::Z)]
	Z(T![Ident]),
	#[atom(CssAtomSet::Alpha)]
	Alpha(T![Ident]),
}

/// A channel value in relative color syntax that can be a number, percentage,
/// `none`, a channel keyword, or a math function containing channel keywords.
///
/// For rgb(): `<number> | <percentage> | none | r | g | b | alpha | <calc()>`
/// For hsl(): `<hue> | <number> | <percentage> | none | h | s | l | alpha | <calc()>`
/// etc.
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RelativeChannelValue<K> {
	Keyword(K),
	Value(NoneOr<NumberOrPercentage>),
}

impl<'a, K: Peek<'a> + Parse<'a>> Peek<'a> for RelativeChannelValue<K> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		K::peek(p, c) || NoneOr::<NumberOrPercentage>::peek(p, c)
	}
}

impl<'a, K: Parse<'a> + Peek<'a>> Parse<'a> for RelativeChannelValue<K> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Try keyword first (channel keywords are single idents like r, g, b)
		if let Some(kw) = p.parse_if_peek::<K>()? {
			Ok(Self::Keyword(kw))
		} else {
			Ok(Self::Value(p.parse::<NoneOr<NumberOrPercentage>>()?))
		}
	}
}

/// A hue-position channel value in relative color syntax.
/// Accepts angle, number, `none`, or a channel keyword.
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RelativeHueValue<K> {
	Keyword(K),
	Value(NoneOr<AngleOrNumber>),
}

impl<'a, K: Peek<'a> + Parse<'a>> Peek<'a> for RelativeHueValue<K> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		K::peek(p, c) || NoneOr::<AngleOrNumber>::peek(p, c)
	}
}

impl<'a, K: Parse<'a> + Peek<'a>> Parse<'a> for RelativeHueValue<K> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if let Some(kw) = p.parse_if_peek::<K>()? {
			Ok(Self::Keyword(kw))
		} else {
			Ok(Self::Value(p.parse::<NoneOr<AngleOrNumber>>()?))
		}
	}
}

/// Relative color params for `rgb()` / `rgba()`.
///
/// ```text,ignore
/// from <color> <channel> <channel> <channel> [ / <alpha-channel> ]?
/// ```
///
/// Where each channel can be a number, percentage, `none`, or a channel keyword (r, g, b, alpha).
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RgbRelativeParams<'a> {
	pub origin: RelativeColorOrigin<'a>,
	pub red: RelativeChannelValue<RgbChannelKeyword>,
	pub green: RelativeChannelValue<RgbChannelKeyword>,
	pub blue: RelativeChannelValue<RgbChannelKeyword>,
	pub slash: Option<T![/]>,
	pub alpha: Option<RelativeChannelValue<RgbChannelKeyword>>,
}

/// Relative color params for `hsl()` / `hsla()`.
///
/// ```text,ignore
/// from <color> <hue-channel> <channel> <channel> [ / <alpha-channel> ]?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HslRelativeParams<'a> {
	pub origin: RelativeColorOrigin<'a>,
	pub hue: RelativeHueValue<HslChannelKeyword>,
	pub saturation: RelativeChannelValue<HslChannelKeyword>,
	pub lightness: RelativeChannelValue<HslChannelKeyword>,
	pub slash: Option<T![/]>,
	pub alpha: Option<RelativeChannelValue<HslChannelKeyword>>,
}

/// Relative color params for `hwb()`.
///
/// ```text,ignore
/// from <color> <hue-channel> <channel> <channel> [ / <alpha-channel> ]?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HwbRelativeParams<'a> {
	pub origin: RelativeColorOrigin<'a>,
	pub hue: RelativeHueValue<HwbChannelKeyword>,
	pub whiteness: RelativeChannelValue<HwbChannelKeyword>,
	pub blackness: RelativeChannelValue<HwbChannelKeyword>,
	pub slash: Option<T![/]>,
	pub alpha: Option<RelativeChannelValue<HwbChannelKeyword>>,
}

/// Relative color params for `lab()` / `oklab()`.
///
/// ```text,ignore
/// from <color> <channel> <channel> <channel> [ / <alpha-channel> ]?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LabRelativeParams<'a> {
	pub origin: RelativeColorOrigin<'a>,
	pub l: RelativeChannelValue<LabChannelKeyword>,
	pub a: RelativeChannelValue<LabChannelKeyword>,
	pub b: RelativeChannelValue<LabChannelKeyword>,
	pub slash: Option<T![/]>,
	pub alpha: Option<RelativeChannelValue<LabChannelKeyword>>,
}

/// Relative color params for `lch()` / `oklch()`.
///
/// ```text,ignore
/// from <color> <channel> <channel> <hue-channel> [ / <alpha-channel> ]?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LchRelativeParams<'a> {
	pub origin: RelativeColorOrigin<'a>,
	pub lightness: RelativeChannelValue<LchChannelKeyword>,
	pub chroma: RelativeChannelValue<LchChannelKeyword>,
	pub hue: RelativeHueValue<LchChannelKeyword>,
	pub slash: Option<T![/]>,
	pub alpha: Option<RelativeChannelValue<LchChannelKeyword>>,
}

/// Relative color params for `color()`.
///
/// ```text,ignore
/// from <color> <colorspace> <channel> <channel> <channel> [ / <alpha-channel> ]?
/// ```
///
/// Channel keywords are the union of predefined-rgb (`r`, `g`, `b`) and xyz (`x`, `y`, `z`)
/// plus `alpha`, since the valid set depends on the colorspace.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorRelativeParams<'a> {
	pub origin: RelativeColorOrigin<'a>,
	pub colorspace: super::color_function::ColorSpace,
	pub c1: RelativeChannelValue<ColorChannelKeyword>,
	pub c2: RelativeChannelValue<ColorChannelKeyword>,
	pub c3: RelativeChannelValue<ColorChannelKeyword>,
	pub slash: Option<T![/]>,
	pub alpha: Option<RelativeChannelValue<ColorChannelKeyword>>,
}

/// Relative colour syntax for `rgb()`.
///
/// ```text,ignore
/// rgb() = rgb( from <color> <channel> <channel> <channel> [ / <alpha-value> ]? )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RgbRelativeFunction<'a> {
	#[atom(CssAtomSet::Rgb)]
	pub name: T![Function],
	pub params: RgbRelativeParams<'a>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for RgbRelativeFunction<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Rgb)
			&& p.equals_atom(p.peek_n(2), &CssAtomSet::From)
	}
}

/// Relative colour syntax for `rgba()`.
///
/// ```text,ignore
/// rgba() = rgba( from <color> <channel> <channel> <channel> [ / <alpha-value> ]? )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RgbaRelativeFunction<'a> {
	#[atom(CssAtomSet::Rgba)]
	pub name: T![Function],
	pub params: RgbRelativeParams<'a>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for RgbaRelativeFunction<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Rgba)
			&& p.equals_atom(p.peek_n(2), &CssAtomSet::From)
	}
}

/// Relative colour syntax for `hsl()`.
///
/// ```text,ignore
/// hsl() = hsl( from <color> <hue-channel> <channel> <channel> [ / <alpha-value> ]? )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HslRelativeFunction<'a> {
	#[atom(CssAtomSet::Hsl)]
	pub name: T![Function],
	pub params: HslRelativeParams<'a>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for HslRelativeFunction<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Hsl)
			&& p.equals_atom(p.peek_n(2), &CssAtomSet::From)
	}
}

/// Relative colour syntax for `hsla()`.
///
/// ```text,ignore
/// hsla() = hsla( from <color> <hue-channel> <channel> <channel> [ / <alpha-value> ]? )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HslaRelativeFunction<'a> {
	#[atom(CssAtomSet::Hsla)]
	pub name: T![Function],
	pub params: HslRelativeParams<'a>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for HslaRelativeFunction<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Hsla)
			&& p.equals_atom(p.peek_n(2), &CssAtomSet::From)
	}
}

/// Relative colour syntax for `hwb()`.
///
/// ```text,ignore
/// hwb() = hwb( from <color> <hue-channel> <channel> <channel> [ / <alpha-value> ]? )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HwbRelativeFunction<'a> {
	#[atom(CssAtomSet::Hwb)]
	pub name: T![Function],
	pub params: HwbRelativeParams<'a>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for HwbRelativeFunction<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Hwb)
			&& p.equals_atom(p.peek_n(2), &CssAtomSet::From)
	}
}

/// Relative colour syntax for `lab()`.
///
/// ```text,ignore
/// lab() = lab( from <color> <channel> <channel> <channel> [ / <alpha-value> ]? )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LabRelativeFunction<'a> {
	#[atom(CssAtomSet::Lab)]
	pub name: T![Function],
	pub params: LabRelativeParams<'a>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for LabRelativeFunction<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Lab)
			&& p.equals_atom(p.peek_n(2), &CssAtomSet::From)
	}
}

/// Relative colour syntax for `lch()`.
///
/// ```text,ignore
/// lch() = lch( from <color> <channel> <channel> <hue-channel> [ / <alpha-value> ]? )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LchRelativeFunction<'a> {
	#[atom(CssAtomSet::Lch)]
	pub name: T![Function],
	pub params: LchRelativeParams<'a>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for LchRelativeFunction<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Lch)
			&& p.equals_atom(p.peek_n(2), &CssAtomSet::From)
	}
}

/// Relative colour syntax for `oklab()`.
///
/// ```text,ignore
/// oklab() = oklab( from <color> <channel> <channel> <channel> [ / <alpha-value> ]? )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OklabRelativeFunction<'a> {
	#[atom(CssAtomSet::Oklab)]
	pub name: T![Function],
	pub params: LabRelativeParams<'a>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for OklabRelativeFunction<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Oklab)
			&& p.equals_atom(p.peek_n(2), &CssAtomSet::From)
	}
}

/// Relative colour syntax for `oklch()`.
///
/// ```text,ignore
/// oklch() = oklch( from <color> <channel> <channel> <hue-channel> [ / <alpha-value> ]? )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OklchRelativeFunction<'a> {
	#[atom(CssAtomSet::Oklch)]
	pub name: T![Function],
	pub params: LchRelativeParams<'a>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for OklchRelativeFunction<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Oklch)
			&& p.equals_atom(p.peek_n(2), &CssAtomSet::From)
	}
}

/// Relative colour syntax for `color()`.
///
/// ```text,ignore
/// color() = color( from <color> <colorspace> <channel> <channel> <channel> [ / <alpha-value> ]? )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ColorRelativeFunction<'a> {
	#[atom(CssAtomSet::Color)]
	pub name: T![Function],
	pub params: ColorRelativeParams<'a>,
	pub close: T![')'],
}

impl<'a> Peek<'a> for ColorRelativeFunction<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Color)
			&& p.equals_atom(p.peek_n(2), &CssAtomSet::From)
	}
}

/// All colour functions in relative mode (`from <color>` origin).
///
/// Mirrors the structure of `ColorFunction` but only matches when `from` is present.
/// Relative variants are checked before their absolute counterparts in `ColorFunction`.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(all))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum RelativeColorFunction<'a> {
	Color(BumpBox<'a, ColorRelativeFunction<'a>>),
	Rgb(BumpBox<'a, RgbRelativeFunction<'a>>),
	Rgba(BumpBox<'a, RgbaRelativeFunction<'a>>),
	Hsl(BumpBox<'a, HslRelativeFunction<'a>>),
	Hsla(BumpBox<'a, HslaRelativeFunction<'a>>),
	Hwb(BumpBox<'a, HwbRelativeFunction<'a>>),
	Lab(BumpBox<'a, LabRelativeFunction<'a>>),
	Lch(BumpBox<'a, LchRelativeFunction<'a>>),
	Oklab(BumpBox<'a, OklabRelativeFunction<'a>>),
	Oklch(BumpBox<'a, OklchRelativeFunction<'a>>),
}

#[cfg(feature = "chromashift")]
mod chromashift_impl {
	use super::super::color_function::ColorSpace;
	use super::*;
	use crate::ToChromashift;
	use chromashift::{
		A98Rgb, Color, DisplayP3, Hsl, Hwb, Lab, Lch, LinearRgb, Oklab, Oklch, ProphotoRgb, Rec2020, Srgb, ToAlpha,
		XyzD50, XyzD65,
	};

	trait ChannelMap<K> {
		fn channel(&self, kw: &K) -> f32;
	}

	impl ChannelMap<RgbChannelKeyword> for Srgb {
		fn channel(&self, kw: &RgbChannelKeyword) -> f32 {
			match kw {
				RgbChannelKeyword::R(_) => self.red as f32,
				RgbChannelKeyword::G(_) => self.green as f32,
				RgbChannelKeyword::B(_) => self.blue as f32,
				RgbChannelKeyword::Alpha(_) => self.alpha,
			}
		}
	}

	impl ChannelMap<HslChannelKeyword> for Hsl {
		fn channel(&self, kw: &HslChannelKeyword) -> f32 {
			match kw {
				HslChannelKeyword::H(_) => self.hue,
				HslChannelKeyword::S(_) => self.saturation,
				HslChannelKeyword::L(_) => self.lightness,
				HslChannelKeyword::Alpha(_) => self.alpha,
			}
		}
	}

	impl ChannelMap<HwbChannelKeyword> for Hwb {
		fn channel(&self, kw: &HwbChannelKeyword) -> f32 {
			match kw {
				HwbChannelKeyword::H(_) => self.hue,
				HwbChannelKeyword::W(_) => self.whiteness,
				HwbChannelKeyword::B(_) => self.blackness,
				HwbChannelKeyword::Alpha(_) => self.alpha,
			}
		}
	}

	impl ChannelMap<LabChannelKeyword> for Lab {
		fn channel(&self, kw: &LabChannelKeyword) -> f32 {
			match kw {
				LabChannelKeyword::L(_) => self.lightness as f32,
				LabChannelKeyword::A(_) => self.a as f32,
				LabChannelKeyword::B(_) => self.b as f32,
				LabChannelKeyword::Alpha(_) => self.alpha,
			}
		}
	}

	impl ChannelMap<LabChannelKeyword> for Oklab {
		fn channel(&self, kw: &LabChannelKeyword) -> f32 {
			match kw {
				LabChannelKeyword::L(_) => self.lightness as f32,
				LabChannelKeyword::A(_) => self.a as f32,
				LabChannelKeyword::B(_) => self.b as f32,
				LabChannelKeyword::Alpha(_) => self.alpha,
			}
		}
	}

	impl ChannelMap<LchChannelKeyword> for Lch {
		fn channel(&self, kw: &LchChannelKeyword) -> f32 {
			match kw {
				LchChannelKeyword::L(_) => self.lightness as f32,
				LchChannelKeyword::C(_) => self.chroma as f32,
				LchChannelKeyword::H(_) => self.hue as f32,
				LchChannelKeyword::Alpha(_) => self.alpha,
			}
		}
	}

	impl ChannelMap<LchChannelKeyword> for Oklch {
		fn channel(&self, kw: &LchChannelKeyword) -> f32 {
			match kw {
				LchChannelKeyword::L(_) => self.lightness as f32,
				LchChannelKeyword::C(_) => self.chroma as f32,
				LchChannelKeyword::H(_) => self.hue as f32,
				LchChannelKeyword::Alpha(_) => self.alpha,
			}
		}
	}

	fn resolve_channel<K, O: ChannelMap<K>>(v: &RelativeChannelValue<K>, origin: &O, pct_scale: f32) -> Option<f32> {
		match v {
			RelativeChannelValue::Keyword(kw) => Some(origin.channel(kw)),
			RelativeChannelValue::Value(NoneOr::None(_)) => None,
			RelativeChannelValue::Value(NoneOr::Some(NumberOrPercentage::Number(n))) => Some(n.value()),
			RelativeChannelValue::Value(NoneOr::Some(NumberOrPercentage::Percentage(p))) => {
				Some(p.value() / 100.0 * pct_scale)
			}
		}
	}

	fn resolve_alpha<K, O: ChannelMap<K>>(v: &RelativeChannelValue<K>, origin: &O) -> Option<f32> {
		match v {
			RelativeChannelValue::Keyword(kw) => Some(origin.channel(kw)),
			RelativeChannelValue::Value(NoneOr::None(_)) => None,
			RelativeChannelValue::Value(NoneOr::Some(NumberOrPercentage::Number(n))) => Some(n.value() * 100.0),
			RelativeChannelValue::Value(NoneOr::Some(NumberOrPercentage::Percentage(p))) => Some(p.value()),
		}
	}

	fn resolve_hue_channel<K, O: ChannelMap<K>>(v: &RelativeHueValue<K>, origin: &O) -> Option<f32> {
		match v {
			RelativeHueValue::Keyword(kw) => Some(origin.channel(kw)),
			RelativeHueValue::Value(NoneOr::None(_)) => None,
			RelativeHueValue::Value(NoneOr::Some(AngleOrNumber::Number(n))) => Some(n.value()),
			RelativeHueValue::Value(NoneOr::Some(AngleOrNumber::Angle(a))) => Some(a.as_degrees()),
		}
	}

	impl crate::ToChromashift for RelativeColorFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			match self {
				Self::Rgb(f) => f.to_chromashift(),
				Self::Rgba(f) => f.to_chromashift(),
				Self::Hsl(f) => f.to_chromashift(),
				Self::Hsla(f) => f.to_chromashift(),
				Self::Hwb(f) => f.to_chromashift(),
				Self::Lab(f) => f.to_chromashift(),
				Self::Lch(f) => f.to_chromashift(),
				Self::Oklab(f) => f.to_chromashift(),
				Self::Oklch(f) => f.to_chromashift(),
				Self::Color(f) => f.to_chromashift(),
			}
		}
	}

	fn rgb_relative_params_to_chromashift(params: &RgbRelativeParams<'_>) -> Option<Color> {
		let origin = params.origin.color.to_chromashift()?;
		let srgb = Srgb::from(origin);
		let red = resolve_channel(&params.red, &srgb, 255.0)?.round() as u8;
		let green = resolve_channel(&params.green, &srgb, 255.0)?.round() as u8;
		let blue = resolve_channel(&params.blue, &srgb, 255.0)?.round() as u8;
		let alpha = params.alpha.as_ref().map_or(Some(100.0), |a| resolve_alpha(a, &srgb))?;
		Some(Color::Srgb(Srgb::new(red, green, blue, alpha)))
	}

	impl ToChromashift for RgbRelativeFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			rgb_relative_params_to_chromashift(&self.params)
		}
	}

	impl ToChromashift for RgbaRelativeFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			rgb_relative_params_to_chromashift(&self.params)
		}
	}

	fn hsl_relative_params_to_chromashift(params: &HslRelativeParams<'_>) -> Option<Color> {
		let origin = params.origin.color.to_chromashift()?;
		let hsl = Hsl::from(origin);
		let hue = resolve_hue_channel(&params.hue, &hsl)?;
		let saturation = resolve_channel(&params.saturation, &hsl, 100.0)?;
		let lightness = resolve_channel(&params.lightness, &hsl, 100.0)?;
		let alpha = params.alpha.as_ref().map_or(Some(100.0), |a| resolve_alpha(a, &hsl))?;
		Some(Color::Hsl(Hsl::new(hue, saturation, lightness, alpha)))
	}

	impl ToChromashift for HslRelativeFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			hsl_relative_params_to_chromashift(&self.params)
		}
	}

	impl ToChromashift for HslaRelativeFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			hsl_relative_params_to_chromashift(&self.params)
		}
	}

	impl ToChromashift for HwbRelativeFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			let origin = self.params.origin.color.to_chromashift()?;
			let hwb = Hwb::from(origin);
			let hue = resolve_hue_channel(&self.params.hue, &hwb)?;
			let whiteness = resolve_channel(&self.params.whiteness, &hwb, 100.0)?;
			let blackness = resolve_channel(&self.params.blackness, &hwb, 100.0)?;
			let alpha = self.params.alpha.as_ref().map_or(Some(100.0), |a| resolve_alpha(a, &hwb))?;
			Some(Color::Hwb(Hwb::new(hue, whiteness, blackness, alpha)))
		}
	}

	impl ToChromashift for LabRelativeFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			let origin = self.params.origin.color.to_chromashift()?;
			let lab = Lab::from(origin);
			let l = resolve_channel(&self.params.l, &lab, 100.0)? as f64;
			let a = resolve_channel(&self.params.a, &lab, 125.0)? as f64;
			let b = resolve_channel(&self.params.b, &lab, 125.0)? as f64;
			let alpha = self.params.alpha.as_ref().map_or(Some(lab.alpha), |ch| resolve_alpha(ch, &lab))?;
			Some(Color::Lab(Lab::new(l, a, b, alpha)))
		}
	}

	impl ToChromashift for LchRelativeFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			let origin = self.params.origin.color.to_chromashift()?;
			let lch = Lch::from(origin);
			let lightness = resolve_channel(&self.params.lightness, &lch, 100.0)? as f64;
			let chroma = resolve_channel(&self.params.chroma, &lch, 150.0)? as f64;
			let hue = resolve_hue_channel(&self.params.hue, &lch)? as f64;
			let alpha = self.params.alpha.as_ref().map_or(Some(lch.alpha), |ch| resolve_alpha(ch, &lch))?;
			Some(Color::Lch(Lch::new(lightness, chroma, hue, alpha)))
		}
	}

	impl ToChromashift for OklabRelativeFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			let origin = self.params.origin.color.to_chromashift()?;
			let oklab = Oklab::from(origin);
			let l = resolve_channel(&self.params.l, &oklab, 1.0)? as f64;
			let a = resolve_channel(&self.params.a, &oklab, 0.4)? as f64;
			let b = resolve_channel(&self.params.b, &oklab, 0.4)? as f64;
			let alpha = self.params.alpha.as_ref().map_or(Some(oklab.alpha), |ch| resolve_alpha(ch, &oklab))?;
			Some(Color::Oklab(Oklab::new(l, a, b, alpha)))
		}
	}

	impl ToChromashift for OklchRelativeFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			let origin = self.params.origin.color.to_chromashift()?;
			let oklch = Oklch::from(origin);
			let lightness = resolve_channel(&self.params.lightness, &oklch, 1.0)? as f64;
			let chroma = resolve_channel(&self.params.chroma, &oklch, 0.4)? as f64;
			let hue = resolve_hue_channel(&self.params.hue, &oklch)? as f64;
			let alpha = self.params.alpha.as_ref().map_or(Some(oklch.alpha), |ch| resolve_alpha(ch, &oklch))?;
			Some(Color::Oklch(Oklch::new(lightness, chroma, hue, alpha)))
		}
	}

	impl ToChromashift for ColorRelativeFunction<'_> {
		fn to_chromashift(&self) -> Option<Color> {
			let origin = self.params.origin.color.to_chromashift()?;
			let space = &self.params.colorspace;

			let unit_from_kw = |kw: &ColorChannelKeyword| -> f64 {
				match kw {
					ColorChannelKeyword::R(_) => Srgb::from(origin).red as f64 / 255.0,
					ColorChannelKeyword::G(_) => Srgb::from(origin).green as f64 / 255.0,
					ColorChannelKeyword::B(_) => Srgb::from(origin).blue as f64 / 255.0,
					ColorChannelKeyword::X(_) => XyzD65::from(origin).x / 100.0,
					ColorChannelKeyword::Y(_) => XyzD65::from(origin).y / 100.0,
					ColorChannelKeyword::Z(_) => XyzD65::from(origin).z / 100.0,
					ColorChannelKeyword::Alpha(_) => origin.to_alpha() as f64 / 100.0,
				}
			};

			let resolve_c = |v: &RelativeChannelValue<ColorChannelKeyword>| -> Option<f64> {
				match v {
					RelativeChannelValue::Keyword(kw) => Some(unit_from_kw(kw)),
					RelativeChannelValue::Value(NoneOr::None(_)) => None,
					RelativeChannelValue::Value(NoneOr::Some(NumberOrPercentage::Number(n))) => Some(n.value() as f64),
					RelativeChannelValue::Value(NoneOr::Some(NumberOrPercentage::Percentage(p))) => {
						Some(p.value() as f64 / 100.0)
					}
				}
			};

			let resolve_a = |v: &RelativeChannelValue<ColorChannelKeyword>| -> Option<f32> {
				match v {
					RelativeChannelValue::Keyword(kw) => Some((unit_from_kw(kw) * 100.0) as f32),
					RelativeChannelValue::Value(NoneOr::None(_)) => None,
					RelativeChannelValue::Value(NoneOr::Some(NumberOrPercentage::Number(n))) => Some(n.value() * 100.0),
					RelativeChannelValue::Value(NoneOr::Some(NumberOrPercentage::Percentage(p))) => Some(p.value()),
				}
			};

			let c1 = resolve_c(&self.params.c1)?;
			let c2 = resolve_c(&self.params.c2)?;
			let c3 = resolve_c(&self.params.c3)?;
			let alpha = self.params.alpha.as_ref().map_or(Some(100.0), resolve_a)?;

			match space {
				ColorSpace::Srgb(_) => Some(Color::Srgb(Srgb::new(
					(c1 * 255.0).round() as u8,
					(c2 * 255.0).round() as u8,
					(c3 * 255.0).round() as u8,
					alpha,
				))),
				ColorSpace::SrgbLinear(_) => Some(Color::LinearRgb(LinearRgb::new(c1, c2, c3, alpha))),
				ColorSpace::DisplayP3(_) => Some(Color::DisplayP3(DisplayP3::new(c1, c2, c3, alpha))),
				ColorSpace::A98Rgb(_) => Some(Color::A98Rgb(A98Rgb::new(c1, c2, c3, alpha))),
				ColorSpace::ProphotoRgb(_) => Some(Color::ProphotoRgb(ProphotoRgb::new(c1, c2, c3, alpha))),
				ColorSpace::Rec2020(_) => Some(Color::Rec2020(Rec2020::new(c1, c2, c3, alpha))),
				ColorSpace::Xyz(_) | ColorSpace::XyzD65(_) => {
					Some(Color::XyzD65(XyzD65::new(c1 * 100.0, c2 * 100.0, c3 * 100.0, alpha)))
				}
				ColorSpace::XyzD50(_) => Some(Color::XyzD50(XyzD50::new(c1 * 100.0, c2 * 100.0, c3 * 100.0, alpha))),
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<RelativeColorOrigin>(), 40);
		assert_eq!(std::mem::size_of::<RgbChannelKeyword>(), 16);
		assert_eq!(std::mem::size_of::<HslChannelKeyword>(), 16);
		assert_eq!(std::mem::size_of::<HwbChannelKeyword>(), 16);
		assert_eq!(std::mem::size_of::<LabChannelKeyword>(), 16);
		assert_eq!(std::mem::size_of::<LchChannelKeyword>(), 16);
		assert_eq!(std::mem::size_of::<XyzChannelKeyword>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, RgbChannelKeyword, "r");
		assert_parse!(CssAtomSet::ATOMS, RgbChannelKeyword, "alpha");
		assert_parse!(CssAtomSet::ATOMS, RelativeColorOrigin, "from red");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, RgbChannelKeyword, "h");
		assert_parse_error!(CssAtomSet::ATOMS, HslChannelKeyword, "r");
	}
}
