use bumpalo::collections::Vec;
use css_lexer::{Cursor, Kind, KindSet};
use css_parse::{
	AtRule, Build, DeclarationList, DeclarationRuleList, NoPreludeAllowed, Parse, Parser, Peek, Result as ParserResult,
	T, atkeyword_set, diagnostics, keyword_set, syntax::CommaSeparated,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};
use csskit_proc_macro::visit;

use crate::{
	Visit, Visitable,
	properties::Property,
	specificity::{Specificity, ToSpecificity},
};

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct PageRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub selectors: Option<PageSelectorList<'a>>,
	pub block: PageRuleBlock<'a>,
}

// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for PageRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, selectors, block) = Self::parse_at_rule(p)?;
		Ok(Self { at_keyword, selectors, block })
	}
}

impl<'a> AtRule<'a> for PageRule<'a> {
	const NAME: Option<&'static str> = Some("page");
	type Prelude = PageSelectorList<'a>;
	type Block = PageRuleBlock<'a>;
}

impl<'a> Visitable<'a> for PageRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_page_rule(self);
		if let Some(selectors) = &self.selectors {
			Visitable::accept(selectors, v);
		}
		Visitable::accept(&self.block, v);
	}
}

#[derive(Peek, Parse, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PageSelectorList<'a>(pub CommaSeparated<'a, PageSelector<'a>>);

impl<'a> Visitable<'a> for PageSelectorList<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for (selector, _) in &self.0 {
			Visitable::accept(selector, v);
		}
	}
}

#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct PageSelector<'a> {
	pub page_type: Option<T![Ident]>,
	pub pseudos: Vec<'a, PagePseudoClass>,
}

impl<'a> Peek<'a> for PageSelector<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident, Kind::Colon]);

	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		c == Self::PEEK_KINDSET
	}
}

impl<'a> Parse<'a> for PageSelector<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut pseudos = Vec::new_in(p.bump());
		let page_type = p.parse_if_peek::<T![Ident]>()?;
		loop {
			if p.peek::<T![:]>() {
				pseudos.push(p.parse::<PagePseudoClass>()?);
			} else {
				return Ok(Self { page_type, pseudos });
			}
		}
	}
}

impl<'a> ToSpecificity for PageSelector<'a> {
	fn specificity(&self) -> Specificity {
		let specificity = self.pseudos.iter().map(ToSpecificity::specificity).sum();
		if self.page_type.is_some() { specificity + Specificity(1, 0, 0) } else { specificity }
	}
}

impl<'a> Visitable<'a> for PageSelector<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_page_selector(self);
	}
}

#[derive(Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PagePseudoClass {
	Left(T![:], T![Ident]),
	Right(T![:], T![Ident]),
	First(T![:], T![Ident]),
	Blank(T![:], T![Ident]),
}

keyword_set!(PagePseudoClassKeyword { Left: "left", Right: "right", First: "first", Blank: "blank" });

impl<'a> Parse<'a> for PagePseudoClass {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let colon = p.parse::<T![:]>()?;
		let skip = p.set_skip(KindSet::NONE);
		let keyword = p.parse::<PagePseudoClassKeyword>();
		p.set_skip(skip);
		let keyword = keyword?;
		let c: Cursor = keyword.into();
		let ident = <T![Ident]>::build(p, c);
		match keyword {
			PagePseudoClassKeyword::Left(_) => Ok(Self::Left(colon, ident)),
			PagePseudoClassKeyword::Right(_) => Ok(Self::Right(colon, ident)),
			PagePseudoClassKeyword::First(_) => Ok(Self::First(colon, ident)),
			PagePseudoClassKeyword::Blank(_) => Ok(Self::Blank(colon, ident)),
		}
	}
}

impl ToSpecificity for PagePseudoClass {
	fn specificity(&self) -> Specificity {
		match self {
			Self::Blank(_, _) => Specificity(0, 1, 0),
			Self::First(_, _) => Specificity(0, 1, 0),
			Self::Left(_, _) => Specificity(0, 0, 1),
			Self::Right(_, _) => Specificity(0, 0, 1),
		}
	}
}

