#![allow(unused)]
use super::prelude::*;

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-line-color-list
// <line-color-list> = [ <line-color-or-repeat> ]+
pub type LineColorList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
}
