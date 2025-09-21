use super::prelude::*;

// https://www.w3.org/TR/css-fonts-4/#typedef-font-palette-palette-identifier
#[derive(Parse, Peek, ToCursors, Debug, Default, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct PaletteIdentifier(T![DashedIdent]);
