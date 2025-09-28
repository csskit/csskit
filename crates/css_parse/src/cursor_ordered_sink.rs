use crate::{Cursor, CursorSink, Kind, SourceOffset};
use bumpalo::collections::Vec;

/// This is a [CursorSink] that buffers cursors and emits them in source order. It uses contiguous coverage tracking to
/// eagerly emit cursors as soon as gaps are filled.
///
/// Many CSS grammars allow constructing a tree in arbitrarily authored order, but have a canonical ordering, so for e.g
/// a function like `foo(bar, baz)` could be represented as `foo(baz, bar)` and still be valid. AST nodes output their
/// cursors in grammar order, which is most often desirable, but this sink will enforce source ordering.
pub struct CursorOrderedSink<'a, S> {
	sink: &'a mut S,
	/// Sorted buffer of cursors by start position
	buffer: Vec<'a, Cursor>,
	/// How far we've committed (emitted contiguously from position 0)
	committed_position: SourceOffset,
	#[cfg(debug_assertions)]
	seen_eof: bool,
}

impl<'a, S: CursorSink> CursorOrderedSink<'a, S> {
	pub fn new(bump: &'a bumpalo::Bump, sink: &'a mut S) -> Self {
		Self {
			sink,
			buffer: Vec::new_in(bump),
			committed_position: SourceOffset(0),
			#[cfg(debug_assertions)]
			seen_eof: false,
		}
	}

	/// Flush all remaining buffered cursors to the delegate sink in source order.
	/// This is typically called when no more cursors will be added.
	pub fn flush(&mut self) {
		self.buffer.sort_by_key(|cursor| cursor.span().start());
		for cursor in self.buffer.iter() {
			self.sink.append(*cursor);
		}
		if let Some(last_cursor) = self.buffer.last() {
			self.committed_position = last_cursor.end_offset();
		}
		self.buffer.clear();
	}
}

