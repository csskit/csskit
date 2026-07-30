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
pub enum AutoOr<T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Auto)]
	Auto(Ident),
	Some(T),
}

impl<T: NodeWithMetadata<CssMetadata>> NodeWithMetadata<CssMetadata> for AutoOr<T> {
	fn metadata(&self) -> CssMetadata {
		match self {
			Self::Auto(_) => CssMetadata::default(),
			Self::Some(t) => t.metadata(),
		}
	}
}

impl<T: ToNumberValue> ToNumberValue for AutoOr<T> {
	fn to_number_value(&self) -> Option<f32> {
		match self {
			Self::Auto(_) => None,
			Self::Some(t) => t.to_number_value(),
		}
	}
}

impl<T: Copy> Copy for AutoOr<T> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use crate::Length;
	use css_parse::{T, assert_parse, assert_parse_error, assert_peek_false};

	type AutoOrIdent = AutoOr<T![Ident]>;
	type AutoOrNumber = AutoOr<T![Number]>;
	type AutoOrLength = AutoOr<Length>;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AutoOrIdent, "auto", AutoOrIdent::Auto(_));
		assert_parse!(CssAtomSet::ATOMS, AutoOrIdent, "all", AutoOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, AutoOrIdent, "none", AutoOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, AutoOrIdent, "some", AutoOrIdent::Some(_));
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, AutoOrIdent, "");
		assert_peek_false!(CssAtomSet::ATOMS, AutoOrIdent, "0");
		assert_parse_error!(CssAtomSet::ATOMS, AutoOrIdent, "auto auto");
		assert_parse_error!(CssAtomSet::ATOMS, AutoOrIdent, "auto all");
	}

	#[test]
	fn test_to_number_value() {
		assert_parse!(CssAtomSet::ATOMS, AutoOrNumber, "47", |node| {
			assert_eq!(node.to_number_value(), Some(47.0));
		});
		assert_parse!(CssAtomSet::ATOMS, AutoOrLength, "47px", |node| {
			assert_eq!(node.to_number_value(), Some(47.0));
		});
		assert_parse!(CssAtomSet::ATOMS, AutoOrLength, "auto", |node| {
			assert_eq!(node.to_number_value(), None);
		});
	}
}
