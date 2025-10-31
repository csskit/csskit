use super::prelude::*;

/// <https://drafts.csswg.org/css-backgrounds-3/#typedef-bg-size>
///
/// ```text,ignore
/// <bg-size> = [ <length-percentage [0,∞]> | auto ]{1,2} | cover | contain
/// ```
#[syntax(" [ <length-percentage [0,∞]> | auto ]{1,2} | cover | contain ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum BgSize {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BgSize>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BgSize, "cover", BgSize::Cover(_));
		assert_parse!(CssAtomSet::ATOMS, BgSize, "contain", BgSize::Contain(_));
		assert_parse!(CssAtomSet::ATOMS, BgSize, "12%", BgSize::AutoOrLengthPercentage(_, _));
		assert_parse!(CssAtomSet::ATOMS, BgSize, "auto auto", BgSize::AutoOrLengthPercentage(_, _));
		assert_parse!(CssAtomSet::ATOMS, BgSize, "12% 10px", BgSize::AutoOrLengthPercentage(_, _));
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("12%", BgSize, LengthPercentage);
		assert_visits!("12% 10px", BgSize, LengthPercentage, LengthPercentage, Length);
		assert_visits!("cover", BgSize);
	}
}
