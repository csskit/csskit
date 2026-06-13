#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<IsolationStyleValue>(), 24);
		assert_eq!(std::mem::size_of::<MixBlendModeStyleValue>(), 24);
	}

	#[test]
	fn test_background_blend_mode() {
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "multiply");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "screen");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "overlay");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "darken");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "lighten");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "color-dodge");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "color-burn");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "hard-light");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "soft-light");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "difference");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "exclusion");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "hue");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "saturation");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "luminosity");
		assert_parse!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "normal, luminosity");
	}

	#[test]
	fn test_background_blend_mode_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, BackgroundBlendModeStyleValue, "normal luminosity");
	}

	#[test]
	fn test_isolation() {
		assert_parse!(CssAtomSet::ATOMS, IsolationStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, IsolationStyleValue, "isolate");
	}

	#[test]
	fn test_isolation_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, IsolationStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, IsolationStyleValue, "auto isolate");
	}

	#[test]
	fn test_mix_blend_mode() {
		assert_parse!(CssAtomSet::ATOMS, MixBlendModeStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, MixBlendModeStyleValue, "multiply");
		assert_parse!(CssAtomSet::ATOMS, MixBlendModeStyleValue, "screen");
		assert_parse!(CssAtomSet::ATOMS, MixBlendModeStyleValue, "difference");
		assert_parse!(CssAtomSet::ATOMS, MixBlendModeStyleValue, "luminosity");
	}

	#[test]
	fn test_mix_blend_mode_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, MixBlendModeStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, MixBlendModeStyleValue, "normal luminosity");
		assert_parse_error!(CssAtomSet::ATOMS, MixBlendModeStyleValue, "normal, luminosity");
	}
}
