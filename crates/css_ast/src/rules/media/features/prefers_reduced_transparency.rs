use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum PrefersReducedTransparencyMediaFeature<"prefers-reduced-transparency", PrefersReducedTransparencyMediaFeatureKeyword>
);

keyword_set!(pub enum PrefersReducedTransparencyMediaFeatureKeyword { NoPreference: "no-preference", Reduce: "reduce" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PrefersReducedTransparencyMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersReducedTransparencyMediaFeature, "(prefers-reduced-transparency)");
		assert_parse!(PrefersReducedTransparencyMediaFeature, "(prefers-reduced-transparency:no-preference)");
		assert_parse!(PrefersReducedTransparencyMediaFeature, "(prefers-reduced-transparency:reduce)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersReducedTransparencyMediaFeature, "(prefers-reduced-transparency:)");
		assert_parse_error!(PrefersReducedTransparencyMediaFeature, "(prefers-reduced-transparency: reduced)");
	}
}
