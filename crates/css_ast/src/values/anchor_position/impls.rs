pub(crate) use crate::{CssAtomSet, CssDiagnostic};
pub(crate) use csskit_derives::*;
pub(crate) use csskit_proc_macro::*;

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	pub fn size_test() {
		assert_eq!(std::mem::size_of::<AnchorNameStyleValue>(), 32);
		assert_eq!(std::mem::size_of::<AnchorScopeStyleValue>(), 40);
		assert_eq!(std::mem::size_of::<PositionAnchorStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<PositionAreaStyleValue>(), 36);
		assert_eq!(std::mem::size_of::<PositionVisibilityStyleValue>(), 48);
		// assert_eq!(std::mem::size_of::<PositionTryFallbacksStyleValue>(), 1);
		assert_eq!(std::mem::size_of::<PositionTryOrderStyleValue>(), 16);
		// assert_eq!(std::mem::size_of::<PositionTryStyleValue>(), 1);
	}

	#[test]
	fn test_position_visibility() {
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "always");
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "anchors-valid");
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "anchors-visible");
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "no-overflow");
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "anchors-valid anchors-visible");
		assert_parse!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "anchors-valid anchors-visible no-overflow");
		assert_parse_error!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "");
		assert_parse_error!(CssAtomSet::ATOMS, PositionVisibilityStyleValue, "none");
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
