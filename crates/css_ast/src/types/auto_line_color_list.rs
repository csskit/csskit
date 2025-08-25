#![allow(warnings)]
use css_parse::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, SourceOffset, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-color-list
// <auto-line-color-list> = [ <line-color-or-repeat> ]* <auto-repeat-line-color> [ <line-color-or-repeat> ]*
pub type AutoLineColorList = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
