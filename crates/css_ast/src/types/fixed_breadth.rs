use crate::LengthPercentage;

/// <https://drafts.csswg.org/css-grid-2/#typedef-fixed-breadth>
///
/// ```text,ignore
/// <fixed-breadth> = <length-percentage [0,∞]>
/// ```
pub type FixedBreadth = crate::NonNegative<LengthPercentage>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FixedBreadth, "10px");
		assert_parse!(CssAtomSet::ATOMS, FixedBreadth, "50%");
	}
}
