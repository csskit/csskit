#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-shapes-1/#typedef-basic-shape-rect
// <basic-shape-rect> = <inset()> | <rect()> | <xywh()>
pub type BasicShapeRect = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
