#![deny(warnings)]

//! CSS AST query utilities using selector-like syntax.

pub mod csskit_atom_set;
pub mod diagnostics;
pub mod selector;
#[cfg(test)]
mod test_helpers;

pub use csskit_atom_set::*;
pub use selector::*;
