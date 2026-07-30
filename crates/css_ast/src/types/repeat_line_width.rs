use super::prelude::*;
use crate::{AutoOr, LineWidth, NonEmpty, PositiveNonZeroInt, RepeatFunction};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-repeat-line-width>
///
/// ```text,ignore
/// <repeat-line-width>        = repeat( [ <integer [1,∞]> ] , [ <line-width> ]+ )
/// <auto-repeat-line-width>   = repeat( auto , [ <line-width> ]+ )
/// ```
pub type RepeatLineWidth<'a> = RepeatFunction<NonEmpty<Vec<'a, LineWidth<'a>>>, AutoOr<PositiveNonZeroInt>>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, RepeatLineWidth, "repeat(2,12px)");
		assert_parse!(CssAtomSet::ATOMS, RepeatLineWidth, "repeat(auto,15rem)");
		assert_parse!(CssAtomSet::ATOMS, RepeatLineWidth, "repeat(2,12px 15px 18px)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, RepeatLineWidth, "repeat(none, 12px)");
	}
}
