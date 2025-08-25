#![allow(warnings)]
use css_parse::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, SourceOffset, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-shapes-1/#typedef-basic-shape-rect
// <basic-shape-rect> = <inset()> | <rect()> | <xywh()>
pub type BasicShapeRect = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
