use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#contextual-alt-values>
///
/// ```text,ignore
/// <contextual-alt-values> = [ contextual | no-contextual ]
/// ```
#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ContextualAltValues {
	#[atom(CssAtomSet::Contextual)]
	Contextual(T![Ident]),
	#[atom(CssAtomSet::NoContextual)]
	NoContextual(T![Ident]),
}
