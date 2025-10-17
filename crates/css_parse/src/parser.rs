use crate::{
	Cursor, Diagnostic, Feature, Kind, KindSet, ParserCheckpoint, ParserReturn, Result, SourceOffset, ToCursors,
	traits::{Parse, Peek},
};
use bitmask_enum::bitmask;
use bumpalo::{Bump, collections::Vec};
use css_lexer::{AtomSet, DynAtomSet, SourceCursor};
use std::mem;

// This is chosen rather arbitrarily, but:
// - It needs to be a number larger than BUFFER_REFILL_INDEX (the largest `peek_n` distance we currently peek).
// - It would be nice to keep Parser aligned to 64. It's not moved/copied... ever, so struct size doesn't really matter
//   but making it, say, 1000, doesn't really improve performance. Always benchmark when changing!
const BUFFER_LEN: usize = 12;
// This number is chosen specifically because we peek_n(5) at most. Ensuring the buffer is always full enough that
// peeks only use the buffer and don't end up cloning the lexer. While cloning the lexer is quite cheap, it's definitely
// cheaper to simply look into the buffer. If we ever peek more than 5 tokens, we should change this number.
const BUFFER_REFILL_INDEX: usize = BUFFER_LEN - 5;

#[derive(Debug)]
pub struct Parser<'a, I: Iterator<Item = Cursor> + Clone> {
	pub(crate) source_text: &'a str,

	pub(crate) cursor_iter: I,

	#[allow(dead_code)]
	pub(crate) features: Feature,

	pub(crate) errors: Vec<'a, Diagnostic>,

	pub(crate) trivia: Vec<'a, (Vec<'a, Cursor>, Cursor)>,

	pub(crate) state: State,

	pub(crate) bump: &'a Bump,

	skip: KindSet,

	stop: KindSet,

	buffer: [Cursor; BUFFER_LEN],
	buffer_index: usize,

	#[cfg(debug_assertions)]
	pub(crate) last_cursor: Option<Cursor>,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[derive(Default)]
pub enum State {
	Nested = 0b0000_0001,
}

