#[cfg(feature = "serde")]
use serde::Serialize;

use smallvec::{smallvec, SmallVec};

use hdx_atom::{atom, Atom};
use hdx_lexer::Token;
use hdx_parser::{
	diagnostics, expect, expect_ident_ignore_case, match_ident_ignore_case, peek, unexpected, unexpected_ident, AtRule,
	FromToken, Parse, Parser, Result as ParserResult, RuleList, Spanned, Vec,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};

use crate::css::stylerule::StyleRule;

mod features;
use features::*;

// https://drafts.csswg.org/mediaqueries-4/
#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct MediaRule<'a> {
	pub query: Spanned<MediaQueryList>,
	pub style: Spanned<MediaDeclaration<'a>>,
}

// https://drafts.csswg.org/css-conditional-3/#at-ruledef-media
impl<'a> Parse<'a> for MediaRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		expect!(parser, Token::AtKeyword(atom!("media")));
		let span = parser.span();
		match Self::parse_at_rule(parser)? {
			(Some(query), Some(style)) => Ok(Self { query, style }),
			(Some(_), None) => Err(diagnostics::MissingAtRuleBlock(span.end(parser.pos())))?,
			(None, Some(_)) => Err(diagnostics::MissingAtRulePrelude(span.end(parser.pos())))?,
			(None, None) => Err(diagnostics::MissingAtRulePrelude(span.end(parser.pos())))?,
		}
	}
}

impl<'a> AtRule<'a> for MediaRule<'a> {
	type Prelude = MediaQueryList;
	type Block = MediaDeclaration<'a>;
}

impl<'a> WriteCss<'a> for MediaRule<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		sink.write_char('@')?;
		atom!("media").write_css(sink)?;
		sink.write_char(' ')?;
		self.query.write_css(sink)?;
		self.style.write_css(sink)?;
		Ok(())
	}
}

#[derive(PartialEq, Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct MediaDeclaration<'a> {
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, Spanned<StyleRule<'a>>>,
}

impl<'a> Parse<'a> for MediaDeclaration<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let rules = Self::parse_rule_list(parser)?;
		Ok(Self { rules })
	}
}

impl<'a> RuleList<'a> for MediaDeclaration<'a> {
	type Rule = StyleRule<'a>;
}

impl<'a> WriteCss<'a> for MediaDeclaration<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		todo!()
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct MediaQueryList(pub SmallVec<[Spanned<MediaQuery>; 1]>);

impl<'a> Parse<'a> for MediaQueryList {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut queries = smallvec![];
		loop {
			let query = MediaQuery::parse_spanned(parser)?;
			queries.push(query);
			if matches!(parser.cur(), Token::Comma) {
				parser.advance();
			} else {
				return Ok(Self(queries));
			}
		}
	}
}

