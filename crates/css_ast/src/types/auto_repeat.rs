use crate::{AutoFillOrFit, FixedSize, NamedRepeatItems, RepeatFunction};

/// <https://drafts.csswg.org/css-grid-2/#typedef-auto-repeat>
///
/// ```text,ignore
/// <auto-repeat> = repeat( [ auto-fill | auto-fit ] , [ <line-names>? <fixed-size> ]+ <line-names>? )
/// ```
pub type AutoRepeat<'a> = RepeatFunction<NamedRepeatItems<'a, FixedSize<'a>>, AutoFillOrFit>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AutoRepeat, "repeat(auto-fill,10px)");
		assert_parse!(CssAtomSet::ATOMS, AutoRepeat, "repeat(auto-fit,[a] 10px)");
		assert_parse!(CssAtomSet::ATOMS, AutoRepeat, "repeat(auto-fit,[a] 10px [b])");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, AutoRepeat, "repeat(2,10px)");
		assert_parse_error!(CssAtomSet::ATOMS, AutoRepeat, "repeat(auto-fill,)");
	}
}