#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct PageRuleBlock<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, (Property<'a>, Option<T![;]>)>,
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, MarginRule<'a>>,
	pub close: Option<T!['}']>,
}

impl<'a> PageRuleBlock<'a> {
	pub fn is_empty(&self) -> bool {
		self.properties.is_empty() && self.rules.is_empty()
	}
}

impl<'a> Parse<'a> for PageRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, rules, close) = Self::parse_declaration_rule_list(p)?;
		Ok(Self { open, properties, rules, close })
	}
}

impl<'a> DeclarationRuleList<'a> for PageRuleBlock<'a> {
	type Declaration = Property<'a>;
	type AtRule = MarginRule<'a>;
}

impl<'a> Visitable<'a> for PageRuleBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for (property, _) in &self.properties {
			Visitable::accept(property, v);
		}
		for rule in &self.rules {
			Visitable::accept(rule, v);
		}
	}
}

// https://drafts.csswg.org/cssom-1/#cssmarginrule
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct MarginRule<'a> {
	pub at_keyword: T![AtKeyword],
	pub block: MarginRuleBlock<'a>,
}

impl<'a> AtRule<'a> for MarginRule<'a> {
	type Prelude = NoPreludeAllowed;
	type Block = MarginRuleBlock<'a>;
}

atkeyword_set!(MarginRuleKeyword {
	TopLeftCorner: "top-left-corner",
	TopLeft: "top-left",
	TopCenter: "top-center",
	TopRight: "top-right",
	TopRightCorner: "top-right-corner",
	RightTop: "right-top",
	RightMiddle: "right-middle",
	RightBottom: "right-bottom",
	BottomRightCorner: "bottom-right-corner",
	BottomRight: "bottom-right",
	BottomCenter: "bottom-center",
	BottomLeft: "bottom-left",
	BottomLeftCorner: "bottom-left-corner",
	LeftBottom: "left-bottom",
	LeftMiddle: "left-middle",
	LeftTop: "left-top"
});

impl<'a> Parse<'a> for MarginRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, _, block) = Self::parse_at_rule(p)?;
		let c: Cursor = at_keyword.into();
		if !MarginRuleKeyword::peek(p, at_keyword.into()) {
			Err(diagnostics::UnexpectedAtRule(p.parse_str(c).into(), c.into()))?
		}
		Ok(Self { at_keyword, block })
	}
}

impl<'a> Visitable<'a> for MarginRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_margin_rule(self);
		Visitable::accept(&self.block, v);
	}
}

#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct MarginRuleBlock<'a> {
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub properties: Vec<'a, (Property<'a>, Option<T![;]>)>,
	pub close: Option<T!['}']>,
}

impl<'a> Parse<'a> for MarginRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, properties, close) = Self::parse_declaration_list(p)?;
		Ok(Self { open, properties, close })
	}
}

impl<'a> DeclarationList<'a> for MarginRuleBlock<'a> {
	type Declaration = Property<'a>;
}

impl<'a> Visitable<'a> for MarginRuleBlock<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		for (property, _) in &self.properties {
			Visitable::accept(property, v);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PageRule>(), 144);
		assert_eq!(std::mem::size_of::<PageSelectorList>(), 32);
		assert_eq!(std::mem::size_of::<PageSelector>(), 48);
		assert_eq!(std::mem::size_of::<PagePseudoClass>(), 28);
		assert_eq!(std::mem::size_of::<PageRuleBlock>(), 96);
		assert_eq!(std::mem::size_of::<MarginRule>(), 80);
		assert_eq!(std::mem::size_of::<MarginRuleBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PageRule, "@page{margin-top:4in;}");
		assert_parse!(PageRule, "@page wide{}");
		assert_parse!(PageRule, "@page wide:left{}");
		assert_parse!(MarginRule, "@top-right{}");
		assert_parse!(PageRule, "@page wide:left{@top-right{}}");
	}
}
