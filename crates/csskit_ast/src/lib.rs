#![deny(warnings)]

//! CSS AST query utilities using selector-like syntax.

mod collector;
pub mod csskit_atom_set;
pub mod diagnostics;
pub mod node_rule;
pub mod rule_block;
pub mod selector;
pub mod sheet;
pub mod stat_rule;
#[cfg(test)]
mod test_helpers;
pub mod when_rule;

pub use collector::*;
pub use csskit_atom_set::*;
pub use node_rule::*;
pub use rule_block::*;
pub use selector::*;
pub use stat_rule::*;
pub use when_rule::*;

// Re-export for derive macros
pub use css_parse::Diagnostic;

mod prelude {
	pub(crate) use crate::CsskitAtomSet;
	pub(crate) use css_lexer::{Cursor, Kind, KindSet};
	pub(crate) use css_parse::{NodeWithMetadata, Parse, Parser, Peek, Result as ParserResult, T, Vec};
	pub(crate) use csskit_derives::*;
}
