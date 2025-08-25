#![allow(warnings)]
use css_parse::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, SourceOffset, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-width-list
// <auto-line-width-list>     = [ <line-width-or-repeat> ]* <auto-repeat-line-width> [ <line-width-or-repeat> ]*
pub type AutoLineWidthList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
