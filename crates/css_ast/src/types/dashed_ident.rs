use super::prelude::*;

/// <https://drafts.csswg.org/css-values/#dashed-idents>
///
/// Wraps `T![DashedIdent]`, but exists for the purposes of Visitable/VisitableMut.
#[derive(IntoCursor, Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct DashedIdent(T![DashedIdent]);
