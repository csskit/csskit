use super::prelude::*;
use css_parse::PreludeList;

mod features;
pub use features::*;

// https://drafts.csswg.org/mediaqueries-4/
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.media"))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct MediaRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Media)]
	pub name: T![AtKeyword],
	pub prelude: MediaQueryList<'a>,
	pub block: MediaRuleBlock<'a>,
}

impl<'a> NodeWithMetadata<CssMetadata> for MediaRule<'a> {
	fn self_metadata(&self) -> CssMetadata {
		CssMetadata { used_at_rules: AtRuleId::Media, node_kinds: NodeKinds::AtRule, ..Default::default() }
	}

	fn metadata(&self) -> CssMetadata {
		self.block.0.metadata().merge(self.self_metadata())
	}
}

#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct MediaRuleBlock<'a>(pub Block<'a, StyleValue<'a>, Rule<'a>, CssMetadata>);

#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct MediaQueryList<'a>(pub Vec<'a, MediaQuery<'a>>);

impl<'a> PreludeList<'a> for MediaQueryList<'a> {
	type PreludeItem = MediaQuery<'a>;
}

impl<'a> Parse<'a> for MediaQueryList<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum MediaType {
	#[atom(CssAtomSet::All)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	All(T![Ident]),
	#[atom(CssAtomSet::Print)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	Print(T![Ident]),
	#[atom(CssAtomSet::Screen)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	Screen(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	Custom(T![Ident]),
}

impl MediaType {
	#[allow(dead_code)]
	fn invalid_ident(atom: &CssAtomSet) -> bool {
		matches!(atom, CssAtomSet::Only | CssAtomSet::Not | CssAtomSet::And | CssAtomSet::Or | CssAtomSet::Layer)
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum MediaPreCondition {
	#[atom(CssAtomSet::Not)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	Not(T![Ident]),
	#[atom(CssAtomSet::Only)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	Only(T![Ident]),
}

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct MediaQuery<'a> {
	precondition: Option<MediaPreCondition>,
	media_type: Option<MediaType>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	and: Option<T![Ident]>,
	condition: Option<MediaCondition<'a>>,
}

impl<'a> Peek<'a> for MediaQuery<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident, Kind::LeftParen]);
}

impl<'a> Parse<'a> for MediaQuery<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut precondition = None;
		let mut media_type = None;
		let mut and = None;
		let mut condition = None;
		if p.peek::<T!['(']>() {
			condition = Some(p.parse::<MediaCondition<'a>>()?);
			return Ok(Self { precondition, media_type, and, condition });
		}
		let c = p.peek_n(1);
		if MediaPreCondition::peek(p, c) {
			precondition = Some(p.parse::<MediaPreCondition>()?);
		} else if MediaType::peek(p, c) {
			media_type = Some(p.parse::<MediaType>()?);
		} else {
			Err(Diagnostic::new(c, Diagnostic::expected_ident))?
		}
		if p.peek::<T![Ident]>() && precondition.is_some() {
			let c: Cursor = p.peek_n(1);
			if MediaType::peek(p, c) {
				media_type = Some(p.parse::<MediaType>()?);
			} else {
				Err(Diagnostic::new(c, Diagnostic::expected_ident))?
			}
		}
		let c = p.peek_n(1);
		if c == Kind::Ident && p.equals_atom(c, &CssAtomSet::And) {
			and = Some(p.parse::<T![Ident]>()?);
			condition = Some(p.parse::<MediaCondition>()?);
		}
		Ok(Self { precondition, media_type, and, condition })
	}
}

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub enum MediaCondition<'a> {
	Is(MediaFeature),
	Not(T![Ident], MediaFeature),
	And(Vec<'a, (MediaFeature, Option<T![Ident]>)>),
	Or(Vec<'a, (MediaFeature, Option<T![Ident]>)>),
}

impl<'a> FeatureConditionList<'a> for MediaCondition<'a> {
	type FeatureCondition = MediaFeature;
	fn keyword_is_not<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Not)
	}
	fn keyword_is_and<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::And)
	}
	fn keyword_is_or<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.equals_atom(c, &CssAtomSet::Or)
	}
	fn build_is(feature: MediaFeature) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: T![Ident], feature: MediaFeature) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (MediaFeature, Option<T![Ident]>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (MediaFeature, Option<T![Ident]>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for MediaCondition<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_condition(p)
	}
}

