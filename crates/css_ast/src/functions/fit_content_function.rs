use super::prelude::*;
use crate::LengthPercentage;

function_set!(pub struct FitContentFunctionName "fit-content");

/// <https://drafts.csswg.org/css-grid-2/#funcdef-grid-template-columns-fit-content>
///
/// ```text
/// fit-content( <length-percentage> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[visit(self)]
pub struct FitContentFunction(Function<FitContentFunctionName, LengthPercentage>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FitContentFunction>(), 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FitContentFunction, "fit-content(1px)");
		assert_parse!(FitContentFunction, "fit-content(10%)");
	}
}