#[inline]
fn eof_cursor(len: usize) -> Cursor {
	let eof_offset = css_lexer::SourceOffset(len as u32);
	Cursor::new(eof_offset, css_lexer::Token::EOF)
}

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = Cursor> + Clone,
{
	/// Create a new parser with an iterator over cursors
	pub fn new(bump: &'a Bump, source_text: &'a str, mut cursor_iter: I) -> Self {
		let eof_cursor = eof_cursor(source_text.len());
		let mut buffer = [eof_cursor; BUFFER_LEN];
		buffer.fill_with(|| cursor_iter.next().unwrap_or(eof_cursor));

		Self {
			source_text,
			cursor_iter,
			features: Feature::none(),
			errors: Vec::new_in(bump),
			trivia: Vec::new_in(bump),
			state: State::none(),
			skip: KindSet::TRIVIA,
			stop: KindSet::NONE,
			buffer,
			buffer_index: 0,
			bump,
			#[cfg(debug_assertions)]
			last_cursor: None,
		}
	}

	pub fn with_features(mut self, features: Feature) -> Self {
		self.features = features;
		self
	}

	fn fill_buffer(&mut self, from: usize) {
		// Shift remaining buffer cursors left to the start of the slice.
		self.buffer.copy_within(from..BUFFER_LEN, 0);
		// Re-fill the buffer with new cursors.
		let eof = eof_cursor(self.source_text.len());
		for i in BUFFER_LEN - from..BUFFER_LEN {
			self.buffer[i] = self.cursor_iter.next().unwrap_or(eof);
		}
		self.buffer_index = 0;
	}

	#[inline]
	pub fn bump(&self) -> &'a Bump {
		self.bump
	}

	#[inline]
	pub fn enabled(&self, other: Feature) -> bool {
		self.features.contains(other)
	}

	#[inline]
	pub fn is(&self, state: State) -> bool {
		self.state.contains(state)
	}

	#[inline]
	pub fn set_state(&mut self, state: State) -> State {
		let old = self.state;
		self.state = state;
		old
	}

	#[inline]
	pub fn set_skip(&mut self, skip: KindSet) -> KindSet {
		let old = self.skip;
		self.skip = skip;
		old
	}

	#[inline]
	pub fn set_stop(&mut self, stop: KindSet) -> KindSet {
		let old = self.stop;
		self.stop = stop;
		old
	}

	pub fn parse_entirely<T: Parse<'a> + ToCursors>(&mut self) -> ParserReturn<'a, T> {
		let output = match T::parse(self) {
			Ok(output) => Some(output),
			Err(error) => {
				self.errors.push(error);
				None
			}
		};
		let remaining_non_trivia = !self.at_end() && self.peek_n(1) != Kind::Eof;
		let at_end = self.peek_n_with_skip(1, KindSet::NONE) == Kind::Eof;

		if !at_end {
			let start = self.peek_n_with_skip(1, KindSet::NONE);
			let mut end;
			loop {
				end = self.next();
				if end == Kind::Eof {
					break;
				}
			}
			if remaining_non_trivia {
				self.errors.push(Diagnostic::new(start, Diagnostic::expected_end).with_end_cursor(end));
			}
		}
		let errors = mem::replace(&mut self.errors, Vec::new_in(self.bump));
		let trivia = mem::replace(&mut self.trivia, Vec::new_in(self.bump));
		ParserReturn::new(output, self.source_text, errors, trivia)
	}

	pub fn parse<T: Parse<'a>>(&mut self) -> Result<T> {
		T::parse(self)
	}

	pub fn peek<T: Peek<'a>>(&self) -> bool {
		T::peek(self, self.peek_n(1))
	}

	pub fn parse_if_peek<T: Peek<'a> + Parse<'a>>(&mut self) -> Result<Option<T>> {
		if T::peek(self, self.peek_n(1)) { T::parse(self).map(Some) } else { Ok(None) }
	}

	pub fn try_parse<T: Parse<'a>>(&mut self) -> Result<T> {
		T::try_parse(self)
	}

	pub fn try_parse_if_peek<T: Peek<'a> + Parse<'a>>(&mut self) -> Result<Option<T>> {
		if T::peek(self, self.peek_n(1)) { T::try_parse(self).map(Some) } else { Ok(None) }
	}

	pub fn equals_atom(&self, c: Cursor, atom: &'static dyn DynAtomSet) -> bool {
		let mut cursor_bits = c.atom_bits();
		if cursor_bits == 0 {
			let source_cursor = self.to_source_cursor(c);
			cursor_bits = atom.str_to_bits(&source_cursor.parse(self.bump));
		}
		cursor_bits == atom.bits()
	}

	pub fn to_atom<A: AtomSet + PartialEq>(&self, c: Cursor) -> A {
		let bits = c.atom_bits();
		if bits == 0 {
			let source_cursor = self.to_source_cursor(c);
			return A::from_str(&source_cursor.parse(self.bump));
		}
		#[cfg(debug_assertions)]
		{
			let source_cursor = self.to_source_cursor(c);
			debug_assert!(
				A::from_bits(bits) == A::from_str(&source_cursor.parse(self.bump)),
				"{:?} -> {:?} != {:?} ({:?})",
				c,
				A::from_bits(bits),
				A::from_str(&source_cursor.parse(self.bump)),
				source_cursor.parse(self.bump)
			);
		}
		A::from_bits(bits)
	}

	#[inline(always)]
	pub fn offset(&self) -> SourceOffset {
		self.buffer[self.buffer_index].offset()
	}

	#[inline(always)]
	pub fn at_end(&self) -> bool {
		self.buffer[self.buffer_index] == Kind::Eof
	}

	pub fn rewind(&mut self, checkpoint: ParserCheckpoint<I>) {
		let ParserCheckpoint { iter, errors_pos, trivia_pos, buffer, buffer_index, .. } = checkpoint;

		self.cursor_iter = iter;

		self.errors.truncate(errors_pos as usize);
		self.trivia.truncate(trivia_pos as usize);

		self.buffer = buffer;
		self.buffer_index = buffer_index;

		#[cfg(debug_assertions)]
		{
			self.last_cursor = None;
		}
	}

	#[inline]
	pub fn checkpoint(&self) -> ParserCheckpoint<I> {
		ParserCheckpoint {
			cursor: self.buffer[self.buffer_index],
			errors_pos: self.errors.len() as u8,
			trivia_pos: self.trivia.len() as u16,
			iter: self.cursor_iter.clone(),
			buffer: self.buffer,
			buffer_index: self.buffer_index,
		}
	}

	#[inline]
	pub fn next_is_stop(&self) -> bool {
		for c in &self.buffer[self.buffer_index..BUFFER_LEN] {
			if c != self.skip {
				return c == self.stop;
			}
		}

		let mut iter = self.cursor_iter.clone();
		loop {
			let Some(cursor) = iter.next() else {
				return false;
			};
			if cursor != self.skip {
				return cursor == self.stop;
			}
		}
	}

	#[inline]
	pub(crate) fn peek_n_with_skip(&self, n: u8, skip: KindSet) -> Cursor {
		let mut remaining = n;

		for c in &self.buffer[self.buffer_index..BUFFER_LEN] {
			if c == Kind::Eof {
				return *c;
			}
			if c != skip {
				remaining -= 1;
				if remaining == 0 {
					return *c;
				}
			}
		}

		let mut iter = self.cursor_iter.clone();
		loop {
			let Some(cursor) = iter.next() else {
				return eof_cursor(self.source_text.len());
			};
			if cursor == Kind::Eof {
				return cursor;
			}
			if cursor != skip {
				remaining -= 1;
				if remaining == 0 {
					return cursor;
				}
			}
		}
	}

	#[inline]
	pub fn peek_n(&self, n: u8) -> Cursor {
		self.peek_n_with_skip(n, self.skip)
	}

	pub fn to_source_cursor(&self, cursor: Cursor) -> SourceCursor<'a> {
		SourceCursor::from(cursor, cursor.str_slice(self.source_text))
	}

	pub fn consume_trivia(&mut self) -> Vec<'a, Cursor> {
		let mut trivia = Vec::new_in(self.bump);
		for i in self.buffer_index..BUFFER_LEN {
			let c = self.buffer[i];
			if c == Kind::Eof {
				return trivia;
			} else if c == self.skip {
				trivia.push(c)
			} else {
				self.fill_buffer(i);
				return trivia;
			}
		}

		loop {
			let Some(c) = self.cursor_iter.next() else {
				return trivia;
			};
			if c == Kind::Eof {
				return trivia;
			} else if c == self.skip {
				trivia.push(c)
			} else {
				let eof = eof_cursor(self.source_text.len());
				self.buffer[0] = c;
				for i in 1..BUFFER_LEN {
					self.buffer[i] = self.cursor_iter.next().unwrap_or(eof);
				}
				self.buffer_index = 0;
				return trivia;
			}
		}
	}

	#[allow(clippy::should_implement_trait)]
	pub fn next(&mut self) -> Cursor {
		// Collect trivia that should be associated with the next content token
		let mut pending_trivia = Vec::new_in(self.bump);

		if self.buffer_index >= BUFFER_REFILL_INDEX {
			self.fill_buffer(self.buffer_index);
		}

		for i in self.buffer_index..BUFFER_LEN {
			let c = self.buffer[i];
			if c == Kind::Eof {
				self.buffer_index = i + 1;
				// Associate pending trivia with EOF if any
				if !pending_trivia.is_empty() {
					self.trivia.push((pending_trivia.clone(), c));
				}
				return c;
			} else if c == self.skip {
				pending_trivia.push(c);
				self.buffer_index = i + 1;
			} else {
				self.buffer_index = i + 1;
				// Associate all pending trivia with this content token
				if !pending_trivia.is_empty() {
					self.trivia.push((pending_trivia.clone(), c));
				}
				return c;
			}
		}

		let c;
		loop {
			let Some(cursor) = self.cursor_iter.next() else {
				let eof_cursor = eof_cursor(self.source_text.len());
				if !pending_trivia.is_empty() {
					self.trivia.push((pending_trivia.clone(), eof_cursor));
				}
				return eof_cursor;
			};
			if cursor == Kind::Eof || cursor != self.skip {
				c = cursor;
				break;
			}
			pending_trivia.push(cursor);
		}

		// Associate pending trivia with the content token we found
		if !pending_trivia.is_empty() {
			self.trivia.push((pending_trivia.clone(), c));
		}

		#[cfg(debug_assertions)]
		if let Some(last_cursor) = self.last_cursor {
			debug_assert!(last_cursor != c, "Detected a next loop, {c:?} was fetched twice");
		}
		#[cfg(debug_assertions)]
		if c == Kind::Eof {
			self.last_cursor = None;
		} else {
			self.last_cursor = Some(c);
		}

		c
	}
}

