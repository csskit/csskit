use crate::{CalcableValue, LengthPercentage, NonNegative};

/// <https://drafts.csswg.org/css-grid-2/#typedef-fixed-breadth>
///
/// ```text,ignore
/// <fixed-breadth> = <length-percentage [0,∞]>
/// ```
pub type FixedBreadth<'a> = CalcableValue<'a, NonNegative<LengthPercentage>>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FixedBreadth, "10px");
		assert_parse!(CssAtomSet::ATOMS, FixedBreadth, "50%");
		assert_parse!(CssAtomSet::ATOMS, FixedBreadth, "calc(10px + 2%)");
		assert_parse!(CssAtomSet::ATOMS, FixedBreadth, "var(--x)");
	}
}
