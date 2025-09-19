use super::prelude::*;

discrete_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum AnyHoverMediaFeature<"any-hover", AnyHoverMediaFeatureKeyword>
);

keyword_set!(pub enum AnyHoverMediaFeatureKeyword { None: "none", Hover: "hover" });

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AnyHoverMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AnyHoverMediaFeature, "(any-hover)");
		assert_parse!(AnyHoverMediaFeature, "(any-hover:hover)");
		assert_parse!(AnyHoverMediaFeature, "(any-hover:none)");
	}
}
