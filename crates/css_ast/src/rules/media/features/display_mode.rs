use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum DisplayModeMediaFeature<"display-mode", DisplayModeMediaFeatureKeyword>
);

keyword_set!(pub enum DisplayModeMediaFeatureKeyword {
	Fullscreen: "fullscreen",
	Standalone: "standalone",
	MinimalUi: "minimal-ui",
	Browser: "browser",
	PictureInPicture: "picture-in-picture",
});

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DisplayModeMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DisplayModeMediaFeature, "(display-mode)");
		assert_parse!(DisplayModeMediaFeature, "(display-mode:fullscreen)");
		assert_parse!(DisplayModeMediaFeature, "(display-mode:minimal-ui)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(DisplayModeMediaFeature, "(display-mode:)");
		assert_parse_error!(DisplayModeMediaFeature, "(display-mode: pointer)");
		assert_parse_error!(DisplayModeMediaFeature, "(pointer: standalone)");
	}
}
