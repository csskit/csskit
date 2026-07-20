use crate::{FixedSize, NamedRepeatItems, RepeatFunction};

/// <https://drafts.csswg.org/css-grid-2/#typedef-fixed-repeat>
///
/// ```text,ignore
/// <fixed-repeat> = repeat( [ <integer [1,∞]> ] , [ <line-names>? <fixed-size> ]+ <line-names>? )
/// ```
pub type FixedRepeat<'a> = RepeatFunction<NamedRepeatItems<'a, FixedSize>>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FixedRepeat>(), 120);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FixedRepeat, "repeat(2,10px)");
		assert_parse!(CssAtomSet::ATOMS, FixedRepeat, "repeat(3,[a] 10px)");
		assert_parse!(CssAtomSet::ATOMS, FixedRepeat, "repeat(3,10px [a])");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FixedRepeat, "repeat(2,)");
		assert_parse_error!(CssAtomSet::ATOMS, FixedRepeat, "repeat(2,1fr)");
	}
}
