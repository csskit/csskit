pub(crate) use crate::{CssAtomSet, CssDiagnostic};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<AnchorNameStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<AnchorScopeStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<PositionAnchorStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<PositionAreaStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<PositionVisibilityStyleValue>(), 48);
		assert_eq!(std::mem::size_of::<PositionTryFallbacksStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<PositionTryOrderStyleValue>(), 24);
		// assert_eq!(std::mem::size_of::<PositionTryStyleValue>(), 1);
	}

	#[test]
	fn test_position_visibility() {
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "always");
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "anchor-valid");
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "anchor-visible");
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "no-overflow");
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "anchor-valid anchor-visible");
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "anchor-valid anchor-visible no-overflow");
		assert_peek_false!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "none");
	}

	#[test]
	fn test_position_try_fallbacks() {
		assert_parse!(CssAtomSet::ATOMS, PositionTryFallbacksStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, PositionTryFallbacksStyleValue, "--my-fallback");
		assert_parse!(CssAtomSet::ATOMS, PositionTryFallbacksStyleValue, "flip-block");
		assert_parse!(CssAtomSet::ATOMS, PositionTryFallbacksStyleValue, "flip-inline flip-block");
		assert_parse!(CssAtomSet::ATOMS, PositionTryFallbacksStyleValue, "--my-fallback flip-block");
		assert_parse!(CssAtomSet::ATOMS, PositionTryFallbacksStyleValue, "top");
		assert_parse!(CssAtomSet::ATOMS, PositionTryFallbacksStyleValue, "--a,--b,flip-block");
		assert_peek_false!(CssAtomSet::ATOMS, PositionTryFallbacksStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, PositionTryFallbacksStyleValue, "auto");
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AnchorNameStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, AnchorNameStyleValue, "--foo,--bar");
		assert_parse!(CssAtomSet::ATOMS, AnchorScopeStyleValue, "all");
		assert_parse!(CssAtomSet::ATOMS, AnchorScopeStyleValue, "--foo,--bar");
		assert_parse!(CssAtomSet::ATOMS, PositionTryOrderStyleValue, "normal");
	}
}
