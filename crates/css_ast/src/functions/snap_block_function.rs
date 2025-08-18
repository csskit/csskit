use css_parse::{Function, T, function_set, keyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::units::LengthPercentage;

function_set!(pub struct SnapBlockFunctionName "snap-block");

// https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-block
// snap-block() = snap-block( <length> , [ start | end | near ]? )
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct SnapBlockFunction<'a>(Function<'a, SnapBlockFunctionName, SnapBlockFunctionParams>);

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct SnapBlockFunctionParams(LengthPercentage, Option<T![,]>, Option<SnapBlockKeyword>, Option<T![,]>);

keyword_set!(pub enum SnapBlockKeyword { Start: "start", End: "end", Near: "near" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SnapBlockFunction>(), 92);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SnapBlockFunction, "snap-block(10%)");
		assert_parse!(SnapBlockFunction, "snap-block(10%,start)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(SnapBlockFunction, "snap-inline(10%)");
		assert_parse_error!(SnapBlockFunction, "snap-block(start)");
	}
}
