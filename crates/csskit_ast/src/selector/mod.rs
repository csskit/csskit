//! CSS AST selector query engine.

mod matcher;
mod output;
mod query;

pub use matcher::SelectorMatcher;
pub use output::MatchOutput;
pub use query::{
	QueryAttribute, QueryAttributeValue, QueryCombinator, QueryCompoundSelector, QueryFunctionalPseudoClass,
	QueryNotPseudo, QueryNthPseudo, QueryPrefixedPseudo, QueryPropertyTypePseudo, QueryPseudoClass,
	QuerySelectorComponent, QuerySelectorList, QueryType, QueryWildcard,
};
