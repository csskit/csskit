use super::prelude::*;

/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger-behavior>
///
/// ```text,ignore
/// <single-animation-trigger-behavior> = once | repeat | alternate | state
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum SingleAnimationTriggerBehavior {
	#[atom(CssAtomSet::Once)]
	Once(T![Ident]),
	#[atom(CssAtomSet::Repeat)]
	Repeat(T![Ident]),
	#[atom(CssAtomSet::Alternate)]
	Alternate(T![Ident]),
	#[atom(CssAtomSet::State)]
	State(T![Ident]),
}
