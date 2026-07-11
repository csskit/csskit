use crate::{NamedRepeatItems, RepeatFunction, TrackSize};

/// <https://drafts.csswg.org/css-grid-2/#typedef-track-repeat>
///
/// ```text,ignore
/// <track-repeat> = repeat( [ <integer [1,∞]> ] , [ <line-names>? <track-size> ]+ <line-names>? )
/// ```
pub type TrackRepeat<'a> = RepeatFunction<NamedRepeatItems<'a, TrackSize>>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TrackRepeat>(), 136);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, TrackRepeat, "repeat(2,10px)");
		assert_parse!(CssAtomSet::ATOMS, TrackRepeat, "repeat(3,[a] 10px)");
		assert_parse!(CssAtomSet::ATOMS, TrackRepeat, "repeat(3,10px [a])");
		assert_parse!(CssAtomSet::ATOMS, TrackRepeat, "repeat(2,10px 1fr)");
		assert_parse!(CssAtomSet::ATOMS, TrackRepeat, "repeat(2,[a] 10px [b] 1fr [c])");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TrackRepeat, "repeat(2,)");
		assert_parse_error!(CssAtomSet::ATOMS, TrackRepeat, "repeat(0,10px)");
	}
}
