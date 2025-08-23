use css_parse::T;
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, Visitable};

// https://drafts.csswg.org/css-anchor-position-1/#typedef-anchor-name
#[derive(
	IntoCursor, Parse, Peek, ToCursors, Visitable, Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct AnchorName(T![DashedIdent]);
