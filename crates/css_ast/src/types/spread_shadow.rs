#![allow(unused)]
use super::prelude::*;

use crate::Todo;

// https://drafts.csswg.org/css-borders-4/#typedef-spread-shadow
// <spread-shadow> = <'box-shadow-color'>? && [ <'box-shadow-offset'> [ <'box-shadow-blur'> <'box-shadow-spread'>? ]? ] && <'box-shadow-position'>?
pub type SpreadShadow = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
}
