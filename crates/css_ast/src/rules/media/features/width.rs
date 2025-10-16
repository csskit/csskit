use super::prelude::*;
use crate::units::Length;

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum WidthMediaFeature{CssAtomSet::Width | CssAtomSet::MinWidth | CssAtomSet::MaxWidth, Length}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WidthMediaFeature>(), 124);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width:360px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(min-width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(max-width:35rem)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width<=800px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(width=1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(1400px=width)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px<=width)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px<width<1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px>width<1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px>=width<=1400px)");
		assert_parse!(CssAtomSet::ATOMS, WidthMediaFeature, "(100px<=width>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(width:)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(width: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(max-width > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(min-width > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(width: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(width: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, WidthMediaFeature, "(pointer: 1px)");
	}
}
