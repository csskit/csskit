#![allow(unused)]
use super::prelude::*;
use crate::Todo;

/// <https://drafts.csswg.org/css-masking-1/#typedef-mask-layer>
///
/// ```text,ignore
/// <mask-layer> = <mask-reference> || <position> [ / <bg-size> ]? || <repeat-style> ||
///   <geometry-box> || [ <geometry-box> | no-clip ] || <compositing-operator> ||
///   <masking-mode>
/// ```
pub type MaskLayer<'a> = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
}
