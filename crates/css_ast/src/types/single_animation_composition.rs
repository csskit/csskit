use super::prelude::*;

/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-composition>
///
/// ```text,ignore
/// <single-animation-composition> = replace | add | accumulate
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SingleAnimationComposition {
	#[atom(CssAtomSet::Replace)]
	Replace(T![Ident]),
	#[atom(CssAtomSet::Add)]
	Add(T![Ident]),
	#[atom(CssAtomSet::Accumulate)]
	Accumulate(T![Ident]),
}
