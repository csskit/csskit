use super::prelude::*;

/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-composition>
///
/// ```text,ignore
/// <single-animation-composition> = replace | add | accumulate
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum SingleAnimationComposition {
	#[atom(CssAtomSet::Replace)]
	Replace(T![Ident]),
	#[atom(CssAtomSet::Add)]
	Add(T![Ident]),
	#[atom(CssAtomSet::Accumulate)]
	Accumulate(T![Ident]),
}
