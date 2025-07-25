#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-grid-2/#typedef-grid-row-start-grid-line
// <grid-line> = auto | <custom-ident> | [ [ <integer [-∞,-1]> | <integer [1,∞]> ] && <custom-ident>? ] | [ span && [ <integer [1,∞]> || <custom-ident> ] ]
pub type GridLine = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
