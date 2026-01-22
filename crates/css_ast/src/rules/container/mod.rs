#[cfg(feature = "visitable")]
use crate::visit::{NodeId, QueryableNode};

use super::prelude::*;

mod features;
pub use features::*;

// https://drafts.csswg.org/css-contain-3/#container-rule
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, queryable(skip))]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.container"))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = AtRule, used_at_rules = Container)]
pub struct ContainerRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Container)]
	pub name: T![AtKeyword],
	pub prelude: ContainerConditionList<'a>,
	#[metadata(delegate)]
	pub block: ContainerRulesBlock<'a>,
}

#[cfg(feature = "visitable")]
impl<'a> QueryableNode for ContainerRule<'a> {
	const NODE_ID: NodeId = NodeId::ContainerRule;
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ContainerRulesBlock<'a>(#[metadata(delegate)] pub RuleList<'a, Rule<'a>, CssMetadata>);

#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct ContainerConditionList<'a>(pub CommaSeparated<'a, ContainerCondition<'a>, 1>);

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct ContainerCondition<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: Option<T![Ident]>,
	pub condition: Option<ContainerQuery<'a>>,
}

impl<'a> Peek<'a> for ContainerCondition<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident, Kind::LeftParen]);
}

impl<'a> Parse<'a> for ContainerCondition<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut name = None;
		let c = p.peek_n(1);
		if c == Kind::Ident {
			match p.to_atom::<CssAtomSet>(c) {
				CssAtomSet::None | CssAtomSet::And | CssAtomSet::Not | CssAtomSet::Or => {}
				_ => {
					name = Some(p.parse::<T![Ident]>()?);
				}
			}
		}
		dbg!(p.peek_n(1));
		let condition =
			if name.is_none() { Some(p.parse::<ContainerQuery>()?) } else { p.parse_if_peek::<ContainerQuery>()? };
		dbg!(&name, &condition);
		Ok(Self { name, condition })
	}
}

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ContainerQuery<'a> {
	Is(ContainerFeature<'a>),
	Not(T![Ident], ContainerFeature<'a>),
	And(Vec<'a, (ContainerFeature<'a>, Option<T![Ident]>)>),
	Or(Vec<'a, (ContainerFeature<'a>, Option<T![Ident]>)>),
}

impl<'a> Peek<'a> for ContainerQuery<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<ContainerFeature>::peek(p, c) || (<T![Ident]>::peek(p, c) && p.to_atom::<CssAtomSet>(c) == CssAtomSet::Not)
	}
}

impl<'a> Parse<'a> for ContainerQuery<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_condition(p)
	}
}

impl<'a> FeatureConditionList<'a> for ContainerQuery<'a> {
	type FeatureCondition = ContainerFeature<'a>;
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
	fn build_is(feature: ContainerFeature<'a>) -> Self {
		Self::Is(feature)
	}
	fn build_not(keyword: T![Ident], feature: ContainerFeature<'a>) -> Self {
		Self::Not(keyword, feature)
	}
	fn build_and(feature: Vec<'a, (ContainerFeature<'a>, Option<T![Ident]>)>) -> Self {
		Self::And(feature)
	}
	fn build_or(feature: Vec<'a, (ContainerFeature<'a>, Option<T![Ident]>)>) -> Self {
		Self::Or(feature)
	}
}

macro_rules! container_feature {
	( $($name: ident($typ: ident))+ ) => {
		#[allow(clippy::large_enum_variant)] // TODO: refine
		#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
		pub enum ContainerFeature<'a> {
			$($name($typ),)+
			Style(StyleQuery<'a>),
			ScrollState(ScrollStateQuery<'a>),
		}
	}
}

apply_container_features!(container_feature);

impl<'a> Peek<'a> for ContainerFeature<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c2 = p.peek_n(2);
		c == Kind::LeftParen && c2 == KindSet::new(&[Kind::Ident, Kind::Dimension])
	}
}

impl<'a> Parse<'a> for ContainerFeature<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<T![Function]>() {
			todo!();
		}
		let mut c = p.peek_n(2);
		macro_rules! match_feature {
			( $($name: ident($typ: ident))+ ) => {
				// Only peek at the token as the underlying media feature parser needs to parse the leading keyword.
				{
					match p.to_atom::<CssAtomSet>(c) {
						$(CssAtomSet::$name => {
							let value = $typ::parse(p)?;
							Self::$name(value)
						},)+
						_ => Err(Diagnostic::new(c, Diagnostic::unexpected))?
					}
				}
			}
		}
		if c == Kind::Ident {
			Ok(apply_container_features!(match_feature))
		} else {
			// Styles like (1em < width < 1em) or (1em <= width <= 1em)
			c = p.peek_n(3);
			if c != Kind::Ident {
				c = p.peek_n(4)
			}
			Ok(apply_container_features!(match_feature))
		}
	}
}

macro_rules! apply_container_features {
	($macro: ident) => {
		$macro! {
			// https://drafts.csswg.org/css-conditional-5/#container-features
			Width(WidthContainerFeature)
			Height(HeightContainerFeature)
			InlineSize(InlineSizeContainerFeature)
			BlockSize(BlockSizeContainerFeature)
			AspectRatio(AspectRatioContainerFeature)
			Orientation(OrientationContainerFeature)
		}
	};
}
use apply_container_features;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ContainerRule>(), 144);
		assert_eq!(std::mem::size_of::<ContainerConditionList>(), 32);
		assert_eq!(std::mem::size_of::<ContainerCondition>(), 536);
		assert_eq!(std::mem::size_of::<ContainerQuery>(), 520);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ContainerQuery, "(width:2px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(width:2px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(inline-size>30em)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(1em<width<1em)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(width > 400px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "--container");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "--container (width > 400px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerConditionList, "(width > 400px), --container (width > 400px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo{}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo (width:2px){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo (10em<width<10em){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo (width:2px){body{color:black}}");
	}
}
