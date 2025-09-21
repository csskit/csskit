use super::prelude::*;

use crate::units::LengthPercentage;

function_set!(pub struct SnapInlineFunctionName "snap-inline");

/// <https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-inline>
///
/// ```text,ignore
/// snap-inline() = snap-inline( <length> , [ left | right | near ]? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct SnapInlineFunction(Function<SnapInlineFunctionName, SnapInlineFunctionParams>);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SnapInlineFunctionParams(LengthPercentage, Option<T![,]>, Option<SnapInlineKeyword>, Option<T![,]>);

keyword_set!(pub enum SnapInlineKeyword { Left: "left", Right: "right", Near: "near" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SnapInlineFunction>(), 92);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SnapInlineFunction, "snap-inline(10%)");
		assert_parse!(SnapInlineFunction, "snap-inline(10%,near)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(SnapInlineFunction, "snap-block(10%)");
		assert_parse_error!(SnapInlineFunction, "snap-inline(near)");
	}
}