macro_rules! media_feature {
	( $($name: ident($typ: ident): $pat: pat,)+) => {
		// https://drafts.csswg.org/mediaqueries-5/#media-descriptor-table
		#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
		pub enum MediaFeature {
			$($name($typ),)+
			#[cfg_attr(feature = "visitable", visit(skip))]
			Hack(HackMediaFeature),
		}
	}
}

apply_medias!(media_feature);

impl<'a> Parse<'a> for MediaFeature {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let checkpoint = p.checkpoint();
		let mut c = p.peek_n(2);
		macro_rules! match_media {
			( $($name: ident($typ: ident): $pat: pat,)+) => {
				// Only peek at the token as the underlying media feature parser needs to parse the leading ident.
				{
					match p.to_atom::<CssAtomSet>(c) {
						$($pat => $typ::parse(p).map(Self::$name),)+
						_ => Err(Diagnostic::new(c, Diagnostic::expected_ident))?
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
				c = p.next();
				Err(Diagnostic::new(c, Diagnostic::unexpected))?
			}
			apply_medias!(match_media)
		}
	}
}

macro_rules! apply_medias {
	($macro: ident) => {
		$macro! {
			// https://drafts.csswg.org/mediaqueries/#media-descriptor-table

			AnyHover(AnyHoverMediaFeature): CssAtomSet::AnyHover,
			AnyPointer(AnyPointerMediaFeature): CssAtomSet::AnyPointer,
			AspectRatio(AspectRatioMediaFeature): CssAtomSet::AspectRatio | CssAtomSet::MinAspectRatio | CssAtomSet::MaxAspectRatio,
			Color(ColorMediaFeature): CssAtomSet::Color | CssAtomSet::MaxColor | CssAtomSet::MinColor,
			ColorGamut(ColorGamutMediaFeature): CssAtomSet::ColorGamut,
			ColorIndex(ColorIndexMediaFeature): CssAtomSet::ColorIndex | CssAtomSet::MaxColorIndex | CssAtomSet::MinColorIndex,
			DeviceAspectRatio(DeviceAspectRatioMediaFeature): CssAtomSet::DeviceAspectRatio | CssAtomSet::MaxDeviceAspectRatio | CssAtomSet::MinDeviceAspectRatio,
			DeviceHeight(DeviceHeightMediaFeature): CssAtomSet::DeviceHeight | CssAtomSet::MaxDeviceHeight | CssAtomSet::MinDeviceHeight,
			DeviceWidth(DeviceWidthMediaFeature): CssAtomSet::DeviceWidth | CssAtomSet::MaxDeviceWidth | CssAtomSet::MinDeviceWidth,
			DisplayMode(DisplayModeMediaFeature): CssAtomSet::DisplayMode,
			DynamicRange(DynamicRangeMediaFeature): CssAtomSet::DynamicRange,
			EnvironmentBlending(EnvironmentBlendingMediaFeature): CssAtomSet::EnvironmentBlending,
			ForcedColors(ForcedColorsMediaFeature): CssAtomSet::ForcedColors,
			Grid(GridMediaFeature): CssAtomSet::Grid,
			Height(HeightMediaFeature): CssAtomSet::Height | CssAtomSet::MaxHeight | CssAtomSet::MinHeight,
			HorizontalViewportSegments(HorizontalViewportSegmentsMediaFeature): CssAtomSet::HorizontalViewportSegments | CssAtomSet::MaxHorizontalViewportSegments | CssAtomSet::MinHorizontalViewportSegments,
			Hover(HoverMediaFeature): CssAtomSet::Hover,
			InvertedColors(InvertedColorsMediaFeature): CssAtomSet::InvertedColors,
			Monochrome(MonochromeMediaFeature): CssAtomSet::Monochrome | CssAtomSet::MaxMonochrome | CssAtomSet::MinMonochrome,
			NavControls(NavControlsMediaFeature): CssAtomSet::NavControls,
			Orientation(OrientationMediaFeature): CssAtomSet::Orientation,
			OverflowBlock(OverflowBlockMediaFeature): CssAtomSet::OverflowBlock,
			OverflowInline(OverflowInlineMediaFeature): CssAtomSet::OverflowInline,
			Pointer(PointerMediaFeature): CssAtomSet::Pointer,
			PrefersColorScheme(PrefersColorSchemeMediaFeature): CssAtomSet::PrefersColorScheme,
			PrefersContrast(PrefersContrastMediaFeature): CssAtomSet::PrefersContrast,
			PrefersReducedData(PrefersReducedDataMediaFeature): CssAtomSet::PrefersReducedData,
			PrefersReducedMotion(PrefersReducedMotionMediaFeature): CssAtomSet::PrefersReducedMotion,
			PrefersReducedTransparency(PrefersReducedTransparencyMediaFeature): CssAtomSet::PrefersReducedTransparency,
			Resolution(ResolutionMediaFeature): CssAtomSet::Resolution | CssAtomSet::MaxResolution | CssAtomSet::MinResolution,
			Scan(ScanMediaFeature): CssAtomSet::Scan,
			Scripting(ScriptingMediaFeature): CssAtomSet::Scripting,
			Update(UpdateMediaFeature): CssAtomSet::Update,
			VerticalViewportSegments(VerticalViewportSegmentsMediaFeature): CssAtomSet::VerticalViewportSegments | CssAtomSet::MaxVerticalViewportSegments | CssAtomSet::MinVerticalViewportSegments,
			VideoColorGamut(VideoColorGamutMediaFeature): CssAtomSet::VideoColorGamut,
			VideoDynamicRange(VideoDynamicRangeMediaFeature): CssAtomSet::VideoDynamicRange,
			Width(WidthMediaFeature): CssAtomSet::Width | CssAtomSet::MaxWidth | CssAtomSet::MinWidth,

			// https://searchfox.org/wubkat/source/Source/WebCore/css/query/MediaQueryFeatures.cpp#192
			WebkitAnimationMediaFeature(WebkitAnimationMediaFeature): CssAtomSet::_WebkitAnimation,
			WebkitDevicePixelRatioMediaFeature(WebkitDevicePixelRatioMediaFeature): CssAtomSet::_WebkitDevicePixelRatio,
			WebkitTransform2dMediaFeature(WebkitTransform2dMediaFeature): CssAtomSet::_WebkitTransform2d,
			WebkitTransform3dMediaFeature(WebkitTransform3dMediaFeature): CssAtomSet::_WebkitTransform3d,
			WebkitTransitionMediaFeature(WebkitTransitionMediaFeature): CssAtomSet::_WebkitTransition,
			WebkitVideoPlayableInlineMediaFeature(WebkitVideoPlayableInlineMediaFeature): CssAtomSet::_WebkitVideoPlayableInline,

			// https://searchfox.org/mozilla-central/source/servo/components/style/gecko/media_features.rs#744
			MozDeviceOrientationMediaFeature(MozDeviceOrientationMediaFeature): CssAtomSet::_MozDeviceOrientation,
			MozDevicePixelRatioMediaFeature(MozDevicePixelRatioMediaFeature): CssAtomSet::_MozDevicePixelRatio | CssAtomSet::_MozMaxDevicePixelRatio | CssAtomSet::_MozMinDevicePixelRatio,
			MozMacGraphiteThemeMediaFeature(MozMacGraphiteThemeMediaFeature): CssAtomSet::_MozMacGraphiteTheme,
			MozMaemoClassicMediaFeature(MozMaemoClassicMediaFeature): CssAtomSet::_MozMaemoClassicTheme,
			MozImagesInMenusMediaFeature(MozImagesInMenusMediaFeature): CssAtomSet::_MozImagesInMenus,
			MozOsVersionMenusMediaFeature(MozOsVersionMediaFeature): CssAtomSet::_MozOsVersion,

			// https://github.com/search?q=%2F%5C(-ms-%5B%5E)%3A%5D%2B%5B)%3A%5D%2F%20language%3ACSS&type=code
			MsHighContrastMediaFeature(MsHighContrastMediaFeature): CssAtomSet::_MsHighContrast,
			MsViewStateMediaFeature(MsViewStateMediaFeature): CssAtomSet::_MsViewState,
			MsImeAlignMediaFeature(MsImeAlignMediaFeature): CssAtomSet::_MsImeAlign,
			MsDevicePixelRatioMediaFeature(MsDevicePixelRatioMediaFeature): CssAtomSet::_MsDevicePixelRatio,
			MsColumnCountMediaFeature(MsColumnCountMediaFeature): CssAtomSet::_MsColumnCount,

			// https://github.com/search?q=%2F%5C(-o-%5B%5E)%3A%5D%2B%5B)%3A%5D%2F%20language%3ACSS&type=code
			ODevicePixelRatioMediaFeature(ODevicePixelRatioMediaFeature): CssAtomSet::_ODevicePixelRatio,
		}
	};
}
use apply_medias;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<MediaRule>(), 168);
		assert_eq!(std::mem::size_of::<MediaQueryList>(), 32);
		assert_eq!(std::mem::size_of::<MediaQuery>(), 192);
		assert_eq!(std::mem::size_of::<MediaCondition>(), 144);
	}

	#[test]
	fn test_writes() {
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"print",
			MediaQuery { precondition: None, media_type: Some(MediaType::Print(_)), and: None, condition: None }
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"not embossed",
			MediaQuery {
				precondition: Some(MediaPreCondition::Not(_)),
				media_type: Some(MediaType::Custom(_)),
				and: None,
				condition: None
			}
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"only screen",
			MediaQuery {
				precondition: Some(MediaPreCondition::Only(_)),
				media_type: Some(MediaType::Screen(_)),
				and: None,
				condition: None
			}
		);
		assert_parse!(CssAtomSet::ATOMS, MediaFeature, "(grid)", MediaFeature::Grid(_));
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"screen and (grid)",
			MediaQuery {
				precondition: None,
				media_type: Some(MediaType::Screen(_)),
				and: Some(_),
				condition: Some(MediaCondition::Is(MediaFeature::Grid(_))),
			}
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"screen and (hover)and (pointer)",
			MediaQuery {
				precondition: None,
				media_type: Some(MediaType::Screen(_)),
				and: Some(_),
				condition: Some(MediaCondition::And(_))
			}
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			MediaQuery,
			"screen and (orientation:landscape)",
			MediaQuery {
				precondition: None,
				media_type: Some(MediaType::Screen(_)),
				and: Some(_),
				condition: Some(MediaCondition::Is(MediaFeature::Orientation(_))),
			}
		);
		assert_parse!(CssAtomSet::ATOMS, MediaQuery, "(hover)and (pointer)");
		assert_parse!(CssAtomSet::ATOMS, MediaQuery, "(hover)or (pointer)");
		// assert_parse!(CssAtomSet::ATOMS, MediaQuery, "not ((width: 2px) or (width: 3px))");
		// assert_parse!(CssAtomSet::ATOMS, MediaQuery, "not ((hover) or (pointer))");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media print{}");
		// assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media print,(prefers-reduced-motion: reduce){}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(min-width:1200px){}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(min-width:1200px){body{color:red;}}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(min-width:1200px){@page{}}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media screen{body{color:black}}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(max-width:575.98px)and (prefers-reduced-motion:reduce){}");
		// assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media only screen and(max-device-width:800px),only screen and (device-width:1024px) and (device-height: 600px),only screen and (width:1280px) and (orientation:landscape), only screen and (device-width:800px), only screen and (max-width:767px) {}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(grid){a{padding:4px}}");
		assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media(min-width:0){background:white}");
		// assert_parse!(
		// 	MediaRule,
		// 	"@media(grid){a{color-scheme:light}}",
		// 	"@media (grid: 0) {\n\ta {\n\t\tcolor-scheme: light;\n\t}\n}"
		// );

		// IE media hack
		// assert_parse!(CssAtomSet::ATOMS, MediaRule, "@media (min-width: 0\\0) {\n\n}");
	}

	// #[test]
	// fn test_errors() {
	// 	assert_parse_error!(CssAtomSet::ATOMS, MediaQuery, "(hover) and or (pointer)");
	// 	assert_parse_error!(CssAtomSet::ATOMS, MediaQuery, "(pointer) or and (pointer)");
	// 	assert_parse_error!(CssAtomSet::ATOMS, MediaQuery, "(pointer) not and (pointer)");
	// 	assert_parse_error!(CssAtomSet::ATOMS, MediaQuery, "only and (pointer)");
	// 	assert_parse_error!(CssAtomSet::ATOMS, MediaQuery, "not and (pointer)");
	// }

	#[test]
	#[cfg(feature = "visitable")]
	fn test_media_rule_visits() {
		use crate::assert_visits;
		assert_visits!(
			"@media (min-width: 768px) { body { color: red; } }",
			MediaRule,
			Length,
			StyleRule,
			SelectorList,
			CompoundSelector,
			Tag,
			HtmlTag,
			StyleValue,
			ColorStyleValue,
			Color
		);
		assert_visits!(
			"@media screen and (min-width: 768px) { body { color: red; } }",
			MediaRule,
			MediaType,
			Length,
			StyleRule,
			SelectorList,
			CompoundSelector,
			Tag,
			HtmlTag,
			StyleValue,
			ColorStyleValue,
			Color
		);
		assert_visits!(
			"@media only screen { body { color: red; } }",
			MediaRule,
			MediaPreCondition,
			MediaType,
			StyleRule,
			SelectorList,
			CompoundSelector,
			Tag,
			HtmlTag,
			StyleValue,
			ColorStyleValue,
			Color
		);
	}
}
