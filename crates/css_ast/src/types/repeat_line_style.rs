use super::prelude::*;
use crate::{AutoOr, LineStyle, NonEmpty, PositiveNonZeroInt, RepeatFunction};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-repeat-line-style>
///
/// ```text,ignore
/// <repeat-line-style>        = repeat( [ <integer [1,∞]> ] , [ <line-style> ]+ )
/// <auto-repeat-line-style>   = repeat( auto , [ <line-style> ]+ )
/// ```
pub type RepeatLineStyle<'a> = RepeatFunction<NonEmpty<Vec<'a, LineStyle>>, AutoOr<PositiveNonZeroInt>>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, RepeatLineStyle, "repeat(2,solid)");
		assert_parse!(CssAtomSet::ATOMS, RepeatLineStyle, "repeat(3,solid dashed dotted)");
		assert_parse!(CssAtomSet::ATOMS, RepeatLineStyle, "repeat(auto,solid)");
		assert_parse!(CssAtomSet::ATOMS, RepeatLineStyle, "repeat(auto,solid dashed)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, RepeatLineStyle, "repeat(none,solid)");
		assert_parse_error!(CssAtomSet::ATOMS, RepeatLineStyle, "repeat(2,)");
		assert_parse_error!(CssAtomSet::ATOMS, RepeatLineStyle, "repeat(auto,)");
		assert_parse_error!(CssAtomSet::ATOMS, RepeatLineStyle, "repeat(0,solid)");
	}
}
