use crate::{
	StyleValue,
	specificity::{Specificity, ToSpecificity},
};
use bumpalo::collections::Vec;
use css_parse::{
	AtRule, Block, Build, Cursor, DeclarationList, Kind, KindSet, NoPreludeAllowed, Parse, Parser, Peek,
	Result as ParserResult, T, atkeyword_set, keyword_set, syntax::CommaSeparated,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

atkeyword_set!(struct AtPageKeyword "page");

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.page"))]
#[visit]
pub struct PageRule<'a>(AtRule<AtPageKeyword, Option<PageSelectorList<'a>>, PageRuleBlock<'a>>);

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PageSelectorList<'a>(pub CommaSeparated<'a, PageSelector<'a>>);

#[derive(ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
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

#[derive(Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum PagePseudoClass {
	Left(T![:], T![Ident]),
	Right(T![:], T![Ident]),
	First(T![:], T![Ident]),
	Blank(T![:], T![Ident]),
}

keyword_set!(pub enum PagePseudoClassKeyword { Left: "left", Right: "right", First: "first", Blank: "blank" });

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

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub struct PageRuleBlock<'a>(Block<'a, StyleValue<'a>, MarginRule<'a>>);

atkeyword_set!(
	pub enum AtMarginRuleKeywords {
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
	}
);

// https://drafts.csswg.org/cssom-1/#cssmarginrule
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct MarginRule<'a>(AtRule<AtMarginRuleKeywords, NoPreludeAllowed, MarginRuleBlock<'a>>);

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub struct MarginRuleBlock<'a>(DeclarationList<'a, StyleValue<'a>>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PageRule>(), 160);
		assert_eq!(std::mem::size_of::<PageSelectorList>(), 32);
		assert_eq!(std::mem::size_of::<PageSelector>(), 48);
		assert_eq!(std::mem::size_of::<PagePseudoClass>(), 28);
		assert_eq!(std::mem::size_of::<PageRuleBlock>(), 96);
		assert_eq!(std::mem::size_of::<MarginRule>(), 96);
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
