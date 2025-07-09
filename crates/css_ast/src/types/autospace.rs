#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-text-4/#typedef-autospace
// <autospace> = no-autospace | [ ideograph-alpha || ideograph-numeric || punctuation ] || [ insert | replace ]
pub type Autospace = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
