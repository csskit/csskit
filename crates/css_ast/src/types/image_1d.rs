use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

use crate::StripesFunction;

// https://drafts.csswg.org/css-images-4/#typedef-image-1d
// <image-1D> = <stripes()>
// <stripes()> = stripes( <color-stripe># )
// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct Image1D<'a>(StripesFunction<'a>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Image1D>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Image1D, "stripes(red 1fr,green 2fr,blue 100px)");
		assert_parse!(
			Image1D,
			"stripes(0.1fr red,0.2fr green,100px blue)",
			"stripes(red 0.1fr,green 0.2fr,blue 100px)"
		);
		assert_parse!(Image1D, "stripes(red 1fr,2fr green,blue 100px)", "stripes(red 1fr,green 2fr,blue 100px)");
	}
}
