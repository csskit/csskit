use super::prelude::*;

/// <https://drafts.fxtf.org/compositing-1/#typedef-compositing-operator>
///
/// ```text,ignore
/// <compositing-operator> = clear | copy | source-over | destination-over | source-in |
///   destination-in | source-out | destination-out | source-atop | destination-atop |
///   xor | lighter | add
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CompositingOperator {
	#[atom(CssAtomSet::Clear)]
	Clear(T![Ident]),
	#[atom(CssAtomSet::Copy)]
	Copy(T![Ident]),
	#[atom(CssAtomSet::SourceOver)]
	SourceOver(T![Ident]),
	#[atom(CssAtomSet::DestinationOver)]
	DestinationOver(T![Ident]),
	#[atom(CssAtomSet::SourceIn)]
	SourceIn(T![Ident]),
	#[atom(CssAtomSet::DestinationIn)]
	DestinationIn(T![Ident]),
	#[atom(CssAtomSet::SourceOut)]
	SourceOut(T![Ident]),
	#[atom(CssAtomSet::DestinationOut)]
	DestinationOut(T![Ident]),
	#[atom(CssAtomSet::SourceAtop)]
	SourceAtop(T![Ident]),
	#[atom(CssAtomSet::DestinationAtop)]
	DestinationAtop(T![Ident]),
	#[atom(CssAtomSet::Xor)]
	Xor(T![Ident]),
	#[atom(CssAtomSet::Lighter)]
	Lighter(T![Ident]),
	#[atom(CssAtomSet::Add)]
	Add(T![Ident]),
}
