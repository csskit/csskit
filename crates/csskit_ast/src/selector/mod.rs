//! CSS AST selector query engine.

mod matcher;
mod metadata;
mod output;
mod query;

pub use matcher::SelectorMatcher;
pub use metadata::{QuerySelectorMetadata, SelectorRequirements, SelectorStructure};
pub use output::MatchOutput;
pub use query::{
	QueryAttribute, QueryAttributeValue, QueryCombinator, QueryCompoundSelector, QueryFunctionalPseudoClass,
	QueryNotPseudo, QueryNthPseudo, QueryPrefixedPseudo, QueryPropertyTypePseudo, QueryPseudoClass,
	QuerySelectorComponent, QuerySelectorList, QueryType, QueryWildcard, SelectorSegment,
};
