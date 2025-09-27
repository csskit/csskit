use crate::{Cursor, CursorSink};

/// This is a [CursorSink] that wraps a Sink (`impl CursorSink`) and a slice of [Cursor]s to interleave. On each
/// [CursorSink::append()] call, will append to the delegate sink, while also interleaving any of the Cursors from the
/// slice of [Cursor]s, in the right places.
///
/// This is useful as way to interleave ancilliary cursors, for example trivia.
pub struct CursorInterleaveSink<'a, S> {
	sink: &'a mut S,
	interleave: &'a [(bumpalo::collections::Vec<'a, Cursor>, Cursor)],
	current_index: usize,
}

impl<'a, S: CursorSink> CursorInterleaveSink<'a, S> {
	pub fn new(sink: &'a mut S, interleave: &'a [(bumpalo::collections::Vec<'a, Cursor>, Cursor)]) -> Self {
		Self { sink, interleave, current_index: 0 }
	}
}

impl<'a, S: CursorSink> CursorSink for CursorInterleaveSink<'a, S> {
	fn append(&mut self, c: Cursor) {
		// Check if this content cursor has associated trivia
		while self.current_index < self.interleave.len() {
			let (trivia, associated_cursor) = &self.interleave[self.current_index];
			if *associated_cursor == c {
				for cursor in trivia {
					self.sink.append(*cursor);
				}
				self.current_index += 1;
				break;
			}
			if associated_cursor.span().start() > c.span().start() {
				break;
			}
			self.current_index += 1;
		}
		self.sink.append(c);
	}
}
