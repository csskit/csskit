use super::prelude::*;

/// <https://drafts.csswg.org/css-ui-4/#typedef-cursor-predefined>
///
/// ```text,ignore
/// <cursor-predefined> = auto | default | none | context-menu | help | pointer | progress | wait | cell | crosshair | text | vertical-text | alias | copy | move | no-drop | not-allowed | grab | grabbing | e-resize | n-resize | ne-resize | nw-resize | s-resize | se-resize | sw-resize | w-resize | ew-resize | ns-resize | nesw-resize | nwse-resize | col-resize | row-resize | all-scroll | zoom-in | zoom-out
/// ```
///
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CursorPredefined {
	#[atom(CssAtomSet::Auto)]
	Auto(T![Ident]),
	#[atom(CssAtomSet::Default)]
	Default(T![Ident]),
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::ContextMenu)]
	ContextMenu(T![Ident]),
	#[atom(CssAtomSet::Help)]
	Help(T![Ident]),
	#[atom(CssAtomSet::Pointer)]
	Pointer(T![Ident]),
	#[atom(CssAtomSet::Progress)]
	Progress(T![Ident]),
	#[atom(CssAtomSet::Wait)]
	Wait(T![Ident]),
	#[atom(CssAtomSet::Cell)]
	Cell(T![Ident]),
	#[atom(CssAtomSet::Crosshair)]
	Crosshair(T![Ident]),
	#[atom(CssAtomSet::Text)]
	Text(T![Ident]),
	#[atom(CssAtomSet::VerticalText)]
	VerticalText(T![Ident]),
	#[atom(CssAtomSet::Alias)]
	Alias(T![Ident]),
	#[atom(CssAtomSet::Copy)]
	Copy(T![Ident]),
	#[atom(CssAtomSet::Move)]
	Move(T![Ident]),
	#[atom(CssAtomSet::NoDrop)]
	NoDrop(T![Ident]),
	#[atom(CssAtomSet::NotAllowed)]
	NotAllowed(T![Ident]),
	#[atom(CssAtomSet::Grab)]
	Grab(T![Ident]),
	#[atom(CssAtomSet::Grabbing)]
	Grabbing(T![Ident]),
	#[atom(CssAtomSet::EResize)]
	EResize(T![Ident]),
	#[atom(CssAtomSet::NResize)]
	NResize(T![Ident]),
	#[atom(CssAtomSet::NeResize)]
	NeResize(T![Ident]),
	#[atom(CssAtomSet::NwResize)]
	NwResize(T![Ident]),
	#[atom(CssAtomSet::SResize)]
	SResize(T![Ident]),
	#[atom(CssAtomSet::SeResize)]
	SeResize(T![Ident]),
	#[atom(CssAtomSet::SwResize)]
	SwResize(T![Ident]),
	#[atom(CssAtomSet::WResize)]
	WResize(T![Ident]),
	#[atom(CssAtomSet::EwResize)]
	EwResize(T![Ident]),
	#[atom(CssAtomSet::NsResize)]
	NsResize(T![Ident]),
	#[atom(CssAtomSet::NeswResize)]
	NeswResize(T![Ident]),
	#[atom(CssAtomSet::NwseResize)]
	NwseResize(T![Ident]),
	#[atom(CssAtomSet::ColResize)]
	ColResize(T![Ident]),
	#[atom(CssAtomSet::RowResize)]
	RowResize(T![Ident]),
	#[atom(CssAtomSet::AllScroll)]
	AllScroll(T![Ident]),
	#[atom(CssAtomSet::ZoomIn)]
	ZoomIn(T![Ident]),
	#[atom(CssAtomSet::ZoomOut)]
	ZoomOut(T![Ident]),
}
