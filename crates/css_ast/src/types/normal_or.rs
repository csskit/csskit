use super::prelude::*;
use crate::CssMetadata;
use css_parse::NodeWithMetadata;
use css_parse::token_macros::Ident;

#[node]
#[derive(
	Parse, Peek, IntoCursor, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub enum NormalOr<T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Normal)]
	Normal(Ident),
	Some(T),
}

impl<T: NodeWithMetadata<CssMetadata>> NodeWithMetadata<CssMetadata> for NormalOr<T> {
	fn metadata(&self) -> CssMetadata {
		match self {
			Self::Normal(_) => CssMetadata::default(),
			Self::Some(t) => t.metadata(),
		}
	}
}

impl<T: Copy> Copy for NormalOr<T> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{T, assert_parse, assert_parse_error, assert_peek_false};

	type NormalOrIdent = NormalOr<T![Ident]>;

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, NormalOrIdent, "normal", NormalOrIdent::Normal(_));
		assert_parse!(CssAtomSet::ATOMS, NormalOrIdent, "all", NormalOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, NormalOrIdent, "auto", NormalOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, NormalOrIdent, "none", NormalOrIdent::Some(_));
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, NormalOrIdent, "");
		assert_peek_false!(CssAtomSet::ATOMS, NormalOrIdent, "0");
		assert_parse_error!(CssAtomSet::ATOMS, NormalOrIdent, "normal normal");
		assert_parse_error!(CssAtomSet::ATOMS, NormalOrIdent, "normal all");
	}
}
