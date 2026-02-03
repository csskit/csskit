use super::prelude::*;
use css_parse::token_macros::Ident;

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub enum NormalOr<T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Normal)]
	Normal(Ident),
	Some(T),
}

impl<T: Copy> Copy for NormalOr<T> {}

impl<T> From<NormalOr<T>> for Cursor
where
	T: Copy,
	Cursor: From<T>,
{
	fn from(value: NormalOr<T>) -> Self {
		match value {
			NormalOr::Normal(ident) => ident.into(),
			NormalOr::Some(t) => t.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{T, assert_parse, assert_parse_error};

	type NormalOrIdent = NormalOr<T![Ident]>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NormalOrIdent>(), 16);
	}

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, NormalOrIdent, "normal", NormalOrIdent::Normal(_));
		assert_parse!(CssAtomSet::ATOMS, NormalOrIdent, "all", NormalOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, NormalOrIdent, "auto", NormalOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, NormalOrIdent, "none", NormalOrIdent::Some(_));
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, NormalOrIdent, "");
		assert_parse_error!(CssAtomSet::ATOMS, NormalOrIdent, "0");
		assert_parse_error!(CssAtomSet::ATOMS, NormalOrIdent, "normal normal");
		assert_parse_error!(CssAtomSet::ATOMS, NormalOrIdent, "normal all");
	}
}