impl<'a> WriteCss<'a> for MediaQueryList {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut queries = self.0.iter().peekable();
		while let Some(query) = queries.next() {
			query.write_css(sink)?;
			if queries.peek().is_some() {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			}
		}
		Ok(())
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum MediaQuery {
	Condition(MediaCondition),
	Typed(MediaType),
	NotTyped(MediaType),
	OnlyTyped(MediaType),
	TypedCondition(MediaType, MediaCondition),
	NotTypedCondition(MediaType, MediaCondition),
	OnlyTypedCondition(MediaType, MediaCondition),
}

impl<'a> Parse<'a> for MediaQuery {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut not = false;
		let mut only = false;
		let mut media_type = None;
		match parser.cur() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("not") => {
					parser.advance();
					not = true;
				}
				atom!("only") => {
					parser.advance();
					only = true;
				}
				_ => {
					if let Some(ty) = MediaType::from_token(parser.cur()) {
						parser.advance();
						media_type = Some(ty);
					} else {
						unexpected_ident!(parser, ident);
					}
				}
			},
			Token::LeftParen => {
				return Ok(Self::Condition(MediaCondition::parse(parser)?));
			}
			token => {
				unexpected!(parser, token);
			}
		}
		match parser.cur() {
			Token::Ident(ident) if only || not => {
				if let Some(ty) = MediaType::from_token(parser.cur()) {
					parser.advance();
					media_type = Some(ty);
				} else {
					unexpected_ident!(parser, ident)
				}
			}
			Token::Ident(ident) if media_type.is_some() && matches!(ident.to_ascii_lowercase(), atom!("and")) => {
				// Must not advance because we need "and" to be consumed by MediaCondition
				return Ok(Self::TypedCondition(media_type.unwrap(), MediaCondition::parse(parser)?));
			}
			token => {
				if let Some(mt) = media_type {
					return Ok(Self::Typed(mt));
				} else {
					unexpected!(parser, token)
				}
			}
		}
		match parser.cur() {
			Token::Ident(ident) if matches!(ident.to_ascii_lowercase(), atom!("and")) => {
				// Must not advance because we need "and" to be consumed by MediaCondition
				if only {
					Ok(Self::OnlyTypedCondition(media_type.unwrap(), MediaCondition::parse(parser)?))
				} else if not {
					Ok(Self::NotTypedCondition(media_type.unwrap(), MediaCondition::parse(parser)?))
				} else {
					unexpected_ident!(parser, ident)
				}
			}
			_ if only => Ok(Self::OnlyTyped(media_type.unwrap())),
			_ if not => Ok(Self::NotTyped(media_type.unwrap())),
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for MediaQuery {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Condition(mc) => mc.write_css(sink),
			Self::Typed(mt) => mt.write_css(sink),
			Self::NotTyped(mt) => {
				atom!("not").write_css(sink)?;
				sink.write_char(' ')?;
				mt.write_css(sink)
			}
			Self::OnlyTyped(mt) => {
				atom!("only").write_css(sink)?;
				sink.write_char(' ')?;
				mt.write_css(sink)
			}
			Self::TypedCondition(mt, mc) => {
				mt.write_css(sink)?;
				mc.write_css(sink)
			}
			Self::NotTypedCondition(mt, mc) => {
				atom!("not").write_css(sink)?;
				sink.write_char(' ')?;
				mt.write_css(sink)?;
				mc.write_css(sink)
			}
			Self::OnlyTypedCondition(mt, mc) => {
				atom!("only").write_css(sink)?;
				sink.write_char(' ')?;
				mt.write_css(sink)?;
				mc.write_css(sink)
			}
		}
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum MediaCondition {
	Is(MediaFeature),
	Not(MediaFeature),
	And(SmallVec<[MediaFeature; 4]>),
	Or(SmallVec<[MediaFeature; 4]>),
}

impl<'a> Parse<'a> for MediaCondition {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::LeftCurly => {
				if peek!(parser, Token::LeftCurly) {
					todo!()
				} else {
					Ok(Self::Is(MediaFeature::parse(parser)?))
				}
			}
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("and") => {
					let mut features = smallvec![];
                    dbg!("parsing media features and condition", parser.cur());
					loop {
						expect_ident_ignore_case!(parser, atom!("and"));
						parser.advance();
						features.push(MediaFeature::parse(parser)?);
						if !match_ident_ignore_case!(parser, atom!("and")) {
							return Ok(Self::And(features));
						}
					}
				}
				atom!("or") => {
					let mut features = smallvec![];
					loop {
						expect_ident_ignore_case!(parser, atom!("or"));
						parser.advance();
						features.push(MediaFeature::parse(parser)?);
						if !match_ident_ignore_case!(parser, atom!("or")) {
							return Ok(Self::And(features));
						}
					}
				}
				atom!("not") => Ok(Self::Not(MediaFeature::parse(parser)?)),
				_ => unexpected_ident!(parser, ident),
			},
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for MediaCondition {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Is(feature) => feature.write_css(sink),
			Self::Not(feature) => {
                sink.write_char(' ')?;
                atom!("not").write_css(sink)?;
                sink.write_char(' ')?;
				feature.write_css(sink)
			}
			Self::And(features) => {
				for feature in features.iter() {
					sink.write_char(' ')?;
					atom!("and").write_css(sink)?;
					sink.write_char(' ')?;
					feature.write_css(sink)?;
				}
				Ok(())
			}
			Self::Or(features) => {
				for feature in features.iter() {
                    sink.write_char(' ')?;
                    atom!("or").write_css(sink)?;
                    sink.write_char(' ')?;
					feature.write_css(sink)?;
				}
				Ok(())
			}
		}
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum MediaFeature {
	Height(HeightMediaFeature),
	Width(WidthMediaFeature),
	AspectRatio(AspectRatioMediaFeature),
	Orientation(OrientationMediaFeature),
	Resolution(ResolutionMediaFeature),
	Scan(ScanMediaFeature),
	Grid(GridMediaFeature),
	Update(UpdateMediaFeature),
	OverflowBlock(OverflowBlockMediaFeature),
	OverflowInline(OverflowInlineMediaFeature),
	Color(ColorMediaFeature),
	ColorIndex(ColorIndexMediaFeature),
	Monochrome(MonochromeMediaFeature),
	ColorGamut(ColorGamutMediaFeature),
	Pointer(PointerMediaFeature),
	Hover(HoverMediaFeature),
	AnyPointer(AnyPointerMediaFeature),
	AnyHover(AnyHoverMediaFeature),
	DeviceWidth(DeviceWidthMediaFeature),
	DeviceHeight(DeviceHeightMediaFeature),
	DeviceAspectRatio(DeviceAspectRatioMediaFeature),
}

impl<'a> Parse<'a> for MediaFeature {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
        dbg!("parsing media feature...", parser.cur(), parser.peek());
		match parser.peek().clone() {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("height") => Ok(Self::Height(HeightMediaFeature::parse(parser)?)),
				atom!("width") => Ok(Self::Width(WidthMediaFeature::parse(parser)?)),
				atom!("aspect-ratio") => Ok(Self::AspectRatio(AspectRatioMediaFeature::parse(parser)?)),
				atom!("orientation") => Ok(Self::Orientation(OrientationMediaFeature::parse(parser)?)),
				atom!("resolution") => Ok(Self::Resolution(ResolutionMediaFeature::parse(parser)?)),
				atom!("scan") => Ok(Self::Scan(ScanMediaFeature::parse(parser)?)),
				atom!("grid") => Ok(Self::Grid(GridMediaFeature::parse(parser)?)),
				atom!("update") => Ok(Self::Update(UpdateMediaFeature::parse(parser)?)),
				atom!("overflow-block") => Ok(Self::OverflowBlock(OverflowBlockMediaFeature::parse(parser)?)),
				atom!("overflow-inline") => Ok(Self::OverflowInline(OverflowInlineMediaFeature::parse(parser)?)),
				atom!("color") => Ok(Self::Color(ColorMediaFeature::parse(parser)?)),
				atom!("color-index") => Ok(Self::ColorIndex(ColorIndexMediaFeature::parse(parser)?)),
				atom!("monochrome") => Ok(Self::Monochrome(MonochromeMediaFeature::parse(parser)?)),
				atom!("color-gamut") => Ok(Self::ColorGamut(ColorGamutMediaFeature::parse(parser)?)),
				atom!("pointer") => Ok(Self::Pointer(PointerMediaFeature::parse(parser)?)),
				atom!("hover") => Ok(Self::Hover(HoverMediaFeature::parse(parser)?)),
				atom!("any-pointer") => Ok(Self::AnyPointer(AnyPointerMediaFeature::parse(parser)?)),
				atom!("any-hover") => Ok(Self::AnyHover(AnyHoverMediaFeature::parse(parser)?)),
				atom!("device-width") => Ok(Self::DeviceWidth(DeviceWidthMediaFeature::parse(parser)?)),
				atom!("device-height") => Ok(Self::DeviceHeight(DeviceHeightMediaFeature::parse(parser)?)),
				atom!("device-aspect-ratio") => {
					Ok(Self::DeviceAspectRatio(DeviceAspectRatioMediaFeature::parse(parser)?))
				}
				_ => unexpected_ident!(parser, ident),
			},
			token => unexpected!(parser, token),
		}
	}
}

