use crate::{CssAtomSet, StyleValue, types::Ratio, units::Length};
#[cfg(feature = "visitable")]
use crate::{Visit, VisitMut, Visitable as VisitableTrait, VisitableMut};
use bumpalo::collections::Vec;
use css_parse::{
	Cursor, Declaration, FeatureConditionList, Parse, Parser, Result as ParserResult, T, discrete_feature,
	ranged_feature,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};
#[cfg(feature = "visitable")]
use csskit_proc_macro::visit;

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum WidthContainerFeature{CssAtomSet::Width, Length}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum HeightContainerFeature{CssAtomSet::Height, Length}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum InlineSizeContainerFeature{CssAtomSet::InlineSize, Length}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum BlockSizeContainerFeature{CssAtomSet::BlockSize, Length}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum AspectRatioContainerFeature{CssAtomSet::AspectRatio, Ratio}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum OrientationContainerFeatureKeyword {
	#[atom(CssAtomSet::Portrait)]
	Portrait(T![Ident]),
	#[atom(CssAtomSet::Landscape)]
	Landscape(T![Ident]),
}

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum OrientationContainerFeature{CssAtomSet::Orientation, OrientationContainerFeatureKeyword}
);

#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", visit)]
pub enum StyleQuery<'a> {
	Is(Declaration<'a, StyleValue<'a>>),
	Not(T![Ident], Declaration<'a, StyleValue<'a>>),
	And(Vec<'a, (Declaration<'a, StyleValue<'a>>, Option<T![Ident]>)>),
	Or(Vec<'a, (Declaration<'a, StyleValue<'a>>, Option<T![Ident]>)>),
}

impl<'a> FeatureConditionList<'a> for StyleQuery<'a> {
	type FeatureCondition = Declaration<'a, StyleValue<'a>>;
	fn keyword_is_not(p: &Parser, c: Cursor) -> bool {
		p.equals_atom(c, &CssAtomSet::Not)
	}
	fn keyword_is_and(p: &Parser, c: Cursor) -> bool {
		p.equals_atom(c, &CssAtomSet::And)
	}
	fn keyword_is_or(p: &Parser, c: Cursor) -> bool {
		p.equals_atom(c, &CssAtomSet::Or)
	}
	fn build_is(feature: Self::FeatureCondition) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: T![Ident], feature: Self::FeatureCondition) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (Self::FeatureCondition, Option<T![Ident]>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (Self::FeatureCondition, Option<T![Ident]>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for StyleQuery<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_condition(p)
	}
}

#[cfg(feature = "visitable")]
impl<'a> VisitableTrait for StyleQuery<'a> {
	fn accept<V: Visit>(&self, v: &mut V) {
		v.visit_style_query(self);
		match self {
			Self::Is(feature) => feature.accept(v),
			Self::Not(_, feature) => feature.accept(v),
			Self::And(features) => {
				for (feature, _) in features {
					feature.accept(v);
				}
			}
			Self::Or(features) => {
				for (feature, _) in features {
					feature.accept(v);
				}
			}
		}
	}
}

#[cfg(feature = "visitable")]
impl<'a> VisitableMut for StyleQuery<'a> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		v.visit_style_query(self);
		match self {
			Self::Is(feature) => feature.accept_mut(v),
			Self::Not(_, feature) => feature.accept_mut(v),
			Self::And(features) => {
				for (feature, _) in features {
					feature.accept_mut(v);
				}
			}
			Self::Or(features) => {
				for (feature, _) in features {
					feature.accept_mut(v);
				}
			}
		}
	}
}

#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", visit)]
pub enum ScrollStateQuery<'a> {
	Is(ScrollStateFeature),
	Not(T![Ident], ScrollStateFeature),
	And(Vec<'a, (ScrollStateFeature, Option<T![Ident]>)>),
	Or(Vec<'a, (ScrollStateFeature, Option<T![Ident]>)>),
}

impl<'a> FeatureConditionList<'a> for ScrollStateQuery<'a> {
	type FeatureCondition = ScrollStateFeature;
	fn keyword_is_not(p: &Parser, c: Cursor) -> bool {
		p.equals_atom(c, &CssAtomSet::Not)
	}
	fn keyword_is_and(p: &Parser, c: Cursor) -> bool {
		p.equals_atom(c, &CssAtomSet::And)
	}
	fn keyword_is_or(p: &Parser, c: Cursor) -> bool {
		p.equals_atom(c, &CssAtomSet::Or)
	}
	fn build_is(feature: ScrollStateFeature) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: T![Ident], feature: ScrollStateFeature) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (ScrollStateFeature, Option<T![Ident]>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (ScrollStateFeature, Option<T![Ident]>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for ScrollStateQuery<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_condition(p)
	}
}

