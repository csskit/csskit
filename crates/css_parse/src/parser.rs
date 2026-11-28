use crate::{
	Arena, Cursor, Diagnostic, Feature, Kind, KindSet, ParserCheckpoint, ParserReturn, Result, SourceOffset, ToCursors,
	Token, Vec,
	traits::{Parse, ParseInfallible, Peek},
};
use atom_set::{AtomSet, DynAtomSet};
use bitmask_enum::bitmask;
use css_lexer::SourceCursor;
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

	base_offset: u32,

	pub(crate) cursor_iter: I,

	#[allow(dead_code)]
	pub(crate) features: Feature,

	pub(crate) errors: Vec<'a, Diagnostic>,

	pub(crate) trivia: Vec<'a, (Vec<'a, Cursor>, Cursor)>,

	pub(crate) state: State,

	pub(crate) alloc: &'a Arena,

	skip: KindSet,

	stop: KindSet,

	buffer: [Cursor; BUFFER_LEN],
	buffer_index: usize,

	/// Nesting depth of substitution functions (`var()`, `env()`, etc.) currently being parsed.
	/// Guards against stack overflow from deeply-nested fallbacks like `var(--a, var(--a, ...))`.
	substitution_depth: u8,

	#[cfg(debug_assertions)]
	pub(crate) last_cursor: Option<Cursor>,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[derive(Default)]
pub enum State {
	Nested = 0b0000_0001,
	/// Disallow relative selectors (:has). Set when inside :has() since nested :has() is invalid.
	DisallowRelativeSelector = 0b0000_0010,
}

#[inline]
fn eof_cursor(base_offset: u32, len: usize) -> Cursor {
	let eof_offset = css_lexer::SourceOffset(base_offset + len as u32);
	Cursor::new(eof_offset, css_lexer::Token::EOF)
}

