use super::prelude::*;

/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger-type>
///
/// ```text,ignore
/// <single-animation-trigger-type> = once | repeat | alternate | state
/// ```
#[derive(Parse, Peek, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
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
