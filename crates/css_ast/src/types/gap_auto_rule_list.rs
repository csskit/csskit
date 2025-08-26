#![allow(warnings)]
use css_parse::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, SourceOffset, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-gap-auto-rule-list
// <gap-auto-rule-list> = <gap-rule-or-repeat>#? , <gap-auto-repeat-rule> , <gap-rule-or-repeat>#?
pub type GapAutoRuleList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
