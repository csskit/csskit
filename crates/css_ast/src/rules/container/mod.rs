use super::prelude::*;
use css_parse::PreludeList;

mod features;
pub use features::*;

// https://drafts.csswg.org/css-contain-3/#container-rule
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, metadata(skip))]
#[cfg_attr(feature = "css_feature_data", derive(::csskit_derives::ToCSSFeature), css_feature("css.at-rules.container"))]
pub struct ContainerRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Container)]
	pub name: T![AtKeyword],
	pub prelude: ContainerConditionList<'a>,
	pub block: ContainerRulesBlock<'a>,
}

impl<'a> NodeWithMetadata<CssMetadata> for ContainerRule<'a> {
	fn self_metadata(&self) -> CssMetadata {
		CssMetadata { used_at_rules: AtRuleId::Container, node_kinds: NodeKinds::AtRule, ..Default::default() }
	}

	fn metadata(&self) -> CssMetadata {
		self.block.0.metadata().merge(self.self_metadata())
	}
}

#[derive(Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct ContainerRulesBlock<'a>(pub RuleList<'a, Rule<'a>, CssMetadata>);

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct ContainerConditionList<'a>(pub Vec<'a, ContainerCondition<'a>>);

impl<'a> PreludeList<'a> for ContainerConditionList<'a> {
	type PreludeItem = ContainerCondition<'a>;
}

impl<'a> Parse<'a> for ContainerConditionList<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self(Self::parse_prelude_list(p)?))
	}
}

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct ContainerCondition<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: Option<T![Ident]>,
	pub condition: Option<ContainerQuery<'a>>,
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
		let condition =
			if name.is_none() { Some(p.parse::<ContainerQuery>()?) } else { p.parse_if_peek::<ContainerQuery>()? };
		Ok(Self { name, condition })
	}
}

#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
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
		<T![Function]>::peek(p, c) || <T![Ident]>::peek(p, c)
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
		pub enum ContainerFeature<'a> {
			$($name($typ),)+
			Style(StyleQuery<'a>),
			ScrollState(ScrollStateQuery<'a>),
		}
	}
}

apply_container_features!(container_feature);

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
				dbg!(c);
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
		assert_eq!(std::mem::size_of::<ContainerRule>(), 136);
		assert_eq!(std::mem::size_of::<ContainerConditionList>(), 32);
		assert_eq!(std::mem::size_of::<ContainerCondition>(), 416);
		assert_eq!(std::mem::size_of::<ContainerQuery>(), 400);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ContainerQuery, "(width:2px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(width:2px)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(inline-size>30em)");
		assert_parse!(CssAtomSet::ATOMS, ContainerCondition, "(1em<width<1em)");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo{}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo (width:2px){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo (10em<width<10em){}");
		assert_parse!(CssAtomSet::ATOMS, ContainerRule, "@container foo (width:2px){body{color:black}}");
	}
}
