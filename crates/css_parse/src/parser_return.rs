use crate::{Cursor, CursorInterleaveSink, CursorSink, Diagnostic, ToCursors};
use bumpalo::collections::Vec;

#[derive(Debug)]
pub struct ParserReturn<'a, T>
where
	T: ToCursors,
{
	pub output: Option<T>,
	pub source_text: &'a str,
	pub errors: Vec<'a, Diagnostic>,
	pub trivia: Vec<'a, (Vec<'a, Cursor>, Cursor)>,
	with_trivia: bool,
}

impl<'a, T: ToCursors> ParserReturn<'a, T> {
	pub fn new(
		output: Option<T>,
		source_text: &'a str,
		errors: Vec<'a, Diagnostic>,
		trivia: Vec<'a, (Vec<'a, Cursor>, Cursor)>,
	) -> Self {
		Self { output, source_text, errors, trivia, with_trivia: false }
	}

	pub fn with_trivia(mut self) -> Self {
		self.with_trivia = true;
		self
	}
}

impl<T: ToCursors> ToCursors for ParserReturn<'_, T> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		if let Some(output) = &self.output {
			if self.with_trivia {
				let mut sink = CursorInterleaveSink::new(s, &self.trivia);
				ToCursors::to_cursors(output, &mut sink);
			} else {
				ToCursors::to_cursors(output, s);
			}
		}
	}
}
