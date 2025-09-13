use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#font-weight-absolute-values>
///
/// ```text,ignore
/// <font-weight-absolute> = [normal | bold | <number [1,1000]>]
/// ```
#[syntax(" normal | bold | <number [1,1000]> ")]
#[derive(IntoCursor, Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum FontWeightAbsolute {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FontWeightAbsolute>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FontWeightAbsolute, "normal");
		assert_parse!(CssAtomSet::ATOMS, FontWeightAbsolute, "bold");
		assert_parse!(CssAtomSet::ATOMS, FontWeightAbsolute, "100");
		assert_parse!(CssAtomSet::ATOMS, FontWeightAbsolute, "500");
		assert_parse!(CssAtomSet::ATOMS, FontWeightAbsolute, "900");
		assert_parse!(CssAtomSet::ATOMS, FontWeightAbsolute, "900.5");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FontWeightAbsolute, "1000.1");
	}
}