impl<'a, I> Parser<'a, I>
where
	I: Iterator<Item = Cursor> + Clone,
{
	/// Create a new parser with an iterator over cursors
	pub fn new(alloc: &'a Arena, source_text: &'a str, cursor_iter: I) -> Self {
		Self::new_at(alloc, source_text, SourceOffset(0), cursor_iter)
	}

	/// Create a new parser over `source_text` whose first byte sits at `base_offset`, so the cursors of
	/// `cursor_iter` take offsets from `base_offset` rather than from zero.
	pub fn new_at(alloc: &'a Arena, source_text: &'a str, base_offset: SourceOffset, mut cursor_iter: I) -> Self {
		let eof_cursor = eof_cursor(base_offset.0, source_text.len());
		let mut buffer = [eof_cursor; BUFFER_LEN];
		buffer.fill_with(|| cursor_iter.next().unwrap_or(eof_cursor));

		Self {
			source_text,
			base_offset: base_offset.0,
			cursor_iter,
			features: Feature::none(),
			errors: Vec::new_in(alloc),
			trivia: Vec::new_in(alloc),
			state: State::none(),
			skip: KindSet::TRIVIA,
			stop: KindSet::NONE,
			buffer,
			buffer_index: 0,
			substitution_depth: 0,
			alloc,
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
		let eof = eof_cursor(self.base_offset, self.source_text.len());
		for i in BUFFER_LEN - from..BUFFER_LEN {
			self.buffer[i] = self.cursor_iter.next().unwrap_or(eof);
		}
		self.buffer_index = 0;
	}

	#[inline]
	pub fn alloc(&self) -> &'a Arena {
		self.alloc
	}

	/// Maximum nesting depth of substitution functions before parsing bails to `Unresolved`.
	pub const MAX_SUBSTITUTION_DEPTH: u8 = 32;

	/// Enters a substitution-function parse scope, incrementing the depth counter.
	///
	/// Returns `false` if the depth limit ([`Self::MAX_SUBSTITUTION_DEPTH`]) would be exceeded;
	/// callers should then consume the tokens as an unresolved token sequence instead of recursing.
	/// On success, callers MUST call [`Self::exit_substitution`] once parsing of the scope ends.
	#[inline]
	#[must_use]
	pub fn enter_substitution(&mut self) -> bool {
		if self.substitution_depth >= Self::MAX_SUBSTITUTION_DEPTH {
			return false;
		}
		self.substitution_depth += 1;
		true
	}

	/// Exits a substitution-function parse scope, decrementing the depth counter.
	#[inline]
	pub fn exit_substitution(&mut self) {
		debug_assert!(self.substitution_depth > 0);
		self.substitution_depth = self.substitution_depth.saturating_sub(1);
	}

	#[inline]
	pub fn enabled(&self, other: Feature) -> bool {
		self.features.contains(other)
	}

	/// The full source text the parser is reading from.
	#[inline]
	pub fn source_text(&self) -> &'a str {
		self.source_text
	}

	/// The source text from `offset` to the end.
	#[inline]
	pub fn source_from(&self, offset: SourceOffset) -> &'a str {
		&self.source_text[(offset.0 - self.base_offset) as usize..]
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
		let errors = mem::replace(&mut self.errors, Vec::new_in(self.alloc));
		let trivia = mem::replace(&mut self.trivia, Vec::new_in(self.alloc));
		ParserReturn::new(output, self.source_text, errors, trivia)
	}

	pub fn parse<T: Parse<'a>>(&mut self) -> Result<T> {
		T::parse(self)
	}

	pub fn parse_infallible<T: ParseInfallible<'a>>(&mut self) -> T {
		T::parse_infallible(self)
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

	/// Records a diagnostic without ending the parse. Use this when a node recovers from an error and keeps the
	/// tokens it could not parse.
	#[inline]
	pub fn diagnose(&mut self, diagnostic: Diagnostic) {
		self.errors.push(diagnostic);
	}

	/// Returns the first diagnostic recorded after `checkpoint`, if any. A parse that recorded a diagnostic recovered
	/// from an error, so its result is not clean.
	#[inline]
	pub fn diagnostic_since(&self, checkpoint: &ParserCheckpoint<I>) -> Option<Diagnostic> {
		self.errors.get(checkpoint.errors_pos as usize).copied()
	}

	/// Records that a token of `kind` is missing before the next token and returns a zero-length cursor which stands
	/// in for it. The cursor sits at the next token's offset so it keeps its place when cursors are written in
	/// source order.
	pub fn missing(&mut self, kind: Kind) -> Cursor {
		let next = self.peek_n_with_skip(1, KindSet::NONE);
		let cursor = Cursor::new(next.offset(), Token::dummy(kind));
		self.diagnose(Diagnostic::new(next, Diagnostic::missing).with_desired_cursor(cursor));
		cursor
	}

	pub fn equals_atom(&self, c: Cursor, atom: &'static dyn DynAtomSet) -> bool {
		let mut cursor_bits = c.token().atom_bits();
		if cursor_bits == 0 {
			if c != KindSet::ATOM_LIKE {
				return false;
			}
			let source_cursor = self.to_source_cursor(c);
			cursor_bits = atom.str_to_bits(&source_cursor.parse(self.alloc));
		}
		cursor_bits == atom.bits()
	}

	pub fn to_atom<A: AtomSet + PartialEq>(&self, c: Cursor) -> A {
		let bits = c.token().atom_bits();
		if bits == 0 {
			if c != KindSet::ATOM_LIKE {
				return A::from_bits(0);
			}
			let source_cursor = self.to_source_cursor(c);
			return A::from_str(&source_cursor.parse(self.alloc));
		}
		#[cfg(debug_assertions)]
		if c == KindSet::ATOM_LIKE && c != Kind::Dimension {
			let is_dashed = c.token().is_dashed_ident();
			let source_cursor = self.to_source_cursor(c);
			let text = source_cursor.parse(self.alloc);
			let comparable = if is_dashed { &text[2..] } else { &text[..] };
			debug_assert!(
				A::from_bits(bits) == A::from_str(comparable),
				"{:?} -> {:?} != {:?} ({:?})",
				c,
				A::from_bits(bits),
				A::from_str(comparable),
				comparable
			);
		}
		A::from_bits(bits)
	}

	#[inline]
	pub fn offset(&self) -> SourceOffset {
		self.buffer[self.buffer_index].offset()
	}

	#[inline]
	pub fn at_end(&self) -> bool {
		self.buffer[self.buffer_index] == Kind::Eof
	}

	pub fn rewind(&mut self, checkpoint: ParserCheckpoint<I>) {
		let ParserCheckpoint { iter, errors_pos, trivia_pos, buffer, buffer_index, skip, stop, state, .. } = checkpoint;

		self.cursor_iter = iter;

		self.errors.truncate(errors_pos as usize);
		self.trivia.truncate(trivia_pos as usize);

		self.buffer = buffer;
		self.buffer_index = buffer_index;

		self.skip = skip;
		self.stop = stop;
		self.state = state;

		#[cfg(debug_assertions)]
		{
			self.last_cursor = None;
		}
	}

	#[inline]
	pub fn checkpoint(&self) -> ParserCheckpoint<I> {
		ParserCheckpoint {
			cursor: self.buffer[self.buffer_index],
			errors_pos: self.errors.len() as u32,
			trivia_pos: self.trivia.len() as u32,
			iter: self.cursor_iter.clone(),
			buffer: self.buffer,
			buffer_index: self.buffer_index,
			skip: self.skip,
			stop: self.stop,
			state: self.state,
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
		self.peek_n_from_iter(remaining, skip)
	}

	#[inline(never)]
	fn peek_n_from_iter(&self, mut remaining: u8, skip: KindSet) -> Cursor {
		let mut iter = self.cursor_iter.clone();
		loop {
			let Some(cursor) = iter.next() else {
				return eof_cursor(self.base_offset, self.source_text.len());
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

	#[inline]
	pub fn peek_n_including_whitespace(&self, n: u8) -> Cursor {
		self.peek_n_with_skip(n, self.skip.remove(Kind::Whitespace))
	}

	pub fn to_source_cursor(&self, cursor: Cursor) -> SourceCursor<'a> {
		let start = (cursor.offset().0 - self.base_offset) as usize;
		SourceCursor::from(cursor, &self.source_text[start..start + cursor.len() as usize])
	}

	pub fn consume_trivia(&mut self) -> Vec<'a, Cursor> {
		let mut trivia = Vec::new_in(self.alloc);
		self.advance_past_trivia(|c| trivia.push(c));
		trivia
	}

	/// Advances past trivia without collecting it.
	pub fn skip_trivia(&mut self) {
		self.advance_past_trivia(|_| {});
	}

	fn advance_past_trivia(&mut self, mut on_trivia: impl FnMut(Cursor)) {
		for i in self.buffer_index..BUFFER_LEN {
			let c = self.buffer[i];
			if c == Kind::Eof {
				self.buffer_index = i;
				return;
			} else if c == self.skip {
				on_trivia(c)
			} else {
				self.buffer_index = i;
				self.fill_buffer(i);
				return;
			}
		}

		let eof = eof_cursor(self.base_offset, self.source_text.len());
		loop {
			let Some(c) = self.cursor_iter.next() else {
				self.buffer = [eof; BUFFER_LEN];
				self.buffer_index = 0;
				return;
			};
			if c == Kind::Eof {
				self.buffer = [eof; BUFFER_LEN];
				self.buffer_index = 0;
				return;
			} else if c == self.skip {
				on_trivia(c)
			} else {
				self.buffer[0] = c;
				for i in 1..BUFFER_LEN {
					self.buffer[i] = self.cursor_iter.next().unwrap_or(eof);
				}
				self.buffer_index = 0;
				return;
			}
		}
	}

	/// Consume trivia and attach it to the next content token for output preservation.
	/// This should be called when you want to consume whitespace/comments but preserve
	/// them for round-trip output fidelity.
	pub fn consume_trivia_as_leading(&mut self) {
		let trivia = self.consume_trivia();
		if !trivia.is_empty() {
			// Peek the next content token to attach trivia to it
			let next = self.peek_n(1);
			self.push_trivia(trivia, next);
		}
	}

	fn push_trivia(&mut self, trivia: Vec<'a, Cursor>, next: Cursor) {
		let len = self.trivia.len();
		if len == self.trivia.capacity() {
			let source_len = self.source_text.len() as u64;
			let consumed = u64::from(next.offset().0).max(1);
			let projected = len as u64 * source_len / consumed;
			let target =
				(projected + projected / 8).clamp(2 * len as u64, 8 * len as u64).min(source_len).max(len as u64 + 1);
			self.trivia.reserve(usize::try_from(target).unwrap_or(usize::MAX) - len);
		}
		self.trivia.push((trivia, next));
	}

	#[allow(clippy::should_implement_trait)]
	#[inline]
	pub fn next(&mut self) -> Cursor {
		let index = self.buffer_index;
		if index + 1 < BUFFER_REFILL_INDEX {
			let cursor = self.buffer[index];
			if cursor != Kind::Eof && cursor != self.skip {
				self.buffer_index = index + 1;
				self.track_last_cursor(cursor);
				return cursor;
			}
		}
		self.next_with_trivia()
	}

	#[inline(always)]
	fn track_last_cursor(&mut self, _cursor: Cursor) {
		#[cfg(debug_assertions)]
		{
			if let Some(last_cursor) = self.last_cursor {
				debug_assert!(last_cursor != _cursor, "Detected a next loop, {_cursor:?} was fetched twice");
			}
			self.last_cursor = Some(_cursor);
		}
	}

	#[inline(never)]
	fn next_with_trivia(&mut self) -> Cursor {
		// Collect trivia that should be associated with the next content token
		let mut pending_trivia = Vec::new_in(self.alloc);

		loop {
			if self.buffer_index >= BUFFER_REFILL_INDEX {
				self.fill_buffer(self.buffer_index);
			}

			for i in self.buffer_index..BUFFER_LEN {
				let c = self.buffer[i];
				if c == Kind::Eof {
					self.buffer_index = i;
					// Associate pending trivia with EOF if any
					if !pending_trivia.is_empty() {
						self.push_trivia(pending_trivia, c);
					}
					#[cfg(debug_assertions)]
					{
						self.last_cursor = None;
					}
					return c;
				} else if c == self.skip {
					pending_trivia.push(c);
				} else {
					self.buffer_index = i + 1;
					if self.buffer_index >= BUFFER_REFILL_INDEX {
						self.fill_buffer(self.buffer_index);
					}
					// Associate all pending trivia with this content token
					if !pending_trivia.is_empty() {
						self.push_trivia(pending_trivia, c);
					}
					self.track_last_cursor(c);
					return c;
				}
			}

			// Buffer exhausted with only skip tokens. Refill so buffer_index stays valid.
			self.fill_buffer(BUFFER_LEN);
		}
	}
}

#[test]
fn test_filling_buffer_with_skip_tokens() {
	let str = "/*x*//*x*//*x*//*x*//*x*//*x*//*x*//*x*//*x*//*x*//*x*/a";
	let alloc = crate::Arena::default();
	let lexer = css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, str);
	let mut p = Parser::new(&alloc, str, lexer);
	let c = p.next();
	assert_eq!(c.token(), Kind::Ident);
	// Must not panic:
	let _ = p.at_end();
	let _ = p.offset();
}

