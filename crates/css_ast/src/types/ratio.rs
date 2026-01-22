use super::prelude::*;
use crate::units::CSSInt;

// https://drafts.csswg.org/css-values-4/#ratios
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Ratio {
	pub numerator: CSSInt,
	pub slash: Option<T![/]>,
	pub denominator: Option<CSSInt>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Ratio>(), 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Ratio, "1/1");
		assert_parse!(CssAtomSet::ATOMS, Ratio, "5/3");
		assert_parse!(CssAtomSet::ATOMS, Ratio, "5");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Ratio, "5 : 3");
		assert_parse_error!(CssAtomSet::ATOMS, Ratio, "5 / 1 / 1");
	}

	// #[cfg(feature = "serde")]
	// #[test]
	// fn test_serializes() {
	// 	assert_json!(Ratio, "5/3", {
	// 		"node": [5, 3],
	// 		"start": 0,
	// 		"end": 5
	// 	});
	// }
}
