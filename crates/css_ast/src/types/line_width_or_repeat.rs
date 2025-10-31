use super::prelude::*;

/// <https://drafts.csswg.org/css-gaps-1/#typedef-line-width-or-repeat>
///
/// ```text,ignore
/// <line-width-or-repeat> = [ <line-width> | <repeat-line-width> ]
/// ```
#[syntax(" <line-width> | <repeat()> ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum LineWidthOrRepeat<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use crate::LineWidth;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LineWidthOrRepeat>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LineWidthOrRepeat, "repeat(2,12px)", LineWidthOrRepeat::RepeatFunction(_));
		assert_parse!(CssAtomSet::ATOMS, LineWidthOrRepeat, "thin", LineWidthOrRepeat::LineWidth(LineWidth::Thin(_)));
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, LineWidthOrRepeat, "repeat(none, 12px)");
	}
}
