use super::prelude::*;

function_set!(pub struct ContentFunctionName "content");

/// <https://drafts.csswg.org/css-content-3/#funcdef-content>
///
/// ```text,ignore
/// content() = content( [ text | before | after | first-letter | marker ]? )
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct ContentFunction(Function<ContentFunctionName, Option<ContentKeywords>>);

keyword_set!(
	pub enum ContentKeywords {
		Text: "text",
		Before: "before",
		After: "after",
		FirstLetter: "first-letter",
		Marker: "marker"
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ContentFunction>(), 44);
		assert_eq!(std::mem::size_of::<ContentKeywords>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ContentFunction, "content(text)");
		assert_parse!(ContentFunction, "content(before)");
		assert_parse!(ContentFunction, "content()");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ContentFunction, "content(text before)");
	}
}