#[cfg(feature = "visitable")]
impl<'a> VisitableTrait for ScrollStateQuery<'a> {
	fn accept<V: Visit>(&self, v: &mut V) {
		match self {
			Self::Is(feature) => feature.accept(v),
			Self::Not(_, feature) => feature.accept(v),
			Self::And(features) => {
				for (feature, _) in features {
					feature.accept(v);
				}
			}
			Self::Or(features) => {
				for (feature, _) in features {
					feature.accept(v);
				}
			}
		}
	}
}

#[cfg(feature = "visitable")]
impl<'a> VisitableMut for ScrollStateQuery<'a> {
	fn accept_mut<V: VisitMut>(&mut self, v: &mut V) {
		match self {
			Self::Is(feature) => feature.accept_mut(v),
			Self::Not(_, feature) => feature.accept_mut(v),
			Self::And(features) => {
				for (feature, _) in features {
					feature.accept_mut(v);
				}
			}
			Self::Or(features) => {
				for (feature, _) in features {
					feature.accept_mut(v);
				}
			}
		}
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum ScrollStateFeature {
	Scrollable(ScrollableScrollStateFeature),
	Snapped(SnappedScrollStateFeature),
	Stuck(StuckScrollStateFeature),
}

discrete_feature!(
	#[derive(Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum ScrollableScrollStateFeature{CssAtomSet::Scrollable, ScrollableScrollStateFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum ScrollableScrollStateFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Top)]
	Top(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
	#[atom(CssAtomSet::Bottom)]
	Bottom(T![Ident]),
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::BlockStart)]
	BlockStart(T![Ident]),
	#[atom(CssAtomSet::InlineStart)]
	InlineStart(T![Ident]),
	#[atom(CssAtomSet::BlockEnd)]
	BlockEnd(T![Ident]),
	#[atom(CssAtomSet::InlineEnd)]
	InlineEnd(T![Ident]),
	#[atom(CssAtomSet::X)]
	X(T![Ident]),
	#[atom(CssAtomSet::Y)]
	Y(T![Ident]),
	#[atom(CssAtomSet::Block)]
	Block(T![Ident]),
	#[atom(CssAtomSet::Inline)]
	Inline(T![Ident]),
	#[atom(CssAtomSet::Discrete)]
	Discrete(T![Ident]),
}

discrete_feature!(
	#[derive(Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum SnappedScrollStateFeature{CssAtomSet::Snapped, SnappedScrollStateFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum SnappedScrollStateFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::X)]
	X(T![Ident]),
	#[atom(CssAtomSet::Y)]
	Y(T![Ident]),
	#[atom(CssAtomSet::Block)]
	Block(T![Ident]),
	#[atom(CssAtomSet::Inline)]
	Inline(T![Ident]),
	#[atom(CssAtomSet::Both)]
	Both(T![Ident]),
	#[atom(CssAtomSet::Discrete)]
	Discrete(T![Ident]),
}

discrete_feature!(
	#[derive(Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
	pub enum StuckScrollStateFeature{CssAtomSet::Stuck, StuckScrollStateFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum StuckScrollStateFeatureKeyword {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Top)]
	Top(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
	#[atom(CssAtomSet::Bottom)]
	Bottom(T![Ident]),
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::BlockStart)]
	BlockStart(T![Ident]),
	#[atom(CssAtomSet::InlineStart)]
	InlineStart(T![Ident]),
	#[atom(CssAtomSet::BlockEnd)]
	BlockEnd(T![Ident]),
	#[atom(CssAtomSet::InlineEnd)]
	InlineEnd(T![Ident]),
	#[atom(CssAtomSet::Discrete)]
	Discrete(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WidthContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<HeightContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<InlineSizeContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<BlockSizeContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<AspectRatioContainerFeature>(), 180);
		assert_eq!(std::mem::size_of::<OrientationContainerFeature>(), 64);
		assert_eq!(std::mem::size_of::<StyleQuery>(), 384);
		assert_eq!(std::mem::size_of::<ScrollStateQuery>(), 80);
		assert_eq!(std::mem::size_of::<ScrollStateFeature>(), 68);
		assert_eq!(std::mem::size_of::<ScrollableScrollStateFeature>(), 64);
		assert_eq!(std::mem::size_of::<SnappedScrollStateFeature>(), 64);
		assert_eq!(std::mem::size_of::<StuckScrollStateFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WidthContainerFeature, "(width:360px)");
		assert_parse!(CssAtomSet::ATOMS, WidthContainerFeature, "(width>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthContainerFeature, "(100px<=width)");
		assert_parse!(CssAtomSet::ATOMS, WidthContainerFeature, "(100px<=width>1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightContainerFeature, "(height:360px)");
		assert_parse!(CssAtomSet::ATOMS, HeightContainerFeature, "(height>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightContainerFeature, "(100px<=height)");
		assert_parse!(CssAtomSet::ATOMS, HeightContainerFeature, "(100px<=height>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WidthContainerFeature, "(min-width > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthContainerFeature, "(width: 1%)");
	}
}
