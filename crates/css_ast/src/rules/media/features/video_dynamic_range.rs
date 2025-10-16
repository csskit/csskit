use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum VideoDynamicRangeMediaFeature{CssAtomSet::VideoDynamicRange, VideoDynamicRangeMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum VideoDynamicRangeMediaFeatureKeyword {
	#[atom(CssAtomSet::Standard)]
	Standard(T![Ident]),
	#[atom(CssAtomSet::High)]
	Hight(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<VideoDynamicRangeMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, VideoDynamicRangeMediaFeature, "(video-dynamic-range)");
		assert_parse!(CssAtomSet::ATOMS, VideoDynamicRangeMediaFeature, "(video-dynamic-range:standard)");
		assert_parse!(CssAtomSet::ATOMS, VideoDynamicRangeMediaFeature, "(video-dynamic-range:high)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, VideoDynamicRangeMediaFeature, "(video-dynamic-range:)");
		assert_parse_error!(CssAtomSet::ATOMS, VideoDynamicRangeMediaFeature, "(video-dynamic-range: low)");
	}
}
