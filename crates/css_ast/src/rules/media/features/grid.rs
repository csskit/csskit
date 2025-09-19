use super::prelude::*;

boolean_feature!(
	#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
	#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
	pub enum GridMediaFeature<"grid">
);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<GridMediaFeature>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(GridMediaFeature, "(grid:1)");
		assert_parse!(GridMediaFeature, "(grid)");
	}
}