impl<'a, S: CursorSink> CursorSink for CursorOrderedSink<'a, S> {
	// Insert cursor into the buffer, maintaining sorted order efficiently
	//
	// The algorithm models cursor coverage as an "array with holes" that get filled over time:
	//
	// ```text
	// Positions: [0][1][2][3][4][5][6]
	// Initial:   [ ][ ][ ][ ][ ][ ][ ]    committed_position = 0
	//
	// Add cursor_4: [ ][ ][ ][ ][4][ ][ ]    buffer: [4], nothing emitted
	// Add cursor_0: [0][ ][ ][ ][4][ ][ ]    emit [0], committed_position = 1
	// Add cursor_2: [0][ ][2][ ][4][ ][ ]    buffer: [2,4], gap at 1
	// Add cursor_1: [0][1][2][ ][4][ ][ ]    emit [1,2], committed_position = 3
	// Add cursor_3: [0][1][2][3][4][ ][ ]    emit [3,4], committed_position = 5
	// Add cursor_5: [0][1][2][3][4][5][ ]    emit [5], committed_position = 6
	// ```
	//
	// Once a contiguous section from `committed_position` is complete, it's emitted immediately.
	fn append(&mut self, cursor: Cursor) {
		#[cfg(debug_assertions)]
		{
			debug_assert!(!self.seen_eof, "Received cursor after EOF: {:?}", cursor);
			if cursor == Kind::Eof {
				self.seen_eof = true;
			}
		}
		let cursor_start = cursor.span().start();
		if self.buffer.is_empty() || cursor_start.0 >= self.buffer.last().unwrap().span().start().0 {
			self.buffer.push(cursor);
		} else if cursor_start == self.committed_position {
			// This cursor is the next in order
			self.sink.append(cursor);
			self.committed_position = cursor.end_offset();
		} else {
			// The cursor needs to be buffered.
			// TODO: binary_search_by_key is giving BTreeMap which would be O(log n) instead of O(n), but
			// for small enough numbers that's fine? Investigate more.
			let insert_pos =
				self.buffer.binary_search_by_key(&cursor_start, |c| c.span().start()).unwrap_or_else(|pos| pos);
			self.buffer.insert(insert_pos, cursor);
		}

		// Check if a contiguous section from committed_position exists, and emit it if so
		while !self.buffer.is_empty() {
			// Remove any overlapping cursors first (those that start before committed_position)
			let mut overlapping_count = 0;
			for cursor in self.buffer.iter() {
				if cursor.span().start().0 < self.committed_position.0 {
					overlapping_count += 1;
				} else {
					break;
				}
			}
			if overlapping_count > 0 {
				self.buffer.drain(0..overlapping_count);
			}

			if self.buffer.is_empty() {
				break;
			}

			// Find how many contiguous cursors can be emitted from the front
			let mut current_pos = self.committed_position;
			let mut emit_count = 0;

			for cursor in self.buffer.iter() {
				let cursor_start = cursor.span().start();

				if cursor_start == current_pos {
					current_pos = cursor.end_offset();
					emit_count += 1;
				} else {
					// If this cursor starts after current_pos, stop, as there is a gap
					break;
				}
			}

			if emit_count > 0 {
				for cursor in self.buffer.drain(0..emit_count) {
					self.sink.append(cursor);
				}
				self.committed_position = current_pos;
			} else {
				// No contiguous section found, stop
				break;
			}
		}

		// If we just processed EOF, flush any remaining buffered cursors
		if cursor == Kind::Eof {
			self.flush();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{ComponentValues, EmptyAtomSet, Parser, SourceCursor, ToCursors};
	use bumpalo::{Bump, collections::Vec as BumpVec};
	use std::fmt::Write;

	#[test]
	fn test_basic() {
		let source_text = "foo bar";
		let bump = Bump::default();
		let mut output = BumpVec::new_in(&bump);
		{
			let mut ordered_sink = CursorOrderedSink::new(&bump, &mut output);
			let mut parser = Parser::new(&bump, &EmptyAtomSet::ATOMS, source_text);
			parser.parse_entirely::<ComponentValues>().output.unwrap().to_cursors(&mut ordered_sink);
			ordered_sink.flush();
		}
		let mut result = String::new();
		for c in output.iter() {
			write!(&mut result, "{}", SourceCursor::from(*c, c.str_slice(source_text))).unwrap();
		}
		assert_eq!(result, "foo bar");
	}

	#[test]
	fn test_manual_flush() {
		use crate::{SourceOffset, Token};

		let bump = Bump::default();
		let mut output = BumpVec::new_in(&bump);
		{
			let mut ordered_sink = CursorOrderedSink::new(&bump, &mut output);
			let cursor1 = Cursor::new(SourceOffset(0), Token::SPACE); // position 0, length 1
			let cursor2 = Cursor::new(SourceOffset(10), Token::SPACE); // position 10, length 1
			let cursor3 = Cursor::new(SourceOffset(4), Token::SPACE); // position 4, length 1

			// Append cursors in non-source-order
			ordered_sink.append(cursor2);
			ordered_sink.append(cursor1);
			ordered_sink.append(cursor3);
			ordered_sink.flush();
		}
		assert_eq!(output.len(), 3);
		assert_eq!(output[0].span().start(), SourceOffset(0));
		assert_eq!(output[1].span().start(), SourceOffset(4));
		assert_eq!(output[2].span().start(), SourceOffset(10));
	}

	#[test]
	fn test_contiguous_eager_emission() {
		use crate::{SourceOffset, Token};
		let bump = Bump::default();
		let mut output = BumpVec::new_in(&bump);
		{
			let mut ordered_sink = CursorOrderedSink::new(&bump, &mut output);
			// Create cursors that form a contiguous sequence
			let cursor_at_0 = Cursor::new(SourceOffset(0), Token::SPACE);
			let cursor_at_1 = Cursor::new(SourceOffset(1), Token::SPACE);
			ordered_sink.append(cursor_at_0);
			ordered_sink.append(cursor_at_1);
			// Avoid flushing as they should be emitted immediately
		}
		assert_eq!(output.len(), 2);
		assert_eq!(output[0].span().start(), SourceOffset(0));
		assert_eq!(output[1].span().start(), SourceOffset(1));
	}

	#[test]
	fn test_gap_filling() {
		use crate::{SourceOffset, Token};
		let bump = Bump::default();
		let mut output = BumpVec::new_in(&bump);

		{
			let mut ordered_sink = CursorOrderedSink::new(&bump, &mut output);
			// Create cursors with a gap, then fill the gap
			let cursor_at_0 = Cursor::new(SourceOffset(0), Token::SPACE); // ends at 1
			let cursor_at_2 = Cursor::new(SourceOffset(2), Token::SPACE); // ends at 3
			let cursor_at_1 = Cursor::new(SourceOffset(1), Token::SPACE); // ends at 2, fills the gap
			ordered_sink.append(cursor_at_0);
			ordered_sink.append(cursor_at_2);
			ordered_sink.append(cursor_at_1);
			// Avoid flushing as they should be emitted immediately
		}
		assert_eq!(output.len(), 3);
		assert_eq!(output[0].span().start(), SourceOffset(0));
		assert_eq!(output[1].span().start(), SourceOffset(1));
		assert_eq!(output[2].span().start(), SourceOffset(2));
	}

	#[test]
	fn test_sequential() {
		use crate::{SourceOffset, Token};
		let bump = Bump::default();
		let mut output = BumpVec::new_in(&bump);
		{
			let mut ordered_sink = CursorOrderedSink::new(&bump, &mut output);
			let cursor1 = Cursor::new(SourceOffset(0), Token::SPACE);
			let cursor2 = Cursor::new(SourceOffset(1), Token::SPACE);
			let cursor3 = Cursor::new(SourceOffset(2), Token::SPACE);
			ordered_sink.append(cursor1);
			ordered_sink.append(cursor2);
			ordered_sink.append(cursor3);
			// Avoid flushing as they should be emitted immediately
		}
		assert_eq!(output.len(), 3);
		assert_eq!(output[0].span().start(), SourceOffset(0));
		assert_eq!(output[1].span().start(), SourceOffset(1));
		assert_eq!(output[2].span().start(), SourceOffset(2));
	}

	#[test]
	fn test_varied_order() {
		use crate::{SourceOffset, Token};
		let bump = Bump::default();
		let mut output = BumpVec::new_in(&bump);
		{
			let mut ordered_sink = CursorOrderedSink::new(&bump, &mut output);
			let cursor_at_4 = Cursor::new(SourceOffset(4), Token::SPACE); // ends at 5
			let cursor_at_0 = Cursor::new(SourceOffset(0), Token::SPACE); // ends at 1
			let cursor_at_6 = Cursor::new(SourceOffset(6), Token::SPACE); // ends at 7
			let cursor_at_2 = Cursor::new(SourceOffset(2), Token::SPACE); // ends at 3
			let cursor_at_1 = Cursor::new(SourceOffset(1), Token::SPACE); // ends at 2
			let cursor_at_3 = Cursor::new(SourceOffset(3), Token::SPACE); // ends at 4
			let cursor_at_5 = Cursor::new(SourceOffset(5), Token::SPACE); // ends at 6
			ordered_sink.append(cursor_at_4);
			ordered_sink.append(cursor_at_0);
			ordered_sink.append(cursor_at_6);
			ordered_sink.append(cursor_at_2);
			ordered_sink.append(cursor_at_1);
			ordered_sink.append(cursor_at_3);
			ordered_sink.append(cursor_at_5);
			// Avoid flushing as they should be emitted immediately
		}
		assert_eq!(output.len(), 7);
		for i in 0..7 {
			assert_eq!(output[i].span().start(), SourceOffset(i as u32));
		}
	}
}
