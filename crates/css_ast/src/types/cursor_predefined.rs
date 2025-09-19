use super::prelude::*;

keyword_set!(
	/// <https://drafts.csswg.org/css-ui-4/#typedef-cursor-predefined>
	///
	/// ```text,ignore
	/// <cursor-predefined> = auto | default | none | context-menu | help | pointer | progress | wait | cell | crosshair | text | vertical-text | alias | copy | move | no-drop | not-allowed | grab | grabbing | e-resize | n-resize | ne-resize | nw-resize | s-resize | se-resize | sw-resize | w-resize | ew-resize | ns-resize | nesw-resize | nwse-resize | col-resize | row-resize | all-scroll | zoom-in | zoom-out
	/// ```
	///
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum CursorPredefined {
		Auto: "auto",
		Default: "default",
		None: "none",
		ContextMenu: "context-menu",
		Help: "help",
		Pointer: "pointer",
		Progress: "progress",
		Wait: "wait",
		Cell: "cell",
		Crosshair: "crosshair",
		Text: "text",
		VerticalText: "vertical-text",
		Alias: "alias",
		Copy: "copy",
		Move: "move",
		NoDrop: "no-drop",
		NotAllowed: "not-allowed",
		Grab: "grab",
		Grabbing: "grabbing",
		EResize: "e-resize",
		NResize: "n-resize",
		NeResize: "ne-resize",
		NwResize: "nw-resize",
		SResize: "s-resize",
		SeResize: "se-resize",
		SwResize: "sw-resize",
		WResize: "w-resize",
		EwResize: "ew-resize",
		NsResize: "ns-resize",
		NeswResize: "nesw-resize",
		NwseResize: "nwse-resize",
		ColResize: "col-resize",
		RowResize: "row-resize",
		AllScroll: "all-scroll",
		ZoomIn: "zoom-in",
		ZoomOut: "zoom-out",
	}
);
