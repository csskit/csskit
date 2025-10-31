use super::prelude::*;
use crate::units::CSSInt;

ranged_feature!(
	#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum MonochromeMediaFeature{CssAtomSet::Monochrome | CssAtomSet::MinMonochrome | CssAtomSet::MaxMonochrome, CSSInt}
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<MonochromeMediaFeature>(), 116);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome:2)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome:8)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(min-monochrome:2)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(max-monochrome:2)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome<=3)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome>=5)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome>=8)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome=16)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(6=monochrome)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(2<=monochrome)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(2<monochrome<4)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(4>monochrome<8)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(4>=monochrome<=8)");
		assert_parse!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(4<=monochrome>8)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome:)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(max-monochrome > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(min-monochrome > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome: 1px)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(monochrome: red)");
		assert_parse_error!(CssAtomSet::ATOMS, MonochromeMediaFeature, "(pointer: 1)");
	}
}
