use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::{Image, NoneKeyword};

// https://drafts.csswg.org/css-backgrounds/#typedef-bg-image
// <bg-image> = <image> | none
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum BgImage<'a> {
	None(NoneKeyword),
	Image(Image<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BgImage>(), 216);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BgImage, "none");
		assert_parse!(BgImage, "url(foo)");
	}
}
