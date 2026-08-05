use crate::{LineStyleList, LineStyleOrRepeat, RepeatLineStyle};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-style-list>
///
/// ```text,ignore
/// <auto-line-style-list> = [ <line-style-or-repeat> ]* <auto-repeat-line-style> [ <line-style-or-repeat> ]*
/// ```
pub type AutoLineStyleList<'a> = LineStyleList<'a>;
pub type AutoLineStyleListItem<'a> = LineStyleOrRepeat<'a>;
pub type AutoRepeatLineStyle<'a> = RepeatLineStyle<'a>;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AutoLineStyleList, "repeat(auto,solid)");
		assert_parse!(CssAtomSet::ATOMS, AutoLineStyleList, "solid repeat(auto,dashed)");
		assert_parse!(CssAtomSet::ATOMS, AutoLineStyleList, "repeat(auto,dashed) dotted");
		assert_parse!(CssAtomSet::ATOMS, AutoLineStyleList, "solid repeat(2,none) repeat(auto,dashed) dotted");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, AutoLineStyleList, "");
		assert_peek_false!(CssAtomSet::ATOMS, AutoLineStyleList, "florp");
		assert_parse_error!(CssAtomSet::ATOMS, AutoLineStyleList, "repeat(auto,)");
	}
}