#[test]
fn peek_and_next() {
	let str = "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21";
	let bump = bumpalo::Bump::default();
	let lexer = css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, &str);
	let mut p = Parser::new(&bump, &str, lexer);
	assert_eq!(p.at_end(), false);
	assert_eq!(p.offset(), 0);
	for n in 0..=1 {
		let c = p.checkpoint();
		for i in 0..=19 {
			let c = p.peek_n(1);
			assert_eq!(c.token(), Kind::Number);
			assert_eq!(c.token().value(), i as f32);
			let c = p.peek_n(2);
			assert_eq!(c.token(), Kind::Number);
			assert_eq!(c.token().value(), (i + 1) as f32);
			let c = p.peek_n(3);
			assert_eq!(c.token(), Kind::Number);
			assert_eq!(c.token().value(), (i + 2) as f32);
			let c = p.next();
			assert_eq!(c.token().value(), i as f32);
			let c = p.peek_n(1);
			assert_eq!(c.token(), Kind::Number);
			assert_eq!(c.token().value(), (i + 1) as f32);
		}
		if n == 0 {
			p.rewind(c)
		}
	}
	let c = p.next();
	assert_eq!(c.token(), Kind::Number);
	assert_eq!(c.token().value(), 20.0);
	let c = p.next();
	assert_eq!(c.token(), Kind::Number);
	assert_eq!(c.token().value(), 21.0);
	let c = p.next();
	assert_eq!(c.token(), Kind::Eof);
}

