use crate::{
	Cursor, Diagnostic, Feature, Kind, KindSet, ParserCheckpoint, ParserReturn, Result, SourceOffset, ToCursors,
	traits::{Parse, Peek},
};
use bitmask_enum::bitmask;
use bumpalo::{Bump, collections::Vec};
use css_lexer::{AtomSet, DynAtomSet, Lexer, SourceCursor};
use std::mem;

#[derive(Debug)]
pub struct Parser<'a> {
	pub(crate) source_text: &'a str,

	pub(crate) lexer: Lexer<'a>,

	#[allow(dead_code)]
	pub(crate) features: Feature,

	pub(crate) errors: Vec<'a, Diagnostic>,

	pub(crate) trivia: Vec<'a, Cursor>,

	pub(crate) state: State,

	pub(crate) bump: &'a Bump,

	skip: KindSet,

	stop: KindSet,

	#[cfg(debug_assertions)]
	pub(crate) last_cursor: Option<Cursor>,
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[derive(Default)]
pub enum State {
	Nested = 0b0000_0001,
}

impl<'a> Parser<'a> {
	/// Create a new parser
	pub fn new(bump: &'a Bump, atoms: &'static dyn DynAtomSet, source_text: &'a str) -> Self {
		Self::new_with_features(bump, atoms, source_text, Feature::none())
	}

	pub fn with_features(mut self, features: Feature) -> Self {
		self.features = features;
		self
	}

	pub fn new_with_features(
		bump: &'a Bump,
		atoms: &'static dyn DynAtomSet,
		source_text: &'a str,
		features: Feature,
	) -> Self {
		Self {
			source_text,
			lexer: Lexer::new_with_features(atoms, source_text, features.into()),
			features,
			errors: Vec::new_in(bump),
			trivia: Vec::new_in(bump),
			state: State::none(),
			skip: KindSet::TRIVIA,
			stop: KindSet::NONE,
			bump,
			#[cfg(debug_assertions)]
			last_cursor: None,
		}
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
		if !self.at_end() && self.peek_next() != Kind::Eof {
			let start = self.peek_next();
			let mut end;
			loop {
				end = self.next();
				self.trivia.push(end);
				if end == Kind::Eof {
					break;
				}
			}
			self.errors.push(Diagnostic::new(start, Diagnostic::expected_end));
		}
		let errors = mem::replace(&mut self.errors, Vec::new_in(self.bump));
		let trivia = mem::replace(&mut self.trivia, Vec::new_in(self.bump));
		ParserReturn::new(output, self.source_text, errors, trivia)
	}

	pub fn parse<T: Parse<'a>>(&mut self) -> Result<T> {
		T::parse(self)
	}

	pub fn peek<T: Peek<'a>>(&self) -> bool {
		T::peek(self, self.peek_next())
	}

	pub fn parse_if_peek<T: Peek<'a> + Parse<'a>>(&mut self) -> Result<Option<T>> {
		if T::peek(self, self.peek_next()) { T::parse(self).map(Some) } else { Ok(None) }
	}

	pub fn try_parse<T: Parse<'a>>(&mut self) -> Result<T> {
		T::try_parse(self)
	}

	pub fn try_parse_if_peek<T: Peek<'a> + Parse<'a>>(&mut self) -> Result<Option<T>> {
		if T::peek(self, self.peek_next()) { T::try_parse(self).map(Some) } else { Ok(None) }
	}

	pub fn equals_atom(&self, c: Cursor, atom: &'static dyn DynAtomSet) -> bool {
		let mut cursor_bits = c.atom_bits();
		if cursor_bits == 0 {
			let source_cursor = self.to_source_cursor(c);
			cursor_bits = atom.str_to_bits(source_cursor.parse(self.bump));
		}
		cursor_bits == atom.bits()
	}

	pub fn to_atom<A: AtomSet + PartialEq>(&self, c: Cursor) -> A {
		let bits = c.atom_bits();
		if bits == 0 {
			let source_cursor = self.to_source_cursor(c);
			return A::from_str(source_cursor.parse(self.bump));
		}
		#[cfg(debug_assertions)]
		{
			let source_cursor = self.to_source_cursor(c);
			debug_assert!(
				A::from_bits(bits) == A::from_str(source_cursor.parse(self.bump)),
				"{:?} -> {:?} != {:?} ({:?})",
				c,
				A::from_bits(bits),
				A::from_str(source_cursor.parse(self.bump)),
				source_cursor.parse(self.bump)
			);
		}
		A::from_bits(bits)
	}

	#[inline(always)]
	pub fn offset(&self) -> SourceOffset {
		self.lexer.offset()
	}

	#[inline(always)]
	pub fn at_end(&self) -> bool {
		self.lexer.at_end()
	}

	pub fn rewind(&mut self, checkpoint: ParserCheckpoint) {
		let ParserCheckpoint { cursor, errors_pos, trivia_pos } = checkpoint;
		self.lexer.rewind(cursor);
		self.errors.truncate(errors_pos as usize);
		self.trivia.truncate(trivia_pos as usize);
		#[cfg(debug_assertions)]
		{
			self.last_cursor = None;
		}
	}

	#[inline]
	pub fn checkpoint(&self) -> ParserCheckpoint {
		ParserCheckpoint {
			cursor: self.lexer.checkpoint(),
			errors_pos: self.errors.len() as u8,
			trivia_pos: self.trivia.len() as u16,
		}
	}

	#[inline]
	pub fn next_is_stop(&self) -> bool {
		let mut lexer = self.lexer.clone();
		loop {
			let t = lexer.advance();
			if t.kind() != self.skip {
				return t.kind() == self.stop;
			}
		}
	}

	#[inline]
	pub(crate) fn peek_next(&self) -> Cursor {
		let mut lexer = self.lexer.clone();
		loop {
			let offset = lexer.offset();
			let t = lexer.advance();
			if t == Kind::Eof || t != self.skip {
				return t.with_cursor(offset);
			}
		}
	}

	#[inline]
	pub(crate) fn peek_next_including_whitespace(&self) -> Cursor {
		let mut lexer = self.lexer.clone();
		loop {
			let offset = lexer.offset();
			let t = lexer.advance();
			if t == Kind::Eof || t == Kind::Whitespace || t != self.skip {
				return t.with_cursor(offset);
			}
		}
	}

	pub fn peek_n(&self, n: u8) -> Cursor {
		let mut lex = self.lexer.clone();
		let mut remaining = n;
		loop {
			let offset = lex.offset();
			let t = lex.advance();
			if t == Kind::Eof {
				return t.with_cursor(offset);
			}
			if t != self.skip {
				remaining -= 1;
				if remaining == 0 {
					return t.with_cursor(offset);
				}
			}
		}
	}

	pub fn to_source_cursor(&self, cursor: Cursor) -> SourceCursor<'a> {
		SourceCursor::from(cursor, cursor.str_slice(self.source_text))
	}

	pub fn consume_trivia(&mut self) {
		loop {
			let offset = self.lexer.offset();
			let c = self.lexer.advance().with_cursor(offset);
			if c == Kind::Eof {
				return;
			} else if c == self.skip {
				self.trivia.push(c)
			} else {
				self.lexer.rewind(c);
				return;
			}
		}
	}

	#[allow(clippy::should_implement_trait)]
	pub fn next(&mut self) -> Cursor {
		let mut c;
		let mut offset;
		loop {
			offset = self.offset();
			c = self.lexer.advance().with_cursor(offset);
			if c == Kind::Eof || c != self.skip {
				break;
			}
			self.trivia.push(c)
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
