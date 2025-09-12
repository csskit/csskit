use crate::{StyleValue, diagnostics};
use css_parse::{
	AtRule, CommaSeparated, Cursor, NoBlockAllowed, Parse, Parser, Peek, QualifiedRule, Result as ParserResult,
	RuleList, T, ToSpan, atkeyword_set, keyword_set,
};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, ToSpan, Visitable};

atkeyword_set!(pub struct AtKeyframesKeyword "keyframes");

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.keyframes"))]
#[visit]
pub struct KeyframesRule<'a>(pub AtRule<AtKeyframesKeyword, KeyframesName, KeyframesRuleBlock<'a>>);

#[derive(Peek, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum KeyframesName {
	Ident(T![Ident]),
	String(T![String]),
}

impl KeyframesName {
	const INVALID: phf::Map<&'static str, bool> = phf::phf_map! {
		"default" => true,
		"initial" => true,
		"unset" => true,
		"none" => true,
	};

	fn valid_ident(str: &str) -> bool {
		!*Self::INVALID.get(str).unwrap_or(&false)
	}
}

// Must use Parse rather than Build so ReservedKeyframeName errors can be emitted
impl<'a> Parse<'a> for KeyframesName {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![String]>() {
			return Ok(Self::String(p.parse::<T![String]>()?));
		}
		let ident = p.parse::<T![Ident]>()?;
		let str = p.parse_str_lower(ident.into());
		if !KeyframesName::valid_ident(str) {
			Err(diagnostics::ReservedKeyframeName(str.into(), ident.to_span()))?
		}
		Ok(Self::Ident(ident))
	}
}

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct KeyframesRuleBlock<'a>(RuleList<'a, Keyframe<'a>>);

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct Keyframe<'a>(QualifiedRule<'a, KeyframeSelectors<'a>, StyleValue<'a>, NoBlockAllowed>);

#[derive(Peek, Parse, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub struct KeyframeSelectors<'a>(pub CommaSeparated<'a, KeyframeSelector>);

#[derive(ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum KeyframeSelector {
	From(T![Ident]),
	To(T![Ident]),
	Percent(T![Dimension::%]),
}

keyword_set!(pub enum KeyframeSelectorKeyword { From: "from", To: "to" });

impl<'a> Peek<'a> for KeyframeSelector {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		KeyframeSelectorKeyword::peek(p, c) || <T![Dimension::%]>::peek(p, c)
	}
}

impl<'a> Parse<'a> for KeyframeSelector {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(keyword) = p.parse_if_peek::<KeyframeSelectorKeyword>()? {
			return match keyword {
				KeyframeSelectorKeyword::From(ident) => Ok(Self::From(ident)),
				KeyframeSelectorKeyword::To(ident) => Ok(Self::To(ident)),
			};
		}
		let percent = p.parse::<T![Dimension::%]>()?;
		let c: Cursor = percent.into();
		let f: f32 = c.token().value();
		if (0.0..=100.0).contains(&f) {
			Ok(Self::Percent(percent))
		} else {
			Err(diagnostics::NumberOutOfBounds(f, format!("{:?}", 0.0..=100.0), c.into()))?
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<KeyframesRule>(), 112);
		assert_eq!(std::mem::size_of::<KeyframeSelector>(), 16);
		assert_eq!(std::mem::size_of::<KeyframesName>(), 16);
		assert_eq!(std::mem::size_of::<KeyframesRuleBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(KeyframesRule, "@keyframes foo{}");
		assert_parse!(KeyframesRule, "@keyframes\"include\"{}");
		assert_parse!(KeyframesRule, "@keyframes spin{0%{rotate:0deg}100%{rotate:360deg}}");
		assert_parse!(KeyframesRule, "@keyframes spin{from,0%{rotate:0deg}to,100%{rotate:360deg}}");
		assert_parse!(KeyframesRule, "@keyframes spin{to{rotate:360deg}}");
		assert_parse!(KeyframesRule, "@keyframes x{to{animation-timing-function:cubic-bezier(0,0,0.2,1)}}");
	}
}
