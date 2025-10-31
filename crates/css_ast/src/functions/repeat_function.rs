use super::prelude::*;
use crate::{AutoOr, LineWidth, PositiveNonZeroInt};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-repeat-line-width>
///
/// ```text,ignore
/// <repeat-line-width>        = repeat( [ <integer [1,∞]> ] , [ <line-width> ]+ )
/// <auto-repeat-line-width>   = repeat( auto , [ <line-width> ]+ )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct RepeatFunction<'a> {
	#[atom(CssAtomSet::Repeat)]
	pub name: T![Function],
	pub params: RepeatFunctionParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
pub struct RepeatFunctionParams<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub count: AutoOr<PositiveNonZeroInt>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub comma: Option<T![,]>,
	pub tracks: Vec<'a, LineWidth>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<RepeatFunction>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, RepeatFunction, "repeat(2,12px)");
		assert_parse!(CssAtomSet::ATOMS, RepeatFunction, "repeat(auto,15rem)");
		assert_parse!(CssAtomSet::ATOMS, RepeatFunction, "repeat(2,12px 15px 18px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, RepeatFunction, "repeat(none, 12px)");
	}
}
