use super::super::prelude::*;
use crate::{types::Ratio, units::Length};
use css_parse::{discrete_feature, ranged_feature};

ranged_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum WidthContainerFeature{CssAtomSet::Width, Length}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum HeightContainerFeature{CssAtomSet::Height, Length}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum InlineSizeContainerFeature{CssAtomSet::InlineSize, Length}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum BlockSizeContainerFeature{CssAtomSet::BlockSize, Length}
);

ranged_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum AspectRatioContainerFeature{CssAtomSet::AspectRatio, Ratio}
);

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum OrientationContainerFeatureKeyword {
	#[atom(CssAtomSet::Portrait)]
	Portrait(T![Ident]),
	#[atom(CssAtomSet::Landscape)]
	Landscape(T![Ident]),
}

discrete_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
	pub enum OrientationContainerFeature{CssAtomSet::Orientation, OrientationContainerFeatureKeyword}
);

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum StyleQuery<'a> {
	Is(Declaration<'a, StyleValue<'a>, CssMetadata>),
	Not(T![Ident], Declaration<'a, StyleValue<'a>, CssMetadata>),
	And(Vec<'a, (Declaration<'a, StyleValue<'a>, CssMetadata>, Option<T![Ident]>)>),
	Or(Vec<'a, (Declaration<'a, StyleValue<'a>, CssMetadata>, Option<T![Ident]>)>),
}

impl<'a> FeatureConditionList<'a> for StyleQuery<'a> {
	type FeatureCondition = Declaration<'a, StyleValue<'a>, CssMetadata>;
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
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_condition(p)
	}
}

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ScrollStateQuery<'a> {
	Is(ScrollStateFeature),
	Not(T![Ident], ScrollStateFeature),
	And(Vec<'a, (ScrollStateFeature, Option<T![Ident]>)>),
	Or(Vec<'a, (ScrollStateFeature, Option<T![Ident]>)>),
}

impl<'a> FeatureConditionList<'a> for ScrollStateQuery<'a> {
	type FeatureCondition = ScrollStateFeature;
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
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_condition(p)
	}
}

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ScrollStateFeature {
	Stuck(
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T!['(']>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![Ident],
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T![:]>,
		Option<StuckScrollStateFeatureKeyword>,
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T![')']>,
	),
	Snapped(
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T!['(']>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![Ident],
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T![:]>,
		Option<SnappedScrollStateFeatureKeyword>,
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T![')']>,
	),
	Scrollable(
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T!['(']>,
		#[cfg_attr(feature = "visitable", visit(skip))] T![Ident],
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T![:]>,
		Option<ScrollableScrollStateFeatureKeyword>,
		#[cfg_attr(feature = "visitable", visit(skip))] Option<T![')']>,
	),
}

impl<'a> Peek<'a> for ScrollStateFeature {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Bare form: `stuck: top`
		if c == Kind::Ident
			&& matches!(p.to_atom::<CssAtomSet>(c), CssAtomSet::Stuck | CssAtomSet::Snapped | CssAtomSet::Scrollable)
		{
			return true;
		}
		// Paren-wrapped form: `(stuck: top)`
		if c == Kind::LeftParen {
			let c2 = p.peek_n(2);
			return c2 == Kind::Ident
				&& matches!(
					p.to_atom::<CssAtomSet>(c2),
					CssAtomSet::Stuck | CssAtomSet::Snapped | CssAtomSet::Scrollable
				);
		}
		false
	}
}

impl<'a> Parse<'a> for ScrollStateFeature {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let open = p.parse_if_peek::<T!['(']>()?;
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		let colon = p.parse_if_peek::<T![:]>()?;
		match p.to_atom::<CssAtomSet>(c) {
			CssAtomSet::Stuck => {
				let value = if colon.is_some() { Some(p.parse::<StuckScrollStateFeatureKeyword>()?) } else { None };
				let close = if open.is_some() { Some(p.parse::<T![')']>()?) } else { None };
				Ok(Self::Stuck(open, ident, colon, value, close))
			}
			CssAtomSet::Snapped => {
				let value = if colon.is_some() { Some(p.parse::<SnappedScrollStateFeatureKeyword>()?) } else { None };
				let close = if open.is_some() { Some(p.parse::<T![')']>()?) } else { None };
				Ok(Self::Snapped(open, ident, colon, value, close))
			}
			CssAtomSet::Scrollable => {
				let value =
					if colon.is_some() { Some(p.parse::<ScrollableScrollStateFeatureKeyword>()?) } else { None };
				let close = if open.is_some() { Some(p.parse::<T![')']>()?) } else { None };
				Ok(Self::Scrollable(open, ident, colon, value, close))
			}
			_ => Err(Diagnostic::new(c, Diagnostic::unexpected_ident))?,
		}
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
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

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
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

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
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
		assert_eq!(std::mem::size_of::<StyleQuery>(), 504);
		assert_eq!(std::mem::size_of::<ScrollStateQuery>(), 96);
		assert_eq!(std::mem::size_of::<ScrollStateFeature>(), 80);
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
