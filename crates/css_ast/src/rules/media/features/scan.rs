use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum ScanMediaFeature{CssAtomSet::Scan, ScanMediaFeatureKeyword}
);

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum ScanMediaFeatureKeyword {
	#[atom(CssAtomSet::Interlace)]
	Interlace(T![Ident]),
	#[atom(CssAtomSet::Progressive)]
	Progressive(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ScanMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ScanMediaFeature, "(scan)");
		assert_parse!(CssAtomSet::ATOMS, ScanMediaFeature, "(scan:interlace)");
		assert_parse!(CssAtomSet::ATOMS, ScanMediaFeature, "(scan:progressive)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ScanMediaFeature, "(scan:)");
		assert_parse_error!(CssAtomSet::ATOMS, ScanMediaFeature, "(scan: landscope)");
	}
}
