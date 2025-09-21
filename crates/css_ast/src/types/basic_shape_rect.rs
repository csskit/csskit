#![allow(unused)]
use super::prelude::*;

use crate::Todo;

// https://drafts.csswg.org/css-shapes-1/#typedef-basic-shape-rect
// <basic-shape-rect> = <inset()> | <rect()> | <xywh()>
pub type BasicShapeRect = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
}
