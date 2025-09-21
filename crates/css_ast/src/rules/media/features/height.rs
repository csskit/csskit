use super::prelude::*;
use crate::units::Length;

ranged_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum HeightMediaFeature<CssAtomSet::Height | CssAtomSet::MinHeight | CssAtomSet::MaxHeight, Length>
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<HeightMediaFeature>(), 124);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height:360px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(min-height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(max-height:35rem)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height<=800px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height>=1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(height=1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(1400px=height)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(100px<=height)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(100px<height<1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(100px>height<1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(100px>=height<=1400px)");
		assert_parse!(CssAtomSet::ATOMS, HeightMediaFeature, "(100px<=height>1400px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(height:)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(height: > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(max-height > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(min-height > 10px)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(height: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(height: 1%)");
		assert_parse_error!(CssAtomSet::ATOMS, HeightMediaFeature, "(pointer: 1px)");
	}
}
