#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_literal_colors() {
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "#fff");
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "#ffffff");
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "rgb(255, 0, 0)");
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "hsl(0, 100%, 50%)");
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "currentcolor");
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "transparent");
	}

	#[test]
	fn test_color_var_support() {
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "var(--color)");
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "var(--primary, red)");
		assert_parse!(CssAtomSet::ATOMS, ColorStyleValue, "var(--bg, #fff)");
	}

	#[test]
	fn test_opacity() {
		// Literal numbers
		assert_parse!(CssAtomSet::ATOMS, OpacityStyleValue, "1");
		assert_parse!(CssAtomSet::ATOMS, OpacityStyleValue, "0");
		assert_parse!(CssAtomSet::ATOMS, OpacityStyleValue, "0.5");
	}

	#[test]
	fn test_opacity_substitution() {
		assert_parse!(CssAtomSet::ATOMS, OpacityStyleValue, "var(--o)");
		assert_parse!(CssAtomSet::ATOMS, OpacityStyleValue, "var(--o, 0.5)");
		assert_parse!(CssAtomSet::ATOMS, OpacityStyleValue, "calc(1 / 2)");
	}
}
