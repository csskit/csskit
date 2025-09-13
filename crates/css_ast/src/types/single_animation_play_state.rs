use super::prelude::*;

/// <https://drafts.csswg.org/css-animations/#typedef-single-animation-play-state>
///
/// ```text,ignore
/// <single-animation-play-state> = running | paused
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum SingleAnimationPlayState {
	#[atom(CssAtomSet::Running)]
	Running(T![Ident]),
	#[atom(CssAtomSet::Paused)]
	Paused(T![Ident]),
}
