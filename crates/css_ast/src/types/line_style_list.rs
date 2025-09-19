#![allow(unused)]
use super::prelude::*;

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-line-style-list
// <line-style-list> = [ <line-style-or-repeat> ]+
pub type LineStyleList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
