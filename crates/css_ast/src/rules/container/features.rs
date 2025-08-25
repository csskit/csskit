use crate::{StyleValue, Visit, VisitMut, Visitable as VisitableTrait, VisitableMut, types::Ratio, units::Length};
use bumpalo::collections::Vec;
use css_parse::{
	ConditionKeyword, Cursor, Declaration, FeatureConditionList, Parse, Parser, Peek, RangedFeatureKeyword,
	Result as ParserResult, discrete_feature, keyword_set, ranged_feature,
};
use csskit_derives::{ToCursors, ToSpan, Visitable};
use csskit_proc_macro::visit;

keyword_set!(pub enum WidthContainerFeatureKeyword { Width: "width" });
impl RangedFeatureKeyword for WidthContainerFeatureKeyword {}

ranged_feature!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum WidthContainerFeature<WidthContainerFeatureKeyword, Length>
);

keyword_set!(pub enum HeightContainerFeatureKeyword { Height: "height" });
impl RangedFeatureKeyword for HeightContainerFeatureKeyword {}

ranged_feature!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum HeightContainerFeature<HeightContainerFeatureKeyword, Length>
);

keyword_set!(pub enum InlineSizeContainerFeatureKeyword { InlineSize: "inline-size" });
impl RangedFeatureKeyword for InlineSizeContainerFeatureKeyword {}

ranged_feature!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum InlineSizeContainerFeature<InlineSizeContainerFeatureKeyword, Length>
);

keyword_set!(pub enum BlockSizeContainerFeatureKeyword { BlockSize: "block-size" });
impl RangedFeatureKeyword for BlockSizeContainerFeatureKeyword {}

ranged_feature!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum BlockSizeContainerFeature<BlockSizeContainerFeatureKeyword, Length>
);

keyword_set!(pub enum AspectRatioContainerFeatureKeyword { AspectRatio: "aspect-ratio" });
impl RangedFeatureKeyword for AspectRatioContainerFeatureKeyword {}

ranged_feature!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum AspectRatioContainerFeature<AspectRatioContainerFeatureKeyword, Ratio>
);

keyword_set!(pub enum OrientationContainerFeatureKeyword { Portrait: "portrait", Landscape: "landscape" });

discrete_feature!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum OrientationContainerFeature<"orientation", OrientationContainerFeatureKeyword>
);

#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
#[visit]
pub enum StyleQuery<'a> {
	Is(Declaration<'a, StyleValue<'a>>),
	Not(ConditionKeyword, Declaration<'a, StyleValue<'a>>),
	And(Vec<'a, (Declaration<'a, StyleValue<'a>>, Option<ConditionKeyword>)>),
	Or(Vec<'a, (Declaration<'a, StyleValue<'a>>, Option<ConditionKeyword>)>),
}

impl<'a> FeatureConditionList<'a> for StyleQuery<'a> {
	type FeatureCondition = Declaration<'a, StyleValue<'a>>;
	fn build_is(feature: Self::FeatureCondition) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: ConditionKeyword, feature: Self::FeatureCondition) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (Self::FeatureCondition, Option<ConditionKeyword>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (Self::FeatureCondition, Option<ConditionKeyword>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for StyleQuery<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_condition(p)
	}
}

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
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
#[visit]
pub enum ScrollStateQuery<'a> {
	Is(ScrollStateFeature),
	Not(ConditionKeyword, ScrollStateFeature),
	And(Vec<'a, (ScrollStateFeature, Option<ConditionKeyword>)>),
	Or(Vec<'a, (ScrollStateFeature, Option<ConditionKeyword>)>),
}

impl<'a> FeatureConditionList<'a> for ScrollStateQuery<'a> {
	type FeatureCondition = ScrollStateFeature;
	fn build_is(feature: ScrollStateFeature) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: ConditionKeyword, feature: ScrollStateFeature) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (ScrollStateFeature, Option<ConditionKeyword>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (ScrollStateFeature, Option<ConditionKeyword>)>) -> Self {
		Self::Or(feature)
	}
}

impl<'a> Parse<'a> for ScrollStateQuery<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_condition(p)
	}
}

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

