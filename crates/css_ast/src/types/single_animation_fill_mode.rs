use super::prelude::*;

/// <https://drafts.csswg.org/css-animations/#typedef-single-animation-fill-mode>
///
/// ```text,ignore
/// <single-animation-fill-mode> = none | forwards | backwards | both
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum SingleAnimationFillMode {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Forwards)]
	Forwards(T![Ident]),
	#[atom(CssAtomSet::Backwards)]
	Backwards(T![Ident]),
	#[atom(CssAtomSet::Both)]
	Both(T![Ident]),
}
