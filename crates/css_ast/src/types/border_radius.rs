use super::prelude::*;
use crate::{CalcableValue, LengthPercentage};

/// <https://drafts.csswg.org/css-borders-4/#typedef-border-radius>
///
/// ```text,ignore
/// <border-radius> = <slash-separated-border-radius-syntax> | <legacy-border-radius-syntax>
/// <slash-separated-border-radius-syntax> = <length-percentage [0,∞]> [ / <length-percentage [0,∞]> ]?
/// <legacy-border-radius-syntax> = <length-percentage [0,∞]>{1,2}
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct BorderRadius<'a>(
	pub CalcableValue<'a, LengthPercentage>,
	pub Option<T![/]>,
	pub Option<CalcableValue<'a, LengthPercentage>>,
);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BorderRadius, "1px");
		assert_parse!(CssAtomSet::ATOMS, BorderRadius, "1px 2px");
		assert_parse!(CssAtomSet::ATOMS, BorderRadius, "1px / 2px");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, BorderRadius, "var(--r)");
		assert_parse!(CssAtomSet::ATOMS, BorderRadius, "calc(1px + 2px)");
		assert_parse!(CssAtomSet::ATOMS, BorderRadius, "1px / var(--y)");
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
