use crate::prelude::*;
use crate::{QuerySelectorList, rule_block::RuleBlock};

/// A node rule for collecting stats.
///
/// # Syntax
///
/// ```css
/// selector {
///   collect: --stat-name;
///   diagnostic: "message";
///   level: warning;
///
///   nested-selector {
///     collect: --another-stat;
///   }
/// }
/// ```
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeRule<'a> {
	pub selector: QuerySelectorList<'a>,
	pub block: RuleBlock<'a>,
}

impl<'a> Parse<'a> for NodeRule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let selector = p.parse::<QuerySelectorList>()?;
		let block = p.parse::<RuleBlock>()?;
		Ok(Self { selector, block })
	}
}
