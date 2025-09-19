use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum ScanMediaFeature<"scan", ScanMediaFeatureKeyword>
);

keyword_set!(pub enum ScanMediaFeatureKeyword { Interlace: "interlace", Progressive: "progressive" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ScanMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(ScanMediaFeature, "(scan)");
		assert_parse!(ScanMediaFeature, "(scan:interlace)");
		assert_parse!(ScanMediaFeature, "(scan:progressive)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(ScanMediaFeature, "(scan:)");
		assert_parse_error!(ScanMediaFeature, "(scan: landscope)");
	}
}