#[test]
fn peek_and_next() {
	let str = "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21";
	let alloc = crate::Arena::default();
	let lexer = css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, str);
	let mut p = Parser::new(&alloc, str, lexer);
	assert!(!p.at_end());
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
	let alloc = crate::Arena::default();
	let lexer = css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, str);
	let mut p = Parser::new(&alloc, str, lexer);
	p.set_skip(KindSet::COMMENTS);
	assert!(!p.at_end());
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

#[test]
fn new_at_offsets_cursors_from_base() {
	let str = "a b";
	let alloc = crate::Arena::default();
	let lexer = css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, str)
		.map(|cursor| Cursor::new(SourceOffset(cursor.offset().0 + 100), cursor.token()));
	let mut p = Parser::new_at(&alloc, str, SourceOffset(100), lexer);
	assert_eq!(p.offset(), 100);
	assert_eq!(p.source_from(SourceOffset(101)), " b");
	let first = p.next();
	assert_eq!(first.offset(), 100);
	assert_eq!(p.to_source_cursor(first).source(), "a");
	let second = p.next();
	assert_eq!(second.offset(), 102);
	assert_eq!(p.to_source_cursor(second).source(), "b");
	assert_eq!(p.source_from(second.end_offset()), "");
	let eof = p.next();
	assert_eq!(eof.token(), Kind::Eof);
	assert_eq!(eof.offset(), 103);
}

