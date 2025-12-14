use crate::{Cursor, Kind, KindSet, Span, State, ToSpan, Token};

/// Represents a point during the [Parser's][crate::Parser] lifecycle; retaining state that can then be rewound.
///
/// Don't use this directly, instead retrieve a checkpoint with [Parser::checkpoint()][crate::Parser::checkpoint] and
/// rewind the parser to a checkpoint with [Parser::rewind()][crate::Parser::rewind()].
#[derive(Debug, Clone)]
pub struct ParserCheckpoint<I> {
	pub(crate) cursor: Cursor,
	pub(crate) errors_pos: u8,
	pub(crate) trivia_pos: u16,
	pub(crate) iter: I,
	pub(crate) buffer: [Cursor; 12],
	pub(crate) buffer_index: usize,
	pub(crate) skip: KindSet,
	pub(crate) stop: KindSet,
	pub(crate) state: State,
}

impl<I> From<ParserCheckpoint<I>> for Cursor {
	fn from(value: ParserCheckpoint<I>) -> Self {
		value.cursor
	}
}

impl<I> From<ParserCheckpoint<I>> for Token {
	fn from(value: ParserCheckpoint<I>) -> Self {
		value.cursor.token()
	}
}

impl<I> From<ParserCheckpoint<I>> for Kind {
	fn from(value: ParserCheckpoint<I>) -> Self {
		value.cursor.token().kind()
	}
}

impl<I> ToSpan for ParserCheckpoint<I> {
	fn to_span(&self) -> Span {
		self.cursor.span()
	}
}
