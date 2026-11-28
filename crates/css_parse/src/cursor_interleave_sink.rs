use crate::{Cursor, CursorSink, Kind, SourceOffset, Vec};

/// This is a [CursorSink] that wraps a Sink (`impl CursorSink`) and a slice of [Cursor]s to interleave. On each
/// [CursorSink::append()] call, will append to the delegate sink, while also interleaving any of the Cursors from the
/// slice of [Cursor]s, in the right places.
///
/// This is useful as way to interleave ancilliary cursors, for example trivia. Cursors at or past `end`, such as those
/// of nodes parsed into a tree after the source, pass through without interleaving.
pub struct CursorInterleaveSink<'a, S: ?Sized> {
	sink: &'a mut S,
	interleave: &'a [(Vec<'a, Cursor>, Cursor)],
	end: SourceOffset,
	current_index: usize,
	#[cfg(debug_assertions)]
	seen_eof: bool,
}

impl<'a, S: CursorSink + ?Sized> CursorInterleaveSink<'a, S> {
	pub fn new(sink: &'a mut S, interleave: &'a [(Vec<'a, Cursor>, Cursor)], end: SourceOffset) -> Self {
		Self {
			sink,
			interleave,
			end,
			current_index: 0,
			#[cfg(debug_assertions)]
			seen_eof: false,
		}
	}
}

impl<'a, S: CursorSink + ?Sized> CursorSink for CursorInterleaveSink<'a, S> {
	fn append(&mut self, c: Cursor) {
		#[cfg(debug_assertions)]
		{
			debug_assert!(!self.seen_eof, "Received cursor after EOF: {:?}", c);
			if c == Kind::Eof {
				self.seen_eof = true;
			}
		}

		if c != Kind::Eof && c.offset() >= self.end {
			self.sink.append(c);
			return;
		}
		// Check if this content cursor has associated trivia
		while self.current_index < self.interleave.len() {
			let (trivia, associated_cursor) = &self.interleave[self.current_index];
			if associated_cursor.offset() == c.offset() && associated_cursor.token().kind() == c.token().kind() {
				for cursor in trivia {
					self.sink.append_trivia(*cursor);
				}
				self.current_index += 1;
				break;
			}
			if associated_cursor.offset() >= c.offset() {
				break;
			}
			self.current_index += 1;
		}

		// If this is EOF, flush any remaining trivia before emitting EOF
		if c == Kind::Eof {
			while self.current_index < self.interleave.len() {
				let (trivia, _) = &self.interleave[self.current_index];
				for cursor in trivia {
					self.sink.append_trivia(*cursor);
				}
				self.current_index += 1;
			}
		}

		self.sink.append(c);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Arena;
	use css_lexer::{EmptyAtomSet, Lexer, Token};

	#[test]
	fn passes_through_cursors_past_end() {
		let alloc = Arena::default();
		let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "a b ");
		let ident_a = lexer.next().unwrap();
		let leading = lexer.next().unwrap();
		let ident_b = lexer.next().unwrap();
		let trailing = lexer.next().unwrap();
		let end = SourceOffset(4);
		let inserted = Cursor::new(end, Token::COMMA);
		let eof = Cursor::new(end, Token::EOF);
		let mut leading_trivia = Vec::new_in(&alloc);
		leading_trivia.push(leading);
		let mut trailing_trivia = Vec::new_in(&alloc);
		trailing_trivia.push(trailing);
		let interleave = [(leading_trivia, ident_b), (trailing_trivia, eof)];
		let mut out = Vec::new_in(&alloc);
		let mut sink = CursorInterleaveSink::new(&mut out, &interleave, end);
		for cursor in [ident_a, inserted, ident_b, eof] {
			sink.append(cursor);
		}
		let offsets: std::vec::Vec<u32> = out.iter().map(|cursor| cursor.offset().0).collect();
		assert_eq!(offsets, [0, 4, 1, 2, 3, 4]);
	}
}
