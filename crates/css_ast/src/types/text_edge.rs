#![allow(unused)]
use super::prelude::*;

use crate::Todo;

// https://drafts.csswg.org/css-inline-3/#typedef-text-edge
// <text-edge> = [ text | ideographic | ideographic-ink ] | [ text | ideographic | ideographic-ink | cap | ex ] [ text | ideographic | ideographic-ink | alphabetic ]
pub type TextEdge = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
}
