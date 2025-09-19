use super::prelude::*;
use crate::{AutoOr, LineWidth, PositiveNonZeroInt};

function_set!(pub struct RepeatFunctionName "repeat");

/// <https://drafts.csswg.org/css-gaps-1/#typedef-repeat-line-width>
///
/// ```text,ignore
/// <repeat-line-width>        = repeat( [ <integer [1,∞]> ] , [ <line-width> ]+ )
/// <auto-repeat-line-width>   = repeat( auto , [ <line-width> ]+ )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct RepeatFunction<'a>(Function<RepeatFunctionName, RepeatFunctionParams<'a>>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct RepeatFunctionParams<'a> {
	#[visit(skip)]
	pub count: AutoOr<PositiveNonZeroInt>,
	#[visit(skip)]
	pub comma: Option<T![,]>,
	pub tracks: Vec<'a, LineWidth>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<RepeatFunction>(), 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(RepeatFunction, "repeat(2,12px)");
		assert_parse!(RepeatFunction, "repeat(auto,15rem)");
		assert_parse!(RepeatFunction, "repeat(2,12px 15px 18px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(RepeatFunction, "repeat(none, 12px)");
	}
}
