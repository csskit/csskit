#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-style-list
// <auto-line-style-list> = [ <line-style-or-repeat> ]* <auto-repeat-line-style> [ <line-style-or-repeat> ]*
pub type AutoLineStyleList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
