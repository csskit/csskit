use super::prelude::*;
use crate::LengthPercentage;

/// <https://drafts.csswg.org/css-borders-4/#typedef-border-radius>
///
/// ```text,ignore
/// <border-radius> = <slash-separated-border-radius-syntax> | <legacy-border-radius-syntax>
/// <slash-separated-border-radius-syntax> = <length-percentage [0,∞]> [ / <length-percentage [0,∞]> ]?
/// <legacy-border-radius-syntax> = <length-percentage [0,∞]>{1,2}
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct BorderRadius(pub LengthPercentage, pub Option<T![/]>, pub Option<LengthPercentage>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BorderRadius>(), 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BorderRadius, "1px");
		assert_parse!(CssAtomSet::ATOMS, BorderRadius, "1px 2px");
		assert_parse!(CssAtomSet::ATOMS, BorderRadius, "1px / 2px");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("12%", BorderRadius, LengthPercentage);
		assert_visits!("12% 10px", BorderRadius, LengthPercentage, LengthPercentage, Length);
		assert_visits!("12% / 10px", BorderRadius, LengthPercentage, LengthPercentage, Length);
	}
}