#[derive(ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum ScrollStateFeature {
	Scrollable(ScrollableScrollStateFeature),
	Snapped(SnappedScrollStateFeature),
	Stuck(StuckScrollStateFeature),
}

keyword_set!(pub enum ScrollStateFeatureKeyword { Scrollable: "scrollable", Snapped: "snapped", Stuck: "stuck" });

impl<'a> Peek<'a> for ScrollStateFeature {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		ScrollStateFeatureKeyword::peek(p, c)
	}
}

impl<'a> Parse<'a> for ScrollStateFeature {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let keyword = p.parse::<ScrollStateFeatureKeyword>()?;
		match keyword {
			ScrollStateFeatureKeyword::Scrollable(_) => p.parse::<ScrollableScrollStateFeature>().map(Self::Scrollable),
			ScrollStateFeatureKeyword::Snapped(_) => p.parse::<SnappedScrollStateFeature>().map(Self::Snapped),
			ScrollStateFeatureKeyword::Stuck(_) => p.parse::<StuckScrollStateFeature>().map(Self::Stuck),
		}
	}
}

discrete_feature!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum ScrollableScrollStateFeature<"scrollable", ScrollableScrollStateFeatureKeyword>
);

keyword_set!(pub enum ScrollableScrollStateFeatureKeyword {
	None: "none",
	Top: "top",
	Right: "right",
	Bottom: "bottom",
	Left: "left",
	BlockStart: "block-start",
	InlineStart: "inline-start",
	BlockEnd: "block-end",
	InlineEnd: "inline-end",
	X: "x",
	Y: "y",
	Block: "block",
	Inline: "inline",
	Discrete: "discrete",
});

discrete_feature!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum SnappedScrollStateFeature<"snapped", SnappedScrollStateFeatureKeyword>
);

keyword_set!(pub enum SnappedScrollStateFeatureKeyword {
	None: "none",
	X: "x",
	Y: "y",
	Block: "block",
	Inline: "inline",
	Both: "both",
	Discrete: "discrete",
});

discrete_feature!(
	#[derive(Visitable)]
	#[visit(self)]
	pub enum StuckScrollStateFeature<"stuck", StuckScrollStateFeatureKeyword>
);

keyword_set!(pub enum StuckScrollStateFeatureKeyword {
	None: "none",
	Top: "top",
	Right: "right",
	Bottom: "bottom",
	Left: "left",
	BlockStart: "block-start",
	InlineStart: "inline-start",
	BlockEnd: "block-end",
	InlineEnd: "inline-end",
	Discrete: "discrete",
});

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WidthContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<HeightContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<InlineSizeContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<BlockSizeContainerFeature>(), 124);
		assert_eq!(std::mem::size_of::<AspectRatioContainerFeature>(), 180);
		assert_eq!(std::mem::size_of::<OrientationContainerFeature>(), 64);
		assert_eq!(std::mem::size_of::<StyleQuery>(), 416);
		assert_eq!(std::mem::size_of::<ScrollStateQuery>(), 88);
		assert_eq!(std::mem::size_of::<ScrollStateFeature>(), 68);
		assert_eq!(std::mem::size_of::<ScrollableScrollStateFeature>(), 64);
		assert_eq!(std::mem::size_of::<SnappedScrollStateFeature>(), 64);
		assert_eq!(std::mem::size_of::<StuckScrollStateFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WidthContainerFeature, "(width:360px)");
		assert_parse!(WidthContainerFeature, "(width>=1400px)");
		assert_parse!(WidthContainerFeature, "(100px<=width)");
		assert_parse!(WidthContainerFeature, "(100px<=width>1400px)");
		assert_parse!(HeightContainerFeature, "(height:360px)");
		assert_parse!(HeightContainerFeature, "(height>=1400px)");
		assert_parse!(HeightContainerFeature, "(100px<=height)");
		assert_parse!(HeightContainerFeature, "(100px<=height>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(WidthContainerFeature, "(min-width > 10px)");
		assert_parse_error!(WidthContainerFeature, "(width: 1%)");
	}
}
