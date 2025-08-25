#![allow(warnings)]
use css_parse::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, SourceOffset, T, ToCursors};

use crate::Todo;

// https://drafts.csswg.org/css-grid-2/#typedef-track-size
// <track-size> = <track-breadth> | minmax( <inflexible-breadth> , <track-breadth> ) | fit-content( <length-percentage [0,∞]> )
// <track-breadth> = <length-percentage [0,∞]> | <flex [0,∞]> | min-content | max-content | auto
// <inflexible-breadth>  = <length-percentage [0,∞]> | min-content | max-content | auto
pub type TrackSize = Todo;

#[cfg(test)]
mod tests {
	use super::*;
}
