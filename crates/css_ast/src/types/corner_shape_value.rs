use super::prelude::*;

/// <https://drafts.csswg.org/css-borders-4/#typedef-corner-shape-value>
///
/// ```text,ignore
/// <corner-shape-value> = round | scoop | bevel | notch | square | squircle | <superellipse()>
/// ```
#[syntax(" round | scoop | bevel | notch | square | squircle | <superellipse()> ")]
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CornerShapeValue<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CornerShapeValue, "square", CornerShapeValue::Square(_));
		assert_parse!(CssAtomSet::ATOMS, CornerShapeValue, "squircle", CornerShapeValue::Squircle(_));
		assert_parse!(CssAtomSet::ATOMS, CornerShapeValue, "superellipse(-infinity)");
		assert_parse!(CssAtomSet::ATOMS, CornerShapeValue, "superellipse(1000)");
	}
}
