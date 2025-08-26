#![allow(warnings)]
use css_parse::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, SourceOffset, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-line-width-list
// <line-width-list> = [ <line-width-or-repeat> ]+
pub type LineWidthList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
