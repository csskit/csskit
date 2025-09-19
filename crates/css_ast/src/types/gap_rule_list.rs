#![allow(unused)]
use super::prelude::*;

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-gap-rule-list
// <gap-rule-list> = <gap-rule-or-repeat>#
pub type GapRuleList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
