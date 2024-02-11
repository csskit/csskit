mod constants;
mod private;
mod string_builder;
mod token;

use std::{collections::VecDeque, str::Chars};

use oxc_allocator::Allocator;
pub use token::{NumType, PairWise, Token};

#[derive(Debug, Clone)]
pub struct LexerCheckpoint<'a> {
	chars: Chars<'a>,
	token: Token,
	prev_pos: u32,
}

pub struct Lexer<'a> {
	allocator: &'a Allocator,
	source: &'a str,
	current: LexerCheckpoint<'a>,
	lookahead: VecDeque<LexerCheckpoint<'a>>,
}

impl<'a> Lexer<'a> {
	pub fn new(allocator: &'a Allocator, source: &'a str) -> Self {
		let token = Token::default();
		let current = LexerCheckpoint { chars: source.chars(), token, prev_pos: 0 };
		Self { allocator, source, current, lookahead: VecDeque::with_capacity(4) }
	}

	/// Remaining string from `Chars`
	fn remaining(&self) -> &'a str {
		self.current.chars.as_str()
	}

	/// Current position in file
	#[inline]
	pub fn pos(&self) -> u32 {
		(self.source.len() - self.remaining().len()) as u32
	}

	/// Creates a checkpoint storing the current lexer state.
	/// Use `rewind` to restore the lexer to the state stored in the checkpoint.
	pub fn checkpoint(&self) -> LexerCheckpoint<'a> {
		LexerCheckpoint {
			prev_pos: self.current.prev_pos,
			chars: self.current.chars.clone(),
			token: self.current.token.clone(),
		}
	}

	/// Rewinds the lexer to the same state as when the passed in `checkpoint` was created.
	pub fn rewind(&mut self, checkpoint: LexerCheckpoint<'a>) {
		self.current = checkpoint;
		self.lookahead.clear();
	}

	/// Find the nth lookahead token lazily
	pub fn lookahead(&mut self, n: u8) -> &Token {
		let n = n as usize;
		debug_assert!(n > 0);

		if self.lookahead.len() > n - 1 {
			return &self.lookahead[n - 1].token;
		}

		let checkpoint = self.checkpoint();

		if let Some(checkpoint) = self.lookahead.back() {
			self.current = checkpoint.clone();
		}

		// reset the current token for `read_next_token`,
		// otherwise it will contain the token from
		// `self.current = checkpoint`
		self.current.token = Token::default();

		let prev_pos = self.pos();

		for _i in self.lookahead.len()..n {
			let peeked = self.read_next_token();
			self.lookahead.push_back(LexerCheckpoint {
				prev_pos,
				chars: self.current.chars.clone(),
				token: peeked,
			});
		}

		self.current = checkpoint;

		&self.lookahead[n - 1].token
	}

	pub fn jump_token(&mut self) -> Token {
		if let Some(checkpoint) = self.lookahead.pop_back() {
			self.current.chars = checkpoint.chars;
			self.lookahead.clear();
			return checkpoint.token;
		}
		self.next_token()
	}

	pub fn next_token(&mut self) -> Token {
		if let Some(checkpoint) = self.lookahead.pop_front() {
			self.current.chars = checkpoint.chars;
			return checkpoint.token;
		}
		self.read_next_token()
	}
}
