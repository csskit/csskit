use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum NavControlsMediaFeature<"nav-controls", NavControlsMediaFeatureKeyword>
);

keyword_set!(pub enum NavControlsMediaFeatureKeyword { None: "none", Back: "back" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NavControlsMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(NavControlsMediaFeature, "(nav-controls)");
		assert_parse!(NavControlsMediaFeature, "(nav-controls:back)");
		assert_parse!(NavControlsMediaFeature, "(nav-controls:none)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(NavControlsMediaFeature, "(nav-controls:)");
		assert_parse_error!(NavControlsMediaFeature, "(nav-controls: hoover)");
	}
}
