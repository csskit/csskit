use super::prelude::*;

/// <https://drafts.csswg.org/css-backgrounds-4/#background-repeat>
///
/// ```text,ignore
/// <repeat-style> = repeat-x | repeat-y | <repetition>{1,2}
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum RepeatStyle {
	#[atom(CssAtomSet::RepeatX)]
	RepeatX(T![Ident]),
	#[atom(CssAtomSet::RepeatY)]
	RepeatY(T![Ident]),
	Repetition(Repetition, Option<Repetition>),
}

/// <https://drafts.csswg.org/css-backgrounds-4/#typedef-repetition>
///
/// ```text,ignore
/// <repetition> = repeat | space | round | no-repeat
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum Repetition {
	#[atom(CssAtomSet::Repeat)]
	Repeat(T![Ident]),
	#[atom(CssAtomSet::Space)]
	Space(T![Ident]),
	#[atom(CssAtomSet::Round)]
	Round(T![Ident]),
	#[atom(CssAtomSet::NoRepeat)]
	NoRepeat(T![Ident]),
}

#[cfg(test)]
mod test {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, RepeatStyle, "repeat-x");
		assert_parse!(CssAtomSet::ATOMS, RepeatStyle, "repeat-y");
		assert_parse!(CssAtomSet::ATOMS, RepeatStyle, "repeat repeat");
		assert_parse!(CssAtomSet::ATOMS, RepeatStyle, "space round");
	}
}
