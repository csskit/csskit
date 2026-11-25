#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, EnvironmentStyleValue, "--sky");
		assert_parse!(CssAtomSet::ATOMS, EnvironmentStyleValue, "none 0.5 90deg,--sky 2");
		assert_parse!(CssAtomSet::ATOMS, EnvironmentMapStyleValue, "none,--sky");
		assert_parse!(CssAtomSet::ATOMS, EnvironmentIntensityStyleValue, "1,0.5");
		assert_parse!(CssAtomSet::ATOMS, EnvironmentRotateStyleValue, "90deg");
		assert_parse!(CssAtomSet::ATOMS, FrontLimitStyleValue, "max");
		assert_parse!(CssAtomSet::ATOMS, FrontLimitStyleValue, "10px");
		assert_parse!(CssAtomSet::ATOMS, InsetFrontStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, InsetLeftStyleValue, "10%");
		assert_parse!(CssAtomSet::ATOMS, PortalStageStyleValue, "auto 10px/20px");
		assert_parse!(CssAtomSet::ATOMS, PositionContextStyleValue, "anchor");
		assert_parse!(CssAtomSet::ATOMS, SpatialStyleValue, "portal");
		assert_parse!(CssAtomSet::ATOMS, ZAlignItemsStyleValue, "safe front");
		assert_parse!(CssAtomSet::ATOMS, ZAlignSelfStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ZAlignSelfStyleValue, "back");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, EnvironmentIntensityStyleValue, "-1");
		assert_parse_error!(CssAtomSet::ATOMS, EnvironmentMapStyleValue, "none --sky");
		assert_parse_error!(CssAtomSet::ATOMS, FrontLimitStyleValue, "-1px");
		assert_parse_error!(CssAtomSet::ATOMS, ZAlignItemsStyleValue, "safe");
	}
}
