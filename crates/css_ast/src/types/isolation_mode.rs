use super::prelude::*;

/// <https://drafts.csswg.org/compositing-2/#isolated-propid>
///
/// ```text,ignore
/// <isolation-mode> = [ auto | isolate ]
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum IsolationMode {
	#[atom(CssAtomSet::Auto)]
	Auto(T![Ident]),
	#[atom(CssAtomSet::Isolate)]
	Isolate(T![Ident]),
}
