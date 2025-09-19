#![allow(unused)]
use super::prelude::*;

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-color-list
// <auto-line-color-list> = [ <line-color-or-repeat> ]* <auto-repeat-line-color> [ <line-color-or-repeat> ]*
pub type AutoLineColorList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
