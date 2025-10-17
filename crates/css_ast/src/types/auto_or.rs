use super::prelude::*;
use css_parse::token_macros::Ident;

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub enum AutoOr<T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Auto)]
	Auto(Ident),
	Some(T),
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

impl<T> From<AutoOr<T>> for Cursor
where
	T: Copy,
	Cursor: From<T>,
{
	fn from(value: AutoOr<T>) -> Self {
		match value {
			AutoOr::Auto(ident) => ident.into(),
			AutoOr::Some(t) => t.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use crate::Length;
	use bumpalo::Bump;
	use css_lexer::Lexer;
	use css_parse::{Parser, T, assert_parse, assert_parse_error};

	type AutoOrIdent = AutoOr<T![Ident]>;
	type AutoOrNumber = AutoOr<T![Number]>;
	type AutoOrLength = AutoOr<Length>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AutoOrIdent>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AutoOrIdent, "auto", AutoOrIdent::Auto(_));
		assert_parse!(CssAtomSet::ATOMS, AutoOrIdent, "all", AutoOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, AutoOrIdent, "none", AutoOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, AutoOrIdent, "some", AutoOrIdent::Some(_));
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AutoOrIdent, "");
		assert_parse_error!(CssAtomSet::ATOMS, AutoOrIdent, "0");
		assert_parse_error!(CssAtomSet::ATOMS, AutoOrIdent, "auto auto");
		assert_parse_error!(CssAtomSet::ATOMS, AutoOrIdent, "auto all");
	}

	#[test]
	fn test_to_number_value() {
		let bump = Bump::default();
		let source_text = "47";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&bump, source_text, lexer);
		let num = p.parse_entirely::<AutoOrNumber>().output.unwrap();
		assert_eq!(num.to_number_value(), Some(47.0));

		let source_text = "47px";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&bump, source_text, lexer);
		let num = p.parse_entirely::<AutoOrLength>().output.unwrap();
		assert_eq!(num.to_number_value(), Some(47.0));

		let source_text = "auto";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&bump, source_text, lexer);
		let num = p.parse_entirely::<AutoOrLength>().output.unwrap();
		assert_eq!(num.to_number_value(), None);
	}
}
