// use super::types::LengthPercentage;
// use super::{MinWidthStyleValue, Width};

// shortcuts for logical properties to resolve to 0
// impl Width {
// 	#[allow(non_upper_case_globals)]
// 	pub const Zero: Width = Width::LengthPercentage(LengthPercentage::Zero();
// }
//
// impl MinWidth {
// 	#[allow(non_upper_case_globals)]
// 	pub const Zero: MinWidth = MinWidth::LengthPercentage(LengthPercentage::Zero);
// }

#[cfg(test)]
mod tests {
	use super::super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<WidthStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<HeightStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<MinWidthStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<MinHeightStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<MaxWidthStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<MaxHeightStyleValue>(), 44);
		assert_eq!(std::mem::size_of::<BoxSizingStyleValue>(), 16);
		assert_eq!(std::mem::size_of::<AspectRatioStyleValue>(), 60);
	}

	#[test]
	fn test_writes() {
		assert_parse!(WidthStyleValue, "0");
		assert_parse!(WidthStyleValue, "1px");
		assert_parse!(WidthStyleValue, "fit-content");
		assert_parse!(WidthStyleValue, "fit-content(20rem)");
		assert_parse!(WidthStyleValue, "fit-content(0)");
		assert_parse!(WidthStyleValue, "stretch");
		assert_parse!(WidthStyleValue, "contain");

		assert_parse!(AspectRatioStyleValue, "auto 1/5");
		assert_parse!(AspectRatioStyleValue, "auto 1/2");
		assert_parse!(AspectRatioStyleValue, "1/3 auto", "auto 1/3");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(AspectRatioStyleValue, "auto auto");
		assert_parse_error!(AspectRatioStyleValue, "1/2 1/2");
	}
}
