use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	AtRule, Build, DeclarationList, Parse, Parser, Peek, QualifiedRule, QualifiedRuleList, Result as ParserResult, T,
	diagnostics, keyword_set,
	syntax::{BadDeclaration, CommaSeparated},
};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, ToSpan};
use csskit_proc_macro::visit;

use crate::{Visit, Visitable, properties::Property};

// https://drafts.csswg.org/css-animations/#at-ruledef-keyframes
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct KeyframesRule<'a> {
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

impl<'a> Visitable<'a> for KeyframesRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_keyframes_rule(self);
		Visitable::accept(&self.block, v);
	}
}

#[derive(Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
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

#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct KeyframesBlock<'a> {
	pub open: T!['{'],
	pub keyframes: Vec<'a, Keyframe<'a>>,
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

impl<'a> Visitable<'a> for KeyframesBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for rule in &self.keyframes {
			Visitable::accept(rule, v);
		}
	}
}

#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'a> Visitable<'a> for Keyframe<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_keyframe(self);
		Visitable::accept(&self.selectors, v);
		Visitable::accept(&self.block, v);
	}
}

#[derive(Peek, Parse, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct KeyframeSelectors<'a>(pub CommaSeparated<'a, KeyframeSelector>);

impl<'a> Visitable<'a> for KeyframeSelectors<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for (selector, _) in &self.0 {
			Visitable::accept(selector, v);
		}
	}
}

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

impl<'a> Visitable<'a> for KeyframeBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for (property, _) in &self.properties {
			Visitable::accept(property, v);
		}
	}
}

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum KeyframeSelector {
	From(T![Ident]),
	To(T![Ident]),
	Percent(T![Dimension::%]),
}

keyword_set!(KeyframeSelectorKeyword { From: "from", To: "to" });

impl<'a> Peek<'a> for KeyframeSelector {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		KeyframeSelectorKeyword::peek(p, c) || <T![Dimension::%]>::peek(p, c)
	}
}

impl<'a> Parse<'a> for KeyframeSelector {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(keyword) = p.parse_if_peek::<KeyframeSelectorKeyword>()? {
			return match keyword {
				KeyframeSelectorKeyword::From(c) => Ok(Self::From(<T![Ident]>::build(p, c))),
				KeyframeSelectorKeyword::To(c) => Ok(Self::To(<T![Ident]>::build(p, c))),
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

impl<'a> Visitable<'a> for KeyframeSelector {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_keyframe_selector(self);
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
