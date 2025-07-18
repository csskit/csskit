use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	AtRule, DeclarationList, Parse, Parser, Peek, QualifiedRule, QualifiedRuleList, Result as ParserResult, T,
	diagnostics, keyword_set,
	syntax::{BadDeclaration, CommaSeparated},
};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, ToSpan, Visitable};

use crate::{Visit, VisitMut, Visitable as VisitableTrait, VisitableMut, properties::Property};

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct KeyframesRule<'a> {
	#[visit(skip)]
	at_keyword: T![AtKeyword],
	name: Option<KeyframesName>,
	block: KeyframesBlock<'a>,
}

impl<'a> AtRule<'a> for KeyframesRule<'a> {
	const NAME: Option<&'static str> = Some("keyframes");
	type Prelude = KeyframesName;
	type Block = KeyframesBlock<'a>;
}

impl<'a> Parse<'a> for KeyframesRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, name, block) = Self::parse_at_rule(p)?;
		Ok(Self { at_keyword, name, block })
	}
}

#[derive(Peek, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum KeyframesName {
	Ident(T![Ident]),
	String(T![String]),
}

impl KeyframesName {
	fn valid_ident(str: &str) -> bool {
		!matches!(str, "default" | "initial" | "inherit" | "unset" | "none")
	}
}

// Must use Parse rather than Build so ReservedKeyframeName errors can be emitted
impl<'a> Parse<'a> for KeyframesName {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![String]>() {
			return Ok(Self::String(p.parse::<T![String]>()?));
		}
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		let str = p.parse_str(c);
		if !KeyframesName::valid_ident(str) {
			Err(diagnostics::ReservedKeyframeName(str.into(), c.into()))?
		}
		Ok(Self::Ident(ident))
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct KeyframesBlock<'a> {
	#[visit(skip)]
	pub open: T!['{'],
	pub keyframes: Vec<'a, Keyframe<'a>>,
	#[visit(skip)]
	pub close: Option<T!['}']>,
}

impl<'a> QualifiedRuleList<'a> for KeyframesBlock<'a> {
	type QualifiedRule = Keyframe<'a>;
}

impl<'a> Parse<'a> for KeyframesBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, keyframes, close) = Self::parse_qualified_rule_list(p)?;
		Ok(Self { open, keyframes, close })
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct Keyframe<'a> {
	selectors: KeyframeSelectors<'a>,
	block: KeyframeBlock<'a>,
}

impl<'a> QualifiedRule<'a> for Keyframe<'a> {
	type Block = KeyframeBlock<'a>;
	type Prelude = KeyframeSelectors<'a>;
	type BadDeclaration = BadDeclaration<'a>;
}

impl<'a> Parse<'a> for Keyframe<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (selectors, block) = Self::parse_qualified_rule(p)?;
		Ok(Self { selectors, block })
	}
}

#[derive(Peek, Parse, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub struct KeyframeSelectors<'a>(pub CommaSeparated<'a, KeyframeSelector>);

#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct KeyframeBlock<'a> {
	open: T!['{'],
	properties: Vec<'a, (Property<'a>, Option<T![;]>)>,
	close: Option<T!['}']>,
}

impl<'a> DeclarationList<'a> for KeyframeBlock<'a> {
	type Declaration = Property<'a>;
}

impl<'a> Parse<'a> for KeyframeBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, close) = Self::parse_declaration_list(p)?;
		Ok(Self { open, properties, close })
	}
}

impl<'a> VisitableTrait for KeyframeBlock<'a> {
	fn accept<V: Visit>(&self, v: &mut V) {
		for (property, _) in &self.properties {
			property.accept(v);
		}
	}
}

impl<'a> VisitableMut for KeyframeBlock<'a> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		for (property, _) in &mut self.properties {
			property.accept_mut(v);
		}
	}
}

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
		assert_eq!(std::mem::size_of::<KeyframesRule>(), 96);
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