#[test]
fn new_at_eof_after_buffer_refill() {
	let str = "0 1 2 3 4 5 6 7 8 9 10 11 12 13 14 15 16 17 18 19 20 21";
	let end = 1000 + str.len() as u32;
	let alloc = crate::Arena::default();
	let lexer = css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, str)
		.map(|cursor| Cursor::new(SourceOffset(cursor.offset().0 + 1000), cursor.token()));
	let mut p = Parser::new_at(&alloc, str, SourceOffset(1000), lexer);
	let peeked = p.peek_n(23);
	assert_eq!(peeked.token(), Kind::Eof);
	assert_eq!(peeked.offset(), end);
	for i in 0..=21 {
		let number = p.next();
		assert_eq!(p.to_source_cursor(number).source(), i.to_string());
	}
	let eof = p.next();
	assert_eq!(eof.token(), Kind::Eof);
	assert_eq!(eof.offset(), end);
}

#[test]
fn peek_n_matches_token_sequence() {
	let str = "a /**/ 1 /**//**//**//**//**//**//**//**//**//**//**//**//**/ b  , /**/ c 2 3 4 5 6 7 8 9 10 11 12 13 d";
	let tokens: std::vec::Vec<Cursor> = css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, str).collect();
	for skip in [KindSet::NONE, KindSet::COMMENTS, KindSet::WHITESPACE, KindSet::TRIVIA] {
		let alloc = crate::Arena::default();
		let lexer = css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, str);
		let mut p = Parser::new(&alloc, str, lexer);
		p.set_skip(skip);
		let expected: std::vec::Vec<Cursor> = tokens.iter().copied().filter(|cursor| *cursor != skip).collect();
		for position in 0..expected.len() {
			for n in 1..=16u8 {
				let got = p.peek_n(n);
				match expected.get(position + n as usize - 1) {
					Some(want) => assert_eq!(
						(got.offset(), got.token()),
						(want.offset(), want.token()),
						"skip {skip:?} position {position} n {n}"
					),
					None => assert_eq!(
						(got.offset().0 as usize, got.token().kind()),
						(str.len(), Kind::Eof),
						"skip {skip:?} position {position} n {n}"
					),
				}
			}
			p.next();
		}
	}
}

