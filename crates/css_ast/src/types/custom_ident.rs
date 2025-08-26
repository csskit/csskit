use css_parse::T;
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, Visitable};

/// <https://drafts.csswg.org/css-values/#custom-idents>
///
/// Wraps `T![Ident]`, but exists for the purposes of Visitable/VisitableMut.
#[derive(IntoCursor, Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct CustomIdent(T![Ident]);
