#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-line-color-list
// <line-color-list> = [ <line-color-or-repeat> ]+
pub type LineColorList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
