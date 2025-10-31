use super::prelude::*;

use crate::StripesFunction;

// https://drafts.csswg.org/css-images-4/#typedef-image-1d
// <image-1D> = <stripes()>
// <stripes()> = stripes( <color-stripe># )
// <color-stripe> = <color> && [ <length-percentage> | <flex> ]?
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct Image1d<'a>(StripesFunction<'a>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Image1d>(), 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Image1d, "stripes(red 1fr,green 2fr,blue 100px)");
	}
}
