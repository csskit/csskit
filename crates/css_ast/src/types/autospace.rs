#![allow(unused)]
use super::prelude::*;

use crate::Todo;

// https://drafts.csswg.org/css-text-4/#typedef-autospace
// <autospace> = no-autospace | [ ideograph-alpha || ideograph-numeric || punctuation ] || [ insert | replace ]
pub type Autospace = Todo;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
}
