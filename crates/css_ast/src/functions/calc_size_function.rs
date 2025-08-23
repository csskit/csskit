use css_parse::{Function, function_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::Todo;

function_set!(pub struct CalcSizeFunctionName "calc-size");

/// <https://drafts.csswg.org/css-values-5/#calc-size>
///
/// ```text,ignore
/// <calc-size()> = calc-size( <calc-size-basis>, <calc-sum> )
/// <calc-size-basis> = [ <size-keyword> | <calc-size()> | any | <calc-sum> ]
/// ```
///
/// The `<size-keyword>` production matches any sizing keywords allowed in the context.
/// For example, in width, it matches auto, min-content, stretch, etc.
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct CalcSizeFunction(Function<CalcSizeFunctionName, Todo>);

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CalcSizeFunction>(), 28);
	}
}
