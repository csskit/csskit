use crate::CssDiagnostic;
use css_parse::{T, keyword_set};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, Visitable};

keyword_set!(pub struct InfiniteKeyword "infinite");

// https://drafts.csswg.org/css-animations/#typedef-single-animation-iteration-count
// <single-animation-iteration-count> = infinite | <number [0,∞]>
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum SingleAnimationIterationCount {
	#[parse(keyword = InfiniteKeyword)]
	Infinite(T![Ident]),
	Number(#[in_range(0.0f32..)] T![Number]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(SingleAnimationIterationCount, "infinite");
		assert_parse!(SingleAnimationIterationCount, "1");
		assert_parse!(SingleAnimationIterationCount, "2.5");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(SingleAnimationIterationCount, "-1");
	}
}