#[test]
fn next_matches_next_with_trivia() {
	let str =
		"a /**/ 1 /**//**//**//**//**//**//**//**//**//**//**//**//**/ b  , /**/ c 2 3 4 5 6 7 8 9 10 11 12 13 d 'x";
	for skip in [KindSet::NONE, KindSet::COMMENTS, KindSet::WHITESPACE, KindSet::TRIVIA] {
		let alloc = crate::Arena::default();
		let mut fast = Parser::new(&alloc, str, css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, str));
		let mut slow = Parser::new(&alloc, str, css_lexer::Lexer::new(&css_lexer::EmptyAtomSet::ATOMS, str));
		fast.set_skip(skip);
		slow.set_skip(skip);
		for step in 0..64 {
			let got = fast.next();
			let want = slow.next_with_trivia();
			assert_eq!((got.offset(), got.token()), (want.offset(), want.token()), "skip {skip:?} step {step}");
			assert_eq!(fast.buffer_index, slow.buffer_index, "skip {skip:?} step {step}");
			assert_eq!(fast.trivia.len(), slow.trivia.len(), "skip {skip:?} step {step}");
			for (fast_trivia, slow_trivia) in fast.trivia.iter().zip(slow.trivia.iter()) {
				assert_eq!(fast_trivia.0.as_slice(), slow_trivia.0.as_slice(), "skip {skip:?} step {step}");
				assert_eq!(fast_trivia.1, slow_trivia.1, "skip {skip:?} step {step}");
			}
		}
	}
}
