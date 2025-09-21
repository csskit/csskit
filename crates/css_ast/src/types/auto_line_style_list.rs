#![allow(unused)]
use super::prelude::*;

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-style-list
// <auto-line-style-list> = [ <line-style-or-repeat> ]* <auto-repeat-line-style> [ <line-style-or-repeat> ]*
pub type AutoLineStyleList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
}
