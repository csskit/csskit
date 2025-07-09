#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-borders-4/#typedef-corner-shape-value
// <corner-shape-value> = round | scoop | bevel | notch | square | squircle | <superellipse()>
// superellipse() = superellipse(<number [-∞,∞]> | infinity | -infinity)
pub type CornerShapeValue = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
