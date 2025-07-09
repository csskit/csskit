#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-borders-4/#typedef-spread-shadow
// <spread-shadow> = <'box-shadow-color'>? && [ <'box-shadow-offset'> [ <'box-shadow-blur'> <'box-shadow-spread'>? ]? ] && <'box-shadow-position'>?
pub type SpreadShadow = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
