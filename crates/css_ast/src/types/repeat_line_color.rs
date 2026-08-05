use super::prelude::*;
use crate::{AutoOr, Color, NonEmpty, PositiveNonZeroInt, RepeatFunction};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-repeat-line-color>
///
/// ```text,ignore
/// <repeat-line-color>        = repeat( [ <integer [1,∞]> ] , [ <color> ]+ )
/// <auto-repeat-line-color>   = repeat( auto , [ <color> ]+ )
/// ```
pub type RepeatLineColor<'a> = RepeatFunction<NonEmpty<Vec<'a, Color<'a>>>, AutoOr<PositiveNonZeroInt>>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, RepeatLineColor, "repeat(2,red)");
		assert_parse!(CssAtomSet::ATOMS, RepeatLineColor, "repeat(3,red #00ff00 rgb(0,0,255))");
		assert_parse!(CssAtomSet::ATOMS, RepeatLineColor, "repeat(auto,red)");
		assert_parse!(CssAtomSet::ATOMS, RepeatLineColor, "repeat(auto,red green)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, RepeatLineColor, "repeat(none,red)");
		assert_parse_error!(CssAtomSet::ATOMS, RepeatLineColor, "repeat(2,)");
		assert_parse_error!(CssAtomSet::ATOMS, RepeatLineColor, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, RepeatLineColor, "repeat(0,red)");
	}
}
