use crate::{LineWidthList, LineWidthOrRepeat, RepeatLineWidth};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-width-list>
///
/// ```text,ignore
/// <auto-line-width-list> = [ <line-width-or-repeat> ]* <auto-repeat-line-width> [ <line-width-or-repeat> ]*
/// ```
pub type AutoLineWidthList<'a> = LineWidthList<'a>;
pub type AutoLineWidthListItem<'a> = LineWidthOrRepeat<'a>;
pub type AutoRepeatLineWidth<'a> = RepeatLineWidth<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AutoLineWidthList, "repeat(auto,12px)");
		assert_parse!(CssAtomSet::ATOMS, AutoLineWidthList, "thin repeat(auto,12px)");
		assert_parse!(CssAtomSet::ATOMS, AutoLineWidthList, "repeat(auto,12px) thick");
		assert_parse!(CssAtomSet::ATOMS, AutoLineWidthList, "thin repeat(2,1px) repeat(auto,12px) 3px");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, AutoLineWidthList, "");
		assert_peek_false!(CssAtomSet::ATOMS, AutoLineWidthList, "florp");
		assert_parse_error!(CssAtomSet::ATOMS, AutoLineWidthList, "repeat(auto,)");
	}
}
