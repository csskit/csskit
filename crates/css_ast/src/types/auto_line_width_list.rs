#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-width-list
// <auto-line-width-list>     = [ <line-width-or-repeat> ]* <auto-repeat-line-width> [ <line-width-or-repeat> ]*
pub type AutoLineWidthList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
