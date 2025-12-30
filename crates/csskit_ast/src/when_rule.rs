use crate::{CsskitAtomSet, rule_block::RuleBlock};
use bumpalo::collections::Vec;
use css_lexer::{Cursor, Kind, KindSet};
use css_parse::{FeatureConditionList, Parse, Parser, Peek, Result as ParserResult, T};
use csskit_derives::*;

/// An `@when` rule for conditional validation.
///
/// # Syntax
///
/// ```css
/// @when (--stat > threshold) {
///   diagnostic: "message";
///   level: error;
///
///   selector {
///     collect: --another-stat;
///   }
/// }
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WhenRule<'a> {
	#[atom(CsskitAtomSet::When)]
	pub at_keyword: T![AtKeyword],
	pub condition: WhenCondition<'a>,
	pub block: RuleBlock<'a>,
}

/// The condition of an @when rule.
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum WhenCondition<'a> {
	Is(WhenFeature),
	Not(T![Ident], WhenFeature),
	And(Vec<'a, (WhenFeature, Option<T![Ident]>)>),
	Or(Vec<'a, (WhenFeature, Option<T![Ident]>)>),
}

impl<'a> WhenCondition<'a> {
	pub fn is_empty(&self) -> bool {
		match self {
			Self::Is(_) => false,
			Self::Not(_, _) => false,
			Self::And(xs) => xs.is_empty(),
			Self::Or(xs) => xs.is_empty(),
		}
	}

	/// Check if all features in this condition have valid threshold values.
	pub fn is_valid(&self, source: &str) -> bool {
		match self {
			Self::Is(feature) => feature.is_valid(source),
			Self::Not(_, feature) => feature.is_valid(source),
			Self::And(features) => features.iter().all(|(f, _)| f.is_valid(source)),
			Self::Or(features) => features.iter().all(|(f, _)| f.is_valid(source)),
		}
	}
}

impl<'a> FeatureConditionList<'a> for WhenCondition<'a> {
	type FeatureCondition = WhenFeature;

	fn keyword_is_not<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.to_atom::<CsskitAtomSet>(c) == CsskitAtomSet::Not
	}

	fn keyword_is_and<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.to_atom::<CsskitAtomSet>(c) == CsskitAtomSet::And
	}

	fn keyword_is_or<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.to_atom::<CsskitAtomSet>(c) == CsskitAtomSet::Or
	}

	fn build_is(feature: WhenFeature) -> Self {
		Self::Is(feature)
	}

	fn build_not(keyword: T![Ident], feature: WhenFeature) -> Self {
		Self::Not(keyword, feature)
	}

	fn build_and(features: Vec<'a, (WhenFeature, Option<T![Ident]>)>) -> Self {
		Self::And(features)
	}

	fn build_or(features: Vec<'a, (WhenFeature, Option<T![Ident]>)>) -> Self {
		Self::Or(features)
	}
}

impl<'a> Parse<'a> for WhenCondition<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_condition(p)
	}
}

impl<'a> Peek<'a> for WhenCondition<'a> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftParen, Kind::Ident]);
}

/// A single condition feature in an @when rule.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[peek(Kind::LeftParen)]
pub struct WhenFeature {
	pub open: T!['('],
	pub stat: T![DashedIdent],
	pub operator: ComparisonOperator,
	pub value: T![Number],
	pub close: Option<T![')']>,
}

impl WhenFeature {
	/// Check if the threshold value is valid (can be parsed as usize).
	pub fn is_valid(&self, source: &str) -> bool {
		let threshold_cursor = Cursor::from(self.value);
		let threshold_str = threshold_cursor.str_slice(source);
		threshold_str.parse::<usize>().is_ok()
	}
}

/// Comparison operator for @when conditions.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ComparisonOperator {
	/// `>`
	GreaterThan(T![>]),
	/// `<`
	LessThan(T![<]),
	/// `>=`
	GreaterThanOrEqual(T![>=]),
	/// `<=`
	LessThanOrEqual(T![<=]),
	/// `=`
	Equal(T![=]),
}
