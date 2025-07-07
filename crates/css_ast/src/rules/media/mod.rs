use bumpalo::collections::Vec;
use css_lexer::{Cursor, Kind, Span};
use css_parse::{
	AtRule, Block, Build, ConditionKeyword, FeatureConditionList, Parse, Parser, Peek, PreludeList,
	Result as ParserResult, T, diagnostics, keyword_set,
};
use csskit_derives::ToCursors;

use crate::{Property, Visit, Visitable, stylesheet::Rule};

mod features;
use features::*;

// https://drafts.csswg.org/mediaqueries-4/
#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MediaRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub query: MediaQueryList<'a>,
	pub block: MediaRules<'a>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-media
impl<'a> Parse<'a> for MediaRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let start = p.offset();
		let (at_keyword, query, block) = Self::parse_at_rule(p)?;
		if let Some(query) = query {
			Ok(Self { at_keyword, query, block })
		} else {
			Err(diagnostics::MissingAtRulePrelude(Span::new(start, p.offset())))?
		}
	}
}

impl<'a> AtRule<'a> for MediaRule<'a> {
	const NAME: Option<&'static str> = Some("media");
	type Prelude = MediaQueryList<'a>;
	type Block = MediaRules<'a>;
}

impl<'a> Visitable<'a> for MediaRule<'a> {
	fn accept<V: Visit<'a>>(&self, _v: &mut V) {
		todo!();
	}
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MediaRules<'a> {
	pub open: T!['{'],
	pub properties: Vec<'a, (Property<'a>, Option<T![;]>)>,
	pub rules: Vec<'a, Rule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for MediaRules<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, rules, close) = Self::parse_block(p)?;
		Ok(Self { open, properties, rules, close })
	}
}

impl<'a> Block<'a> for MediaRules<'a> {
	type Declaration = Property<'a>;
	type Rule = Rule<'a>;
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MediaQueryList<'a>(pub Vec<'a, MediaQuery<'a>>);

impl<'a> PreludeList<'a> for MediaQueryList<'a> {
	type PreludeItem = MediaQuery<'a>;
}

impl<'a> Parse<'a> for MediaQueryList<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

keyword_set!(MediaPreCondition { Not: "not", Only: "only" });

#[derive(ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub enum MediaType {
	All(T![Ident]),
	Print(T![Ident]),
	Screen(T![Ident]),
	Custom(T![Ident]),
}

impl MediaType {
	const MAP: phf::Map<&'static str, MediaType> = phf::phf_map! {
			"all" => MediaType::All(<T![Ident]>::dummy()),
			"print" => MediaType::Print(<T![Ident]>::dummy()),
			"screen" => MediaType::Screen(<T![Ident]>::dummy()),
	};
	const INVALID: phf::Map<&'static str, bool> = phf::phf_map! {
		"only" => true,
		"not" => true,
		"and" => true,
		"or" => true,
		"layer" => true,
	};
}

impl<'a> Peek<'a> for MediaType {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c) && !(*Self::INVALID.get(p.parse_str_lower(c)).unwrap_or(&false))
	}
}

impl<'a> Build<'a> for MediaType {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		let str = &p.parse_str_lower(c);
		let media_type = Self::MAP.get(str);
		match media_type {
			Some(Self::All(_)) => Self::All(<T![Ident]>::build(p, c)),
			Some(Self::Print(_)) => Self::Print(<T![Ident]>::build(p, c)),
			Some(Self::Screen(_)) => Self::Screen(<T![Ident]>::build(p, c)),
			_ if *Self::INVALID.get(str).unwrap_or(&false) => unreachable!(),
			_ => Self::Custom(<T![Ident]>::build(p, c)),
		}
	}
}

impl From<MediaType> for Cursor {
	fn from(value: MediaType) -> Cursor {
		match value {
			MediaType::All(c) => c.into(),
			MediaType::Print(c) => c.into(),
			MediaType::Screen(c) => c.into(),
			MediaType::Custom(c) => c.into(),
		}
	}
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct MediaQuery<'a> {
	precondition: Option<MediaPreCondition>,
	media_type: Option<MediaType>,
	and: Option<T![Ident]>,
	condition: Option<MediaCondition<'a>>,
}

