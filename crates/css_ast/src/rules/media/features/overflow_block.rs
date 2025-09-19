use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum OverflowBlockMediaFeature<"overflow-block", OverflowBlockMediaFeatureKeyword>
);

keyword_set!(pub enum OverflowBlockMediaFeatureKeyword { None: "none", Scroll: "scroll", Paged: "paged" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OverflowBlockMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OverflowBlockMediaFeature, "(overflow-block)");
		assert_parse!(OverflowBlockMediaFeature, "(overflow-block:none)");
		assert_parse!(OverflowBlockMediaFeature, "(overflow-block:scroll)");
		assert_parse!(OverflowBlockMediaFeature, "(overflow-block:paged)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(OverflowBlockMediaFeature, "(overflow-block:)");
		assert_parse_error!(OverflowBlockMediaFeature, "(overflow-block: page)");
	}
}
