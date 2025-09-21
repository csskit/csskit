use super::prelude::*;

/// <https://drafts.csswg.org/css-content-3/#string-function>
///
/// ```text,ignore
/// string() = string( <custom-ident> , [ first | start | last | first-except ]? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct StringFunction {
	#[atom(CssAtomSet::String)]
	pub name: T![Function],
	pub params: StringFunctionParams,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct StringFunctionParams {
	pub ident: T![Ident],
	pub comma: Option<T![,]>,
	pub keyword: Option<StringKeyword>,
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum StringKeyword {
	#[atom(CssAtomSet::First)]
	First(T![Ident]),
	#[atom(CssAtomSet::Start)]
	Start(T![Ident]),
	#[atom(CssAtomSet::Last)]
	Last(T![Ident]),
	#[atom(CssAtomSet::FirstExcept)]
	FirstExcept(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StringFunction>(), 68);
		assert_eq!(std::mem::size_of::<StringKeyword>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StringFunction, "string(foo)");
		assert_parse!(CssAtomSet::ATOMS, StringFunction, "string(foo,first)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, StringFunction, "string(foo bar)");
	}
}