impl<'a> Parse<'a> for MediaQuery<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut precondition = None;
		let mut media_type = None;
		let mut and = None;
		let mut condition = None;
		if p.peek::<T!['(']>() {
			condition = Some(p.parse::<MediaCondition<'a>>()?);
			return Ok(Self { precondition, media_type, and, condition });
		}
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		if MediaPreCondition::peek(p, c) {
			precondition = Some(MediaPreCondition::build(p, c));
		} else if MediaType::peek(p, c) {
			media_type = Some(MediaType::build(p, c));
		} else {
			Err(diagnostics::UnexpectedIdent(p.parse_str(c).into(), c.into()))?
		}
		if p.peek::<T![Ident]>() && precondition.is_some() {
			let ident = p.parse::<T![Ident]>()?;
			let c: Cursor = ident.into();
			if MediaType::peek(p, c) {
				media_type = Some(MediaType::build(p, c));
			} else {
				Err(diagnostics::UnexpectedIdent(p.parse_str(c).into(), c.into()))?
			}
		}
		let c = p.peek_n(1);
		if c == Kind::Ident && p.eq_ignore_ascii_case(c, "and") {
			and = Some(p.parse::<T![Ident]>()?);
			condition = Some(p.parse::<MediaCondition>()?);
		}
		Ok(Self { precondition, media_type, and, condition })
	}
}

#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum MediaCondition<'a> {
	Is(MediaFeature),
	Not(ConditionKeyword, MediaFeature),
	And(Vec<'a, (MediaFeature, Option<ConditionKeyword>)>),
	Or(Vec<'a, (MediaFeature, Option<ConditionKeyword>)>),
}

impl<'a> FeatureConditionList<'a> for MediaCondition<'a> {
	type FeatureCondition = MediaFeature;
	fn build_is(feature: MediaFeature) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: ConditionKeyword, feature: MediaFeature) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (MediaFeature, Option<ConditionKeyword>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (MediaFeature, Option<ConditionKeyword>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for MediaCondition<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_condition(p)
	}
}

macro_rules! media_feature {
	( $($name: ident($typ: ident): $pat: pat,)+) => {
		// https://drafts.csswg.org/mediaqueries-5/#media-descriptor-table
		#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
		pub enum MediaFeature {
			$($name($typ),)+
			Hack(HackMediaFeature),
		}
	}
}

apply_medias!(media_feature);

impl<'a> Parse<'a> for MediaFeature {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		let mut c = p.peek_n(2);
		macro_rules! match_media {
			( $($name: ident($typ: ident): $pat: pat,)+) => {
				// Only peek at the token as the underlying media feature parser needs to parse the leading ident.
				{
					match p.parse_str_lower(c) {
						$($pat => $typ::parse(p).map(Self::$name),)+
						str => Err(diagnostics::UnexpectedIdent(str.into(), c.into()))?,
					}
				}
			}
		}
		if c == Kind::Ident {
			let value = apply_medias!(match_media).or_else(|err| {
				p.rewind(checkpoint);
				if let Ok(hack) = p.parse::<HackMediaFeature>() { Ok(Self::Hack(hack)) } else { Err(err) }
			})?;
			Ok(value)
		} else {
			// Styles like (1em < width < 1em) or (1em <= width <= 1em)
			c = p.peek_n(4);
			if c != Kind::Ident {
				c = p.peek_n(5)
			}
			if c != Kind::Ident {
				c = p.parse::<T![Any]>()?.into();
				Err(diagnostics::Unexpected(c.into(), c.into()))?
			}
			apply_medias!(match_media)
		}
	}
}

