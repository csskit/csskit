use super::prelude::*;

/// <https://drafts.csswg.org/css-borders-4/#typedef-corner-shape-value>
///
/// ```text,ignore
/// <corner-shape-value> = round | scoop | bevel | notch | square | squircle | <superellipse()>
/// ```
#[syntax(" round | scoop | bevel | notch | square | squircle | <superellipse()> ")]
#[derive(Peek, Parse, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum CornerShapeValue {}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CornerShapeValue>(), 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CornerShapeValue, "square", CornerShapeValue::Square(_));
		assert_parse!(CornerShapeValue, "squircle", CornerShapeValue::Squircle(_));
		assert_parse!(CornerShapeValue, "superellipse(-infinity)");
		assert_parse!(CornerShapeValue, "superellipse(1000)");
	}
}
