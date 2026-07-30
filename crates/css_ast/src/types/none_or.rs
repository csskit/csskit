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
pub enum NoneOr<T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::None)]
	None(Ident),
	Some(T),
}

impl<T: NodeWithMetadata<CssMetadata>> NodeWithMetadata<CssMetadata> for NoneOr<T> {
	fn metadata(&self) -> CssMetadata {
		match self {
			Self::None(_) => CssMetadata::default(),
			Self::Some(t) => t.metadata(),
		}
	}
}

impl<T: ToNumberValue> ToNumberValue for NoneOr<T> {
	fn to_number_value(&self) -> Option<f32> {
		match self {
			Self::None(_) => None,
			Self::Some(t) => t.to_number_value(),
		}
	}
}

impl<T: Copy> Copy for NoneOr<T> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use crate::Length;
	use css_parse::{T, assert_parse, assert_parse_error, assert_peek_false};

	type NoneOrIdent = NoneOr<T![Ident]>;
	type NoneOrNumber = NoneOr<T![Number]>;
	type NoneOrLength = NoneOr<Length>;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, NoneOrIdent, "none", NoneOrIdent::None(_));
		assert_parse!(CssAtomSet::ATOMS, NoneOrIdent, "all", NoneOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, NoneOrIdent, "auto", NoneOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, NoneOrIdent, "some", NoneOrIdent::Some(_));
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, NoneOrIdent, "");
		assert_peek_false!(CssAtomSet::ATOMS, NoneOrIdent, "0");
		assert_parse_error!(CssAtomSet::ATOMS, NoneOrIdent, "none none");
		assert_parse_error!(CssAtomSet::ATOMS, NoneOrIdent, "none all");
	}

	#[test]
	fn test_to_number_value() {
		assert_parse!(CssAtomSet::ATOMS, NoneOrNumber, "47", |node| {
			assert_eq!(node.to_number_value(), Some(47.0));
		});
		assert_parse!(CssAtomSet::ATOMS, NoneOrLength, "47px", |node| {
			assert_eq!(node.to_number_value(), Some(47.0));
		});
		assert_parse!(CssAtomSet::ATOMS, NoneOrLength, "none", |node| {
			assert_eq!(node.to_number_value(), None);
		});
	}
}
