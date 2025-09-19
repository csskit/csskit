use super::prelude::*;

function_set!(pub struct StringFunctionName "string");

/// <https://drafts.csswg.org/css-content-3/#string-function>
///
/// ```text,ignore
/// string() = string( <custom-ident> , [ first | start | last | first-except ]? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct StringFunction(Function<StringFunctionName, (T![Ident], Option<T![,]>, Option<StringKeywords>)>);

keyword_set!(
	pub enum StringKeywords {
		First: "first",
		Start: "start",
		Last: "last",
		FirstExcept: "first-except"
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StringFunction>(), 72);
		assert_eq!(std::mem::size_of::<StringKeywords>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(StringFunction, "string(foo)");
		assert_parse!(StringFunction, "string(foo,first)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(StringFunction, "string(foo bar)");
	}
}