macro_rules! apply_medias {
	($macro: ident) => {
		$macro! {
			// https://drafts.csswg.org/mediaqueries/#media-descriptor-table

			AnyHover(AnyHoverMediaFeature): "any-hover",
			AnyPointer(AnyPointerMediaFeature): "any-pointer",
			AspectRatio(AspectRatioMediaFeature): "aspect-ratio" | "max-aspect-ratio" | "min-aspect-ratio",
			Color(ColorMediaFeature): "color" | "max-color" | "min-color",
			ColorGamut(ColorGamutMediaFeature): "color-gamut",
			ColorIndex(ColorIndexMediaFeature): "color-index" | "max-color-index" | "min-color-index",
			DeviceAspectRatio(DeviceAspectRatioMediaFeature): "device-aspect-ratio" | "max-device-aspect-ratio" | "min-device-aspect-ratio",
			DeviceHeight(DeviceHeightMediaFeature): "device-height" | "max-device-height" | "min-device-height",
			DeviceWidth(DeviceWidthMediaFeature): "device-width" | "max-device-width" | "min-device-width",
			DisplayMode(DisplayModeMediaFeature): "display-mode",
			DynamicRange(DynamicRangeMediaFeature): "dynamic-range",
			EnvironmentBlending(EnvironmentBlendingMediaFeature): "environment-blending",
			ForcedColors(ForcedColorsMediaFeature): "forced-colors",
			Grid(GridMediaFeature): "grid",
			Height(HeightMediaFeature): "height" | "max-height" | "min-height",
			HorizontalViewportSegments(HorizontalViewportSegmentsMediaFeature): "horizontal-viewport-segments" | "max-horizontal-viewport-segments" | "min-horizontal-viewport-segments",
			Hover(HoverMediaFeature): "hover",
			InvertedColors(InvertedColorsMediaFeature): "inverted-colors",
			Monochrome(MonochromeMediaFeature): "monochrome" | "max-monochrome" | "min-monochrome",
			NavControls(NavControlsMediaFeature): "nav-controls",
			Orientation(OrientationMediaFeature): "orientation",
			OverflowBlock(OverflowBlockMediaFeature): "overflow-block",
			OverflowInline(OverflowInlineMediaFeature): "overflow-inline",
			Pointer(PointerMediaFeature): "pointer",
			PrefersColorScheme(PrefersColorSchemeMediaFeature): "prefers-color-scheme",
			PrefersContrast(PrefersContrastMediaFeature): "prefers-contrast",
			PrefersReducedData(PrefersReducedDataMediaFeature): "prefers-reduced-data",
			PrefersReducedMotion(PrefersReducedMotionMediaFeature): "prefers-reduced-motion",
			PrefersReducedTransparency(PrefersReducedTransparencyMediaFeature): "prefers-reduced-transparency",
			Resolution(ResolutionMediaFeature): "resolution" | "max-resolution" | "min-resolution",
			Scan(ScanMediaFeature): "scan",
			Scripting(ScriptingMediaFeature): "scripting",
			Update(UpdateMediaFeature): "update",
			VerticalViewportSegments(VerticalViewportSegmentsMediaFeature): "vertical-viewport-segments" | "max-vertical-viewport-segments" | "min-vertical-viewport-segments",
			VideoColorGamut(VideoColorGamutMediaFeature): "video-color-gamut",
			VideoDynamicRange(VideoDynamicRangeMediaFeature): "video-dynamic-range",
			Width(WidthMediaFeature): "width" | "max-width" | "min-width",

			// https://searchfox.org/wubkat/source/Source/WebCore/css/query/MediaQueryFeatures.cpp#192
			WebkitAnimationMediaFeature(WebkitAnimationMediaFeature): "-webkit-animation",
			WebkitDevicePixelRatioMediaFeature(WebkitDevicePixelRatioMediaFeature): "-webkit-device-pixel-ratio",
			WebkitTransform2dMediaFeature(WebkitTransform2dMediaFeature): "-webkit-transform-2d",
			WebkitTransform3dMediaFeature(WebkitTransform3dMediaFeature): "-webkit-transform-3d",
			WebkitTransitionMediaFeature(WebkitTransitionMediaFeature): "-webkit-transition",
			WebkitVideoPlayableInlineMediaFeature(WebkitVideoPlayableInlineMediaFeature): "-webkit-video-playable-inline",

			// https://searchfox.org/mozilla-central/source/servo/components/style/gecko/media_features.rs#744
			MozDeviceOrientationMediaFeature(MozDeviceOrientationMediaFeature): "-moz-device-orientation",
			MozDevicePixelRatioMediaFeature(MozDevicePixelRatioMediaFeature): "-moz-device-pixel-ratio" | "max--moz-device-pixel-ratio" | "min--moz-device-pixel-ratio",
			MozMacGraphiteThemeMediaFeature(MozDevicePixelRatioMediaFeature): "-moz-mac-graphite-theme",
			MozMaemoClassicMediaFeature(MozDevicePixelRatioMediaFeature): "-moz-maemo-classic",
			MozImagesInMenusMediaFeature(MozDevicePixelRatioMediaFeature): "-moz-images-in-menus",
			MozOsVersionMenusMediaFeature(MozDevicePixelRatioMediaFeature): "-moz-os-version",

			// https://github.com/search?q=%2F%5C(-ms-%5B%5E)%3A%5D%2B%5B)%3A%5D%2F%20language%3ACSS&type=code
			MsHighContrastMediaFeature(MsHighContrastMediaFeature): "-ms-high-contrast",
			MsViewStateMediaFeature(MsViewStateMediaFeature): "-ms-view-state",
			MsImeAlignMediaFeature(MsImeAlignMediaFeature): "-ms-ime-align",
			MsDevicePixelRatioMediaFeature(MsDevicePixelRatioMediaFeature): "-ms-device-pixel-ratio",
			MsColumnCountMediaFeature(MsColumnCountMediaFeature): "-ms-column-count",

			// https://github.com/search?q=%2F%5C(-o-%5B%5E)%3A%5D%2B%5B)%3A%5D%2F%20language%3ACSS&type=code
			ODevicePixelRatioMediaFeature(ODevicePixelRatioMediaFeature): "-o-device-pixel-ratio",
		}
	};
}
use apply_medias;

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<MediaRule>(), 144);
		assert_eq!(std::mem::size_of::<MediaQueryList>(), 32);
		assert_eq!(std::mem::size_of::<MediaQuery>(), 200);
		assert_eq!(std::mem::size_of::<MediaCondition>(), 152);
	}

	#[test]
	fn test_writes() {
		assert_parse!(MediaQuery, "print");
		assert_parse!(MediaQuery, "not embossed");
		assert_parse!(MediaQuery, "only screen");
		assert_parse!(MediaFeature, "(grid)");
		assert_parse!(MediaQuery, "screen and (grid)");
		assert_parse!(MediaQuery, "screen and (hover)and (pointer)");
		assert_parse!(MediaQuery, "screen and (orientation:landscape)");
		assert_parse!(MediaQuery, "(hover)and (pointer)");
		assert_parse!(MediaQuery, "(hover)or (pointer)");
		// assert_parse!(MediaQuery, "not ((width: 2px) or (width: 3px))");
		// assert_parse!(MediaQuery, "not ((hover) or (pointer))");
		assert_parse!(MediaRule, "@media print{}");
		// assert_parse!(MediaRule, "@media print,(prefers-reduced-motion: reduce){}");
		assert_parse!(MediaRule, "@media(min-width:1200px){}");
		assert_parse!(MediaRule, "@media(min-width:1200px){body{color:red;}}");
		assert_parse!(MediaRule, "@media(min-width:1200px){@page{}}");
		assert_parse!(MediaRule, "@media(max-width:575.98px)and (prefers-reduced-motion:reduce){}");
		// assert_parse!(MediaRule, "@media only screen and(max-device-width:800px),only screen and (device-width:1024px) and (device-height: 600px),only screen and (width:1280px) and (orientation:landscape), only screen and (device-width:800px), only screen and (max-width:767px) {}");
		assert_parse!(MediaRule, "@media(grid){a{padding:4px}}");
		assert_parse!(MediaRule, "@media(min-width:0){background:white}");
		// assert_parse!(
		// 	MediaRule,
		// 	"@media(grid){a{color-scheme:light}}",
		// 	"@media (grid: 0) {\n\ta {\n\t\tcolor-scheme: light;\n\t}\n}"
		// );

		// IE media hack
		// assert_parse!(MediaRule, "@media (min-width: 0\\0) {\n\n}");
	}

	// #[test]
	// fn test_errors() {
	// 	assert_parse_error!(MediaQuery, "(hover) and or (pointer)");
	// 	assert_parse_error!(MediaQuery, "(pointer) or and (pointer)");
	// 	assert_parse_error!(MediaQuery, "(pointer) not and (pointer)");
	// 	assert_parse_error!(MediaQuery, "only and (pointer)");
	// 	assert_parse_error!(MediaQuery, "not and (pointer)");
	// }
}