impl<'a> WriteCss<'a> for MediaFeature {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Height(f) => f.write_css(sink),
			Self::Width(f) => f.write_css(sink),
			Self::AspectRatio(f) => f.write_css(sink),
			Self::Orientation(f) => f.write_css(sink),
			Self::Resolution(f) => f.write_css(sink),
			Self::Scan(f) => f.write_css(sink),
			Self::Grid(f) => f.write_css(sink),
			Self::Update(f) => f.write_css(sink),
			Self::OverflowBlock(f) => f.write_css(sink),
			Self::OverflowInline(f) => f.write_css(sink),
			Self::Color(f) => f.write_css(sink),
			Self::ColorIndex(f) => f.write_css(sink),
			Self::Monochrome(f) => f.write_css(sink),
			Self::ColorGamut(f) => f.write_css(sink),
			Self::Pointer(f) => f.write_css(sink),
			Self::Hover(f) => f.write_css(sink),
			Self::AnyPointer(f) => f.write_css(sink),
			Self::AnyHover(f) => f.write_css(sink),
			Self::DeviceWidth(f) => f.write_css(sink),
			Self::DeviceHeight(f) => f.write_css(sink),
			Self::DeviceAspectRatio(f) => f.write_css(sink),
		}
	}
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum MediaType {
	All,          // atom!("all")
	Print,        // atom!("print")
	Screen,       // atom!("screen")
	Custom(Atom), // atom!("tty")
}

