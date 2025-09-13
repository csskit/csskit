use super::prelude::*;

/// <https://drafts.csswg.org/css-animations/#typedef-single-animation-direction>
///
/// ```text,ignore
/// <single-animation-direction> = normal | reverse | alternate | alternate-reverse
/// ```
#[derive(Parse, Peek, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum SingleAnimationDirection {
	#[atom(CssAtomSet::Normal)]
	Normal(T![Ident]),
	#[atom(CssAtomSet::Reverse)]
	Reverse(T![Ident]),
	#[atom(CssAtomSet::Alternate)]
	Alternate(T![Ident]),
	#[atom(CssAtomSet::AlternateReverse)]
	AlternateReverse(T![Ident]),
}
