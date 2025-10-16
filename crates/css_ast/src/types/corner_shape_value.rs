use super::prelude::*;

/// <https://drafts.csswg.org/css-borders-4/#typedef-corner-shape-value>
///
/// ```text,ignore
/// <corner-shape-value> = round | scoop | bevel | notch | square | squircle | <superellipse()>
/// ```
#[syntax(" round | scoop | bevel | notch | square | squircle | <superellipse()> ")]
#[derive(Peek, Parse, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum CornerShapeValue {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CornerShapeValue>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CornerShapeValue, "square", CornerShapeValue::Square(_));
		assert_parse!(CssAtomSet::ATOMS, CornerShapeValue, "squircle", CornerShapeValue::Squircle(_));
		assert_parse!(CssAtomSet::ATOMS, CornerShapeValue, "superellipse(-infinity)");
		assert_parse!(CssAtomSet::ATOMS, CornerShapeValue, "superellipse(1000)");
	}
}