impl FromToken for MediaType {
	fn from_token(token: Token) -> Option<Self> {
		match token {
			Token::Ident(ident) => match ident.to_ascii_lowercase() {
				atom!("all") => Some(Self::All),
				atom!("print") => Some(Self::Print),
				atom!("screen") => Some(Self::Screen),
				// https://drafts.csswg.org/mediaqueries/#mq-syntax
				// The <media-type> production does not include the keywords only, not, and, or, and layer.
				atom!("only") | atom!("not") | atom!("and") | atom!("or") | atom!("layer") => None,
				_ => Some(Self::Custom(ident)),
			},
			_ => None,
		}
	}
}

impl<'a> WriteCss<'a> for MediaType {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::All => atom!("all").write_css(sink),
			Self::Print => atom!("print").write_css(sink),
			Self::Screen => atom!("screen").write_css(sink),
			Self::Custom(atom) => atom.write_css(sink),
		}
	}
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<MediaRule>(), 128);
		assert_eq!(size_of::<MediaDeclaration>(), 32);
		assert_eq!(size_of::<MediaQueryList>(), 80);
		assert_eq!(size_of::<MediaQuery>(), 56);
		assert_eq!(size_of::<MediaCondition>(), 32);
		assert_eq!(size_of::<MediaType>(), 16);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<MediaQuery>(&allocator, "print", "print");
		test_write::<MediaQuery>(&allocator, "not embossed", "not embossed");
		test_write::<MediaQuery>(&allocator, "only screen", "only screen");
		test_write::<MediaQuery>(&allocator, "screen and (grid)", "screen and (grid)");
		test_write::<MediaQuery>(&allocator, "screen and (grid)", "screen and (grid)");
		// test_write::<MediaQuery>(&allocator, "screen and (orientation: landscape)", "screen and (orientation: landscape)");
		// test_write::<MediaRule>(&allocator, "@media print {}", "@media print {}");
		// test_write::<MediaRule>(&allocator, "@media (min-width: 1200px) {}", "@media (min-width: 1200px) {}");
		// @media only screen and (max-device-width: 800px), only screen and (device-width: 1024px) and (device-height: 600px), only screen and (width: 1280px) and (orientation: landscape), only screen and (device-width: 800px), only screen and (max-width: 767px) {
	}
}
