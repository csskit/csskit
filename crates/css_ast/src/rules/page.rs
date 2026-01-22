use super::prelude::*;
use crate::specificity::{Specificity, ToSpecificity};
use css_parse::RuleVariants;

// https://drafts.csswg.org/cssom-1/#csspagerule
// https://drafts.csswg.org/css-page-3/#at-page-rule
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.page"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Page)]
pub struct PageRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Page)]
	pub name: T![AtKeyword],
	pub prelude: Option<PageSelectorList<'a>>,
	#[metadata(delegate)]
	pub block: PageRuleBlock<'a>,
}

#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PageSelectorList<'a>(pub CommaSeparated<'a, PageSelector<'a>>);

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PageSelector<'a> {
	pub page_type: Option<T![Ident]>,
	pub pseudos: Vec<'a, PagePseudoClass>,
}

impl<'a> Peek<'a> for PageSelector<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident, Kind::Colon]);

	fn peek<I>(_: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		c == Self::PEEK_KINDSET
	}
}

impl<'a> Parse<'a> for PageSelector<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
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

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PagePseudoClass {
	Left(T![:], T![Ident]),
	Right(T![:], T![Ident]),
	First(T![:], T![Ident]),
	Blank(T![:], T![Ident]),
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

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PageRuleBlock<'a>(#[metadata(delegate)] Block<'a, StyleValue<'a>, MarginRule<'a>, CssMetadata>);

// https://drafts.csswg.org/cssom-1/#cssmarginrule
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum MarginRule<'a> {
	#[atom(CssAtomSet::TopLeftCorner)]
	TopLeftCorner(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::TopLeft)]
	TopLeft(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::TopCenter)]
	TopCenter(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::TopRight)]
	TopRight(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::TopRightCorner)]
	TopRightCorner(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::RightTop)]
	RightTop(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::RightMiddle)]
	RightMiddle(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::RightBottom)]
	RightBottom(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::BottomRightCorner)]
	BottomRightCorner(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::BottomRight)]
	BottomRight(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::BottomCenter)]
	BottomCenter(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::BottomLeft)]
	BottomLeft(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::BottomLeftCorner)]
	BottomLeftCorner(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::LeftBottom)]
	LeftBottom(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::LeftMiddle)]
	LeftMiddle(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
	#[atom(CssAtomSet::LeftTop)]
	LeftTop(#[cfg_attr(feature = "visitable", visit(skip))] T![AtKeyword], MarginRuleBlock<'a>),
}

impl<'a> NodeWithMetadata<CssMetadata> for MarginRule<'a> {
	fn metadata(&self) -> CssMetadata {
		self.block().0.metadata()
	}
}

impl<'a> MarginRule<'a> {
	pub fn name(&self) -> &T![AtKeyword] {
		match self {
			Self::TopLeftCorner(a, _) => a,
			Self::TopLeft(a, _) => a,
			Self::TopCenter(a, _) => a,
			Self::TopRight(a, _) => a,
			Self::TopRightCorner(a, _) => a,
			Self::RightTop(a, _) => a,
			Self::RightMiddle(a, _) => a,
			Self::RightBottom(a, _) => a,
			Self::BottomRightCorner(a, _) => a,
			Self::BottomRight(a, _) => a,
			Self::BottomCenter(a, _) => a,
			Self::BottomLeft(a, _) => a,
			Self::BottomLeftCorner(a, _) => a,
			Self::LeftBottom(a, _) => a,
			Self::LeftMiddle(a, _) => a,
			Self::LeftTop(a, _) => a,
		}
	}

	pub fn block(&self) -> &MarginRuleBlock<'a> {
		match self {
			Self::TopLeftCorner(_, b) => b,
			Self::TopLeft(_, b) => b,
			Self::TopCenter(_, b) => b,
			Self::TopRight(_, b) => b,
			Self::TopRightCorner(_, b) => b,
			Self::RightTop(_, b) => b,
			Self::RightMiddle(_, b) => b,
			Self::RightBottom(_, b) => b,
			Self::BottomRightCorner(_, b) => b,
			Self::BottomRight(_, b) => b,
			Self::BottomCenter(_, b) => b,
			Self::BottomLeft(_, b) => b,
			Self::BottomLeftCorner(_, b) => b,
			Self::LeftBottom(_, b) => b,
			Self::LeftMiddle(_, b) => b,
			Self::LeftTop(_, b) => b,
		}
	}
}

impl<'a> RuleVariants<'a> for MarginRule<'a> {
	type DeclarationValue = StyleValue<'a>;
	type Metadata = CssMetadata;
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
pub struct MarginRuleBlock<'a>(DeclarationList<'a, StyleValue<'a>, CssMetadata>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PageRule>(), 176);
		assert_eq!(std::mem::size_of::<PageSelectorList>(), 32);
		assert_eq!(std::mem::size_of::<PageSelector>(), 48);
		assert_eq!(std::mem::size_of::<PagePseudoClass>(), 28);
		assert_eq!(std::mem::size_of::<PageRuleBlock>(), 128);
		assert_eq!(std::mem::size_of::<MarginRule>(), 112);
		assert_eq!(std::mem::size_of::<MarginRuleBlock>(), 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PageRule, "@page{margin-top:4in;}");
		assert_parse!(CssAtomSet::ATOMS, PageRule, "@page wide{}");
		assert_parse!(CssAtomSet::ATOMS, PageRule, "@page wide:left{}");
		assert_parse!(CssAtomSet::ATOMS, MarginRule, "@top-right{}");
		assert_parse!(CssAtomSet::ATOMS, PageRule, "@page wide:left{@top-right{}}");
	}
}
