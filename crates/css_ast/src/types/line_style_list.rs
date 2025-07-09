#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-line-style-list
// <line-style-list> = [ <line-style-or-repeat> ]+
pub type LineStyleList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
