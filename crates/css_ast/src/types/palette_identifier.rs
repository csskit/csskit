use super::prelude::*;

/// <https://www.w3.org/TR/css-fonts-4/#typedef-font-palette-palette-identifier>
///
/// ```text,ignore
/// <palette-identifier> = <dashed-ident>
/// ```
#[derive(Parse, Peek, ToCursors, SemanticEq, Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaletteIdentifier(T![DashedIdent]);
