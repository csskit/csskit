use super::prelude::*;
use css_parse::token_macros::Ident;

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub enum AutoNoneOr<T> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Auto)]
	Auto(Ident),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::None)]
	None(Ident),
	Some(T),
}

impl<T: ToNumberValue> ToNumberValue for AutoNoneOr<T> {
	fn to_number_value(&self) -> Option<f32> {
		match self {
			Self::None(_) => None,
			Self::Auto(_) => None,
			Self::Some(t) => t.to_number_value(),
		}
	}
}

impl<T: Copy> Copy for AutoNoneOr<T> {}

impl<T> From<AutoNoneOr<T>> for Cursor
where
	T: Copy,
	Cursor: From<T>,
{
	fn from(value: AutoNoneOr<T>) -> Self {
		match value {
			AutoNoneOr::Auto(ident) => ident.into(),
			AutoNoneOr::None(ident) => ident.into(),
			AutoNoneOr::Some(t) => t.into(),
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
	use css_parse::{T, assert_parse, assert_parse_error};

	type AuroNoneOrIdent = AutoNoneOr<T![Ident]>;
	type AutoNoneOrNumber = AutoNoneOr<T![Number]>;
	type AutoNoneOrLength = AutoNoneOr<Length>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AuroNoneOrIdent>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AuroNoneOrIdent, "auto", AuroNoneOrIdent::Auto(_));
		assert_parse!(CssAtomSet::ATOMS, AuroNoneOrIdent, "none", AuroNoneOrIdent::None(_));
		assert_parse!(CssAtomSet::ATOMS, AuroNoneOrIdent, "all", AuroNoneOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, AuroNoneOrIdent, "some", AuroNoneOrIdent::Some(_));
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AuroNoneOrIdent, "");
		assert_parse_error!(CssAtomSet::ATOMS, AuroNoneOrIdent, "0");
		assert_parse_error!(CssAtomSet::ATOMS, AuroNoneOrIdent, "auto none");
		assert_parse_error!(CssAtomSet::ATOMS, AuroNoneOrIdent, "none none");
		assert_parse_error!(CssAtomSet::ATOMS, AuroNoneOrIdent, "auto auto");
		assert_parse_error!(CssAtomSet::ATOMS, AuroNoneOrIdent, "auto all");
	}

	#[test]
	fn test_to_number_value() {
		let bump = Bump::default();
		let source_text = "47";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&bump, source_text, lexer);
		let num = p.parse_entirely::<AutoNoneOrNumber>().output.unwrap();
		assert_eq!(num.to_number_value(), Some(47.0));

		let source_text = "47px";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&bump, source_text, lexer);
		let num = p.parse_entirely::<AutoNoneOrLength>().output.unwrap();
		assert_eq!(num.to_number_value(), Some(47.0));

		let source_text = "none";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&bump, source_text, lexer);
		let num = p.parse_entirely::<AutoNoneOrLength>().output.unwrap();
		assert_eq!(num.to_number_value(), None);
	}
}
