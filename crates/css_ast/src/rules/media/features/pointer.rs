use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum PointerMediaFeature<"pointer", PointerMediaFeatureKeyword>
);

keyword_set!(pub enum PointerMediaFeatureKeyword { None: "none", Coarse: "coarse", Fine: "fine" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<PointerMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(PointerMediaFeature, "(pointer)");
		assert_parse!(PointerMediaFeature, "(pointer:none)");
		assert_parse!(PointerMediaFeature, "(pointer:coarse)");
		assert_parse!(PointerMediaFeature, "(pointer:fine)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(PointerMediaFeature, "(pointer:)");
		assert_parse_error!(PointerMediaFeature, "(pointer: pointer)");
	}
}
