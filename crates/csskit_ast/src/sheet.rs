use bumpalo::collections::Vec;
use css_lexer::{Cursor, Kind, SourceOffset, Span};
use css_parse::{
	ComponentValues, CursorSink, NodeWithMetadata, Parse, Parser, Peek, Result as ParserResult, RuleVariants,
	SemanticEq, StyleSheet, ToCursors, ToSpan,
};
use csskit_derives::*;

use crate::CsskitAtomSet;
use crate::node_rule::NodeRule;
use crate::stat_rule::StatRule;
use crate::when_rule::WhenRule;

/// A stats stylesheet containing rules for collecting statistics and diagnostics.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Sheet<'a> {
	pub rules: Vec<'a, Rule<'a>>,
}

impl<'a> Parse<'a> for Sheet<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let (rules, _) = Self::parse_stylesheet(p)?;
		Ok(Self { rules })
	}
}

impl<'a> StyleSheet<'a, ()> for Sheet<'a> {
	type Rule = Rule<'a>;
}

impl ToCursors for Sheet<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for rule in &self.rules {
			rule.to_cursors(s);
		}
	}
}

impl ToSpan for Sheet<'_> {
	fn to_span(&self) -> Span {
		if self.rules.is_empty() {
			return Span::new(SourceOffset(0), SourceOffset(0));
		}
		let start = self.rules.first().map(|r| r.to_span().start()).unwrap_or(SourceOffset(0));
		let end = self.rules.last().map(|r| r.to_span().end()).unwrap_or(SourceOffset(0));
		Span::new(start, end)
	}
}

impl SemanticEq for Sheet<'_> {
	fn semantic_eq(&self, other: &Self) -> bool {
		if self.rules.len() != other.rules.len() {
			return false;
		}
		self.rules.iter().zip(other.rules.iter()).all(|(a, b)| a.semantic_eq(b))
	}
}

/// A rule within a stats stylesheet.
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Rule<'a> {
	/// A `@stat` rule defining a statistic.
	Stat(StatRule<'a>),
	/// A node rule for collecting stats (e.g., `selector { collect: --foo; }`).
	NodeRule(NodeRule<'a>),
	/// An `@when` rule for conditional validation.
	WhenRule(WhenRule<'a>),
	/// Unknown rule.
	Unknown(ComponentValues<'a>),
}

impl<'a> Parse<'a> for Rule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_rule_variants(p)
	}
}

impl<'a> RuleVariants<'a> for Rule<'a> {
	type DeclarationValue = ComponentValues<'a>;
	type Metadata = ();

	fn parse_at_rule<I>(p: &mut Parser<'a, I>, c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let atom = p.to_atom::<CsskitAtomSet>(c);
		match atom {
			CsskitAtomSet::Stat => Ok(Self::Stat(p.parse::<StatRule>()?)),
			CsskitAtomSet::When => Ok(Self::WhenRule(p.parse::<WhenRule>()?)),
			_ => Err(css_parse::Diagnostic::new(c, css_parse::Diagnostic::unexpected))?,
		}
	}

	fn parse_unknown_at_rule<I>(p: &mut Parser<'a, I>, _c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self::Unknown(p.parse::<ComponentValues>()?))
	}

	fn parse_qualified_rule<I>(p: &mut Parser<'a, I>, _c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self::NodeRule(p.parse::<NodeRule>()?))
	}

	fn parse_unknown_qualified_rule<I>(p: &mut Parser<'a, I>, _c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self::Unknown(p.parse::<ComponentValues>()?))
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}
}

impl<'a> Peek<'a> for Rule<'a> {
	fn peek<I>(_p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		c != Kind::Eof
	}
}

impl NodeWithMetadata<()> for Rule<'_> {
	fn metadata(&self) {}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn test_stats_sheet_with_stat_rules() {
		assert_parse!(
			CsskitAtomSet::ATOMS,
			Sheet,
			"@stat --total-selectors{type:counter}@stat --unique-selectors{type:unique;normalize:true}"
		);
	}

	#[test]
	fn test_stats_rule_stat() {
		assert_parse!(CsskitAtomSet::ATOMS, Rule, "@stat --total-selectors{type:counter}", Rule::Stat(_));
	}
}
