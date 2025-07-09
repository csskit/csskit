#![allow(warnings)]
use css_lexer::Cursor;
use css_parse::{CursorSink, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-values-5/#calc-size
// <calc-size()> = calc-size( <calc-size-basis>, <calc-sum> )
// <calc-size-basis> = [ <size-keyword> | <calc-size()> | any | <calc-sum> ]
//
// The <size-keyword> production matches any sizing keywords allowed in the context. For example, in width, it matches auto, min-content, stretch, etc.
pub type CalcSize = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
