use crate::{LineColorList, LineColorOrRepeat, RepeatLineColor};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-color-list>
///
/// ```text,ignore
/// <auto-line-color-list> = [ <line-color-or-repeat> ]* <auto-repeat-line-color> [ <line-color-or-repeat> ]*
/// ```
pub type AutoLineColorList<'a> = LineColorList<'a>;
pub type AutoLineColorListItem<'a> = LineColorOrRepeat<'a>;
pub type AutoRepeatLineColor<'a> = RepeatLineColor<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AutoLineColorList, "repeat(auto,red)");
		assert_parse!(CssAtomSet::ATOMS, AutoLineColorList, "red repeat(auto,green)");
		assert_parse!(CssAtomSet::ATOMS, AutoLineColorList, "repeat(auto,green) blue");
		assert_parse!(CssAtomSet::ATOMS, AutoLineColorList, "red repeat(2,green) repeat(auto,blue) currentcolor");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, AutoLineColorList, "");
		assert_peek_false!(CssAtomSet::ATOMS, AutoLineColorList, "florp");
		assert_parse_error!(CssAtomSet::ATOMS, AutoLineColorList, "repeat(auto,)");
	}
}
