#![allow(warnings)]
use css_parse::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, SourceOffset, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-animations-2/#typedef-single-animation-trigger
// <single-animation-trigger> = <single-animation-trigger-behavior> || [ none | auto | [ [ <dashed-ident> | <scroll()> | <view()> ] [ normal | <length-percentage> | <timeline-range-name> <length-percentage>? ]{0,4} ] ]
pub type SingleAnimationTrigger = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
