use super::prelude::*;

/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger-type>
///
/// ```text,ignore
/// <single-animation-trigger-type> = once | repeat | alternate | state
/// ```
#[derive(Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum SingleAnimationTriggerType {
	#[atom(CssAtomSet::Once)]
	Once(T![Ident]),
	#[atom(CssAtomSet::Repeat)]
	Repeat(T![Ident]),
	#[atom(CssAtomSet::Alternate)]
	Alternate(T![Ident]),
	#[atom(CssAtomSet::State)]
	State(T![Ident]),
}
