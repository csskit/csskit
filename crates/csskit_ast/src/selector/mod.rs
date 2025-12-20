//! CSS AST selector query engine.

mod matcher;
mod output;
mod query;

pub use matcher::SelectorMatcher;
pub use output::MatchOutput;
pub use query::{
	NthValue, QueryAttribute, QueryCombinator, QueryPseudo, QuerySelector, QuerySelectorList, QuerySelectorPart,
	QuerySimpleSelector, SelectorParseError,
};
