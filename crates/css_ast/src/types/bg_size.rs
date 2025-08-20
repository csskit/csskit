use css_parse::keyword_set;
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::LengthPercentageOrAuto;

/// <https://drafts.csswg.org/css-backgrounds-3/#typedef-bg-size>
///
/// ```text,ignore
/// <bg-size> = [ <length-percentage [0,∞]> | auto ]{1,2} | cover | contain
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum BgSize {
	LengthPercentage(LengthPercentageOrAuto, Option<LengthPercentageOrAuto>),
	Cover(CoverKeyword),
	Contain(ContainKeyword),
}

keyword_set!(pub struct CoverKeyword "cover");
keyword_set!(pub struct ContainKeyword "contain");

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BgSize>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BgSize, "cover");
		assert_parse!(BgSize, "contain");
		assert_parse!(BgSize, "12%");
		assert_parse!(BgSize, "auto auto");
		assert_parse!(BgSize, "12% 10px");
	}
}
