use super::prelude::*;

/// <https://drafts.csswg.org/css-transitions-2/#typedef-transition-behavior-value>
///
/// ```text,ignore
/// <transition-behavior-value> = normal | allow-discrete
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TransitionBehaviorValue {
	#[atom(CssAtomSet::Normal)]
	Normal(T![Ident]),
	#[atom(CssAtomSet::AllowDiscrete)]
	AllowDiscrete(T![Ident]),
}
