#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-content-3/#content-values
// <content-list> = [ <string> | <image> | <attr()> | contents | <quote> | <leader()> | <target> | <string()> | <content()> | <counter> ]+
pub type ContentList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
