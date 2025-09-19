use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum PrefersColorSchemeMediaFeature<"prefers-color-scheme", PrefersColorSchemeMediaFeatureKeyword>
);

keyword_set!(pub enum PrefersColorSchemeMediaFeatureKeyword { Light: "light", Dark: "dark" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PrefersColorSchemeMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PrefersColorSchemeMediaFeature, "(prefers-color-scheme)");
		assert_parse!(PrefersColorSchemeMediaFeature, "(prefers-color-scheme:light)");
		assert_parse!(PrefersColorSchemeMediaFeature, "(prefers-color-scheme:dark)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PrefersColorSchemeMediaFeature, "(prefers-color-scheme:)");
		assert_parse_error!(PrefersColorSchemeMediaFeature, "(prefers-color-scheme: dimmed)");
	}
}
