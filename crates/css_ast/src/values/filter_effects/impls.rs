#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColorInterpolationFiltersStyleValue, "auto");
		assert_parse!(CssAtomSet::ATOMS, ColorInterpolationFiltersStyleValue, "srgb");
		assert_parse!(CssAtomSet::ATOMS, ColorInterpolationFiltersStyleValue, "linearrgb");

		assert_parse!(CssAtomSet::ATOMS, FloodColorStyleValue, "currentcolor");
		assert_parse!(CssAtomSet::ATOMS, FloodColorStyleValue, "red");
		assert_parse!(CssAtomSet::ATOMS, FloodColorStyleValue, "transparent");

		// flood-opacity uses <'opacity'> = <number [0,1]>
		assert_parse!(CssAtomSet::ATOMS, FloodOpacityStyleValue, "0.5");
		assert_parse!(CssAtomSet::ATOMS, FloodOpacityStyleValue, "1");

		assert_parse!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "blur(100px)");
		assert_parse!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "brightness(0)");
		assert_parse!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "contrast(300%)");
		assert_parse!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "grayscale(0)");
		assert_parse!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "hue-rotate(90deg)");
		assert_parse!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "invert(0)");
		assert_parse!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "opacity(0)");
		assert_parse!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "saturate(300%)");
		assert_parse!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "sepia(0)");

		assert_parse!(CssAtomSet::ATOMS, FilterStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, FilterStyleValue, "blur(100px)");
		assert_parse!(CssAtomSet::ATOMS, FilterStyleValue, "brightness(0)");
		assert_parse!(CssAtomSet::ATOMS, FilterStyleValue, "contrast(300%)");
		assert_parse!(CssAtomSet::ATOMS, FilterStyleValue, "grayscale(0)");
		assert_parse!(CssAtomSet::ATOMS, FilterStyleValue, "hue-rotate(90deg)");
		assert_parse!(CssAtomSet::ATOMS, FilterStyleValue, "invert(0)");
		assert_parse!(CssAtomSet::ATOMS, FilterStyleValue, "opacity(0)");
		assert_parse!(CssAtomSet::ATOMS, FilterStyleValue, "saturate(300%)");
		assert_parse!(CssAtomSet::ATOMS, FilterStyleValue, "sepia(0)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ColorInterpolationFiltersStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, ColorInterpolationFiltersStyleValue, "linearrgb srgb");

		assert_peek_false!(CssAtomSet::ATOMS, FloodColorStyleValue, "none");
		assert_parse_error!(CssAtomSet::ATOMS, FloodColorStyleValue, "black white");

		assert_parse_error!(CssAtomSet::ATOMS, FloodOpacityStyleValue, "2 3");

		assert_peek_false!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "blur(10)");
		assert_parse_error!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "blur(-100px)");
		assert_parse_error!(CssAtomSet::ATOMS, BackdropFilterStyleValue, "hue-rotate(90)");

		assert_peek_false!(CssAtomSet::ATOMS, FilterStyleValue, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, FilterStyleValue, "blur(10)");
		assert_parse_error!(CssAtomSet::ATOMS, FilterStyleValue, "blur(-100px)");
		assert_parse_error!(CssAtomSet::ATOMS, FilterStyleValue, "hue-rotate(90)");
	}
}
