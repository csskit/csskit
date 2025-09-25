use super::prelude::*;
use css_parse::token_macros::Ident;

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum NoneOr<T> {
	#[visit(skip)]
	#[atom(CssAtomSet::None)]
	None(Ident),
	Some(T),
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

impl<T> From<NoneOr<T>> for Cursor
where
	T: Copy,
	Cursor: From<T>,
{
	fn from(value: NoneOr<T>) -> Self {
		match value {
			NoneOr::None(ident) => ident.into(),
			NoneOr::Some(t) => t.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use crate::Length;
	use bumpalo::Bump;
	use css_parse::{T, assert_parse, assert_parse_error};

	type NoneOrIdent = NoneOr<T![Ident]>;
	type NoneOrNumber = NoneOr<T![Number]>;
	type NoneOrLength = NoneOr<Length>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NoneOrIdent>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, NoneOrIdent, "none", NoneOrIdent::None(_));
		assert_parse!(CssAtomSet::ATOMS, NoneOrIdent, "all", NoneOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, NoneOrIdent, "auto", NoneOrIdent::Some(_));
		assert_parse!(CssAtomSet::ATOMS, NoneOrIdent, "some", NoneOrIdent::Some(_));
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, NoneOrIdent, "");
		assert_parse_error!(CssAtomSet::ATOMS, NoneOrIdent, "0");
		assert_parse_error!(CssAtomSet::ATOMS, NoneOrIdent, "none none");
		assert_parse_error!(CssAtomSet::ATOMS, NoneOrIdent, "none all");
	}

	#[test]
	fn test_to_number_value() {
		let bump = Bump::default();
		let source_text = "47";
		let mut p = Parser::new(&bump, &CssAtomSet::ATOMS, source_text);
		let num = p.parse_entirely::<NoneOrNumber>().output.unwrap();
		assert_eq!(num.to_number_value(), Some(47.0));

		let source_text = "47px";
		let mut p = Parser::new(&bump, &CssAtomSet::ATOMS, source_text);
		let num = p.parse_entirely::<NoneOrLength>().output.unwrap();
		assert_eq!(num.to_number_value(), Some(47.0));

		let source_text = "none";
		let mut p = Parser::new(&bump, &CssAtomSet::ATOMS, source_text);
		let num = p.parse_entirely::<NoneOrLength>().output.unwrap();
		assert_eq!(num.to_number_value(), None);
	}
}
