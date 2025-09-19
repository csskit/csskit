use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum PrefersReducedDataMediaFeature<"prefers-reduced-data", PrefersReducedDataMediaFeatureKeyword>
);

keyword_set!(pub enum PrefersReducedDataMediaFeatureKeyword { NoPreference: "no-preference", Reduce: "reduce" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PrefersReducedDataMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersReducedDataMediaFeature, "(prefers-reduced-data)");
		assert_parse!(PrefersReducedDataMediaFeature, "(prefers-reduced-data:no-preference)");
		assert_parse!(PrefersReducedDataMediaFeature, "(prefers-reduced-data:reduce)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersReducedDataMediaFeature, "(prefers-reduced-data:)");
		assert_parse_error!(PrefersReducedDataMediaFeature, "(prefers-reduced-data: reduced)");
	}
}
