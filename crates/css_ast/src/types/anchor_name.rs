use css_parse::T;
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

// https://drafts.csswg.org/css-anchor-position-1/#typedef-anchor-name
#[derive(ToSpan, Parse, Peek, ToCursors, Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AnchorName(T![DashedIdent]);
