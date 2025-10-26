use super::prelude::*;

/// <https://drafts.csswg.org/css-animations-2/#typedef-animation-action>
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum AnimationAction {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Play)]
	Play(T![Ident]),
	#[atom(CssAtomSet::PlayForwards)]
	PlayForwards(T![Ident]),
	#[atom(CssAtomSet::PlayBackwards)]
	PlayBackwards(T![Ident]),
	#[atom(CssAtomSet::Pause)]
	Pause(T![Ident]),
	#[atom(CssAtomSet::Reset)]
	Reset(T![Ident]),
	#[atom(CssAtomSet::Replay)]
	Replay(T![Ident]),
}