#[test]
fn peek_and_next_with_whitsespace() {
	let str = "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21";
	let bump = bumpalo::Bump::default();
	let lexer = css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, &str);
	let mut p = Parser::new(&bump, &str, lexer);
	p.set_skip(KindSet::COMMENTS);
	assert_eq!(p.at_end(), false);
	assert_eq!(p.offset(), 0);
	for n in 0..=1 {
		let c = p.checkpoint();
		for i in 0..=19 {
			let c = p.peek_n(1);
			assert_eq!(c.token(), Kind::Number);
			assert_eq!(c.token().value(), i as f32);
			let c = p.peek_n(2);
			assert_eq!(c.token(), Kind::Whitespace);
			let c = p.peek_n(3);
			assert_eq!(c.token(), Kind::Number);
			assert_eq!(c.token().value(), (i + 1) as f32);
			let c = p.peek_n(4);
			assert_eq!(c.token(), Kind::Whitespace);
			let c = p.peek_n(5);
			assert_eq!(c.token(), Kind::Number);
			assert_eq!(c.token().value(), (i + 2) as f32);
			let c = p.next();
			assert_eq!(c.token().value(), i as f32);
			let c = p.peek_n(1);
			assert_eq!(c.token(), Kind::Whitespace);
			let c = p.peek_n(2);
			assert_eq!(c.token(), Kind::Number);
			assert_eq!(c.token().value(), (i + 1) as f32);
			p.next();
		}
		if n == 0 {
			p.rewind(c);
		}
	}
	let c = p.next();
	assert_eq!(c.token(), Kind::Number);
	assert_eq!(c.token().value(), 20.0);
	let c = p.next();
	assert_eq!(c.token(), Kind::Whitespace);
	let c = p.next();
	assert_eq!(c.token(), Kind::Number);
	assert_eq!(c.token().value(), 21.0);
	let c = p.next();
	assert_eq!(c.token(), Kind::Eof);
}
