#![allow(warnings)]
use css_parse::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, SourceOffset, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-gap-rule-list
// <gap-rule-list> = <gap-rule-or-repeat>#
pub type GapRuleList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
