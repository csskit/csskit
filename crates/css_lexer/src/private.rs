use crate::{
	CommentStyle, DynAtomSet, Feature, Lexer, QuoteStyle, Token, Whitespace,
	constants::SINGLE_CHAR_TOKENS,
	small_str_buf::SmallStrBuf,
	syntax::{
		CR, EOF, FF, LF, ParseEscape, SPACE, TAB,
		identifier::{
			is_ident, is_ident_ascii_lower_or_digit, is_ident_ascii_start, is_ident_byte, is_ident_start,
			is_ident_start_sequence,
		},
		is_escape_sequence, is_newline, is_newline_or_eof, is_non_printable, is_quote, is_url_ident, is_whitespace,
	},
};
use std::{char::REPLACEMENT_CHARACTER, str::Chars};

// 7 makes size_of::<SmallStrBuf<8>>() == size_of::<usize>()
const MAX_SMALL_IDENT_SIZE: usize = 7;

trait CharsConsumer {
	fn is_last(&self) -> bool;

	fn peek_nth(&self, n: usize) -> char;

	fn peek2(&self) -> (char, char);

	fn peek3(&self) -> (char, char, char);

	#[must_use]
	fn consume_newline(&mut self) -> u32;

	#[must_use]
	fn consume_same(&mut self, char: char) -> u32;

	#[must_use]
	fn consume_whitespace(&mut self) -> (u32, Whitespace);

	#[must_use]
	fn consume_ident_sequence(&mut self, atoms: &dyn DynAtomSet) -> (u32, bool, bool, bool, u32, bool);

	#[must_use]
	fn consume_escape_sequence(&mut self) -> u32;

	#[must_use]
	fn consume_url_sequence(&mut self, len: u32, ident_escaped: bool) -> Token;

	#[must_use]
	fn consume_remnants_of_bad_url(&mut self, len: u32) -> Token;

	#[must_use]
	fn consume_numeric_token(&mut self, atoms: &dyn DynAtomSet) -> Token;

	#[must_use]
	fn consume_hash_token(&mut self, atoms: &dyn DynAtomSet) -> Token;

	#[must_use]
	fn consume_ident_like_token(&mut self, atoms: &dyn DynAtomSet) -> Token;

	#[must_use]
	fn consume_string_token(&mut self) -> Token;

	#[must_use]
	fn is_number_start(&mut self) -> bool;
}

impl<'a> CharsConsumer for Chars<'a> {
	#[inline]
	fn is_last(&self) -> bool {
		self.clone().next().is_none()
	}

	#[inline]
	fn peek_nth(&self, n: usize) -> char {
		self.clone().nth(n).unwrap_or(EOF)
	}

	#[inline]
	fn peek2(&self) -> (char, char) {
		let mut chars = self.clone();
		let c0 = chars.next().unwrap_or(EOF);
		let c1 = chars.next().unwrap_or(EOF);
		(c0, c1)
	}

	#[inline]
	fn peek3(&self) -> (char, char, char) {
		let mut chars = self.clone();
		let c0 = chars.next().unwrap_or(EOF);
		let c1 = chars.next().unwrap_or(EOF);
		let c2 = chars.next().unwrap_or(EOF);
		(c0, c1, c2)
	}

	fn consume_newline(&mut self) -> u32 {
		if let Some(c) = self.next()
			&& c == CR
			&& self.peek_nth(0) == LF
		{
			self.next();
			return 2;
		}
		1
	}

	fn consume_same(&mut self, char: char) -> u32 {
		let mut i = 0;
		while self.peek_nth(0) == char {
			self.next();
			i += 1;
		}
		i
	}

	fn consume_whitespace(&mut self) -> (u32, Whitespace) {
		let mut i = 0;
		let mut style = Whitespace::none();
		while is_whitespace(self.peek_nth(0)) {
			let c = self.next().unwrap();
			if c == ' ' {
				style |= Whitespace::Space;
			} else if c == '\t' {
				style |= Whitespace::Tab;
			} else {
				style |= Whitespace::Newline;
			}
			i += 1;
		}
		(i, style)
	}

	fn consume_ident_sequence(&mut self, atoms: &dyn DynAtomSet) -> (u32, bool, bool, bool, u32, bool) {
		let mut dashed_ident = false;
		let mut contains_non_lower_ascii = false;
		let mut contains_escape = false;

		let str = self.as_str();
		if !str.is_empty() {
			let bytes = str.as_bytes();
			const MAX_FAST_SCAN_IDENT_SIZE: u32 = 50;
			let end = MAX_FAST_SCAN_IDENT_SIZE.min(bytes.len() as u32) as usize;
			let mut i = 0;
			if bytes.len() >= 2 && bytes[0] == b'-' && bytes[1] == b'-' {
				i = 2;
				dashed_ident = true;
			}
			let scan_start = i;
			while i < end {
				let byte = bytes[i];
				if !is_ident_byte(byte) {
					break;
				}
				contains_non_lower_ascii |= byte.wrapping_sub(b'A') < 26;
				i += 1;
			}

			let ascii_len = (i - scan_start) as u32;
			let len = i as u32;
			if ascii_len > 0 {
				let next_byte = if i < bytes.len() { bytes[i] } else { b' ' };
				if next_byte < 128 && !is_ident_byte(next_byte) && next_byte != b'\\' && next_byte != 0 {
					// Tempting to use `nth(len)` here but this is faster
					for _ in 0..len {
						self.next();
					}
					let atom_bits = if dashed_ident {
						atoms.str_to_bits(&str[2..len as usize])
					} else {
						atoms.str_to_bits(&str[0..len as usize])
					};
					return (
						len,
						contains_non_lower_ascii,
						dashed_ident,
						false,
						atom_bits,
						ascii_len == 3 && is_url_ident(&str[0..len as usize]),
					);
				}
			}
		}

		let mut len = 0;
		let mut small_ident = SmallStrBuf::<MAX_SMALL_IDENT_SIZE>::new();

		let mut chars_peek = self.clone();
		while let Some(c) = chars_peek.next() {
			// Most common case first: lowercase ASCII, digits, or dash (except leading dash)
			if is_ident_ascii_lower_or_digit(c) || (c == '-' && len > 0) {
				self.next();
				len += 1;
				small_ident.append(c);
			} else if len == 0 && c == '-' {
				// Leading dash case - check for dashed ident
				let next = chars_peek.clone().next().unwrap_or(EOF);
				self.next();
				len += 1;
				if next == '-' {
					self.next();
					chars_peek.next();
					len += 1;
					dashed_ident = true;
					// Reset the small_ident buffer as dashed_idents always begin with two dashes.
					small_ident = SmallStrBuf::<MAX_SMALL_IDENT_SIZE>::new();
					continue;
				}
				small_ident.append(c);
			} else if is_ident(c) {
				self.next();
				len += c.len_utf8() as u32;
				contains_non_lower_ascii = true;
				small_ident.append(c);
			} else {
				let next = chars_peek.clone().next().unwrap_or(EOF);
				if is_escape_sequence(c, next) {
					self.next();
					if small_ident.over_capacity() {
						contains_escape = true;
						len += self.consume_escape_sequence();
						chars_peek = self.clone();
					} else {
						let (char, esc_len) = self.parse_escape_sequence();
						small_ident.append(char);
						len += 1 + esc_len as u32;
						contains_escape = true;
						chars_peek = self.clone();
					}
					continue;
				} else if c == '\0' {
					self.next();
					contains_escape = true;
					len += 1;
				} else {
					break;
				}
			}
		}
		// The ident was small enough to be fully encoded into the small_ident buffer,
		// so it should be used over the str slice as it will have parsed escape sequences
		let (is_url, atom_bits) = if let Some(ident) = small_ident.as_str() {
			(is_url_ident(ident), atoms.str_to_bits(ident))
		} else if dashed_ident {
			// For dashed identifiers, skip the leading "--" for atom lookup
			let slice = &str[2..len as usize];
			(false, atoms.str_to_bits(slice))
		} else {
			// We intentionally make the small_ident buffer large enough to capture the `url` ident an unescape it,
			// so this branch would never be hit with a valid URL, we can guarantee this ident is not a URL.
			(false, atoms.str_to_bits(&str[0..len as usize]))
		};
		(len, contains_non_lower_ascii, dashed_ident, contains_escape, atom_bits, !dashed_ident && is_url)
	}

	fn consume_escape_sequence(&mut self) -> u32 {
		let mut len = 1;
		if let Some(c) = self.next() {
			len += c.len_utf8() as u32;
			if c.is_ascii_hexdigit() {
				let mut i = 1; // We already consumed one hex digit (c)
				let mut chars = self.clone().peekable();
				while chars.peek().unwrap_or(&EOF).is_ascii_hexdigit() {
					chars.next();
					self.next();
					len += 1;
					i += 1;
					if i > 5 {
						break;
					}
				}
				if is_whitespace(*chars.peek().unwrap_or(&EOF)) {
					let c = self.next();
					len += 1;
					// https://drafts.csswg.org/css-syntax/#input-preprocessing
					// Replace any U+000D CARRIAGE RETURN (CR) code points, U+000C FORM FEED (FF) code points,
					// or pairs of U+000D CARRIAGE RETURN (CR) followed by U+000A LINE FEED (LF) in input by
					// single U+000A LINE FEED (LF) code point.
					if c == Some('\r') && self.peek_nth(0) == '\n' {
						self.next();
						len += 1;
					}
				}
			}
		}
		len
	}

	fn consume_url_sequence(&mut self, leading_len: u32, ident_escaped: bool) -> Token {
		let mut len = leading_len;
		let mut trailing_len = 0;
		let mut contains_escape = ident_escaped;
		let mut ends_with_paren = false;
		let (whitespace_count, _) = self.consume_whitespace();
		if whitespace_count > 0 {
			len += whitespace_count;
		}
		loop {
			let c = self.peek_nth(0);
			match c {
				')' => {
					self.next();
					len += 1;
					trailing_len += 1;
					ends_with_paren = true;
					break;
				}
				EOF => {
					break;
				}
				_ if is_whitespace(c) => {
					trailing_len += self.consume_whitespace().0;
					len += trailing_len;
					// Consider trailing whitespace as escape to allow the string
					// parser to consume characters one-by-one
					contains_escape = true;
					match self.peek_nth(0) {
						')' => {
							self.next();
							len += 1;
							trailing_len += 1;
							ends_with_paren = true;
							break;
						}
						EOF => {
							break;
						}
						_ => {
							return self.consume_remnants_of_bad_url(len);
						}
					};
				}
				'\'' | '"' | '(' => {
					return self.consume_remnants_of_bad_url(len);
				}
				_ if is_non_printable(c) => {
					return self.consume_remnants_of_bad_url(len);
				}
				'\\' => {
					let (_, next) = self.peek2();
					if is_escape_sequence(c, next) {
						self.next();
						len += self.consume_escape_sequence();
						contains_escape = true;
					} else {
						return self.consume_remnants_of_bad_url(len);
					}
				}
				c => {
					self.next();
					len += c.len_utf8() as u32;
				}
			}
		}
		Token::new_url(
			ends_with_paren,
			whitespace_count > 0,
			contains_escape,
			leading_len + whitespace_count,
			trailing_len,
			len,
		)
	}

	fn consume_remnants_of_bad_url(&mut self, len: u32) -> Token {
		let mut len = len;
		while let Some(ch) = self.next() {
			match ch {
				')' => {
					len += 1;
					break;
				}
				'\\' => {
					if is_escape_sequence(ch, self.peek_nth(0)) {
						len += self.consume_escape_sequence();
					} else if let Some(ch) = self.next() {
						len += ch.len_utf8() as u32 + 1;
					}
				}
				_ => {
					len += ch.len_utf8() as u32;
				}
			}
		}
		Token::new_bad_url(len)
	}

	fn consume_numeric_token(&mut self, atoms: &dyn DynAtomSet) -> Token {
		let str = self.as_str();
		let bytes = str.as_bytes();

		let mut num_len = 1;
		let first_byte = bytes[0];
		let mut is_float = first_byte == b'.';
		let has_sign = first_byte == b'+' || first_byte == b'-';

		// Scan integer part
		let mut i = 1;
		while i < bytes.len() && bytes[i].is_ascii_digit() {
			num_len += 1;
			i += 1;
		}

		// Check for decimal point + fractional part
		if !is_float && i + 1 < bytes.len() && bytes[i] == b'.' && bytes[i + 1].is_ascii_digit() {
			num_len += 1;
			i += 1;
			while i < bytes.len() && bytes[i].is_ascii_digit() {
				num_len += 1;
				i += 1;
			}
			is_float = true;
		}

		// Check for exponent part
		if i < bytes.len() && matches!(bytes[i], b'e' | b'E') {
			let next_byte = if i + 1 < bytes.len() { bytes[i + 1] } else { 0 };
			let next_next_byte = if i + 2 < bytes.len() { bytes[i + 2] } else { 0 };

			if next_byte.is_ascii_digit() || (matches!(next_byte, b'-' | b'+') && next_next_byte.is_ascii_digit()) {
				num_len += 1;
				i += 1;
				if matches!(bytes[i], b'-' | b'+') {
					num_len += 1;
					i += 1;
				}
				while i < bytes.len() && bytes[i].is_ascii_digit() {
					num_len += 1;
					i += 1;
				}
				is_float = true;
			}
		}

		let value = str[0..num_len].parse::<f32>().unwrap();
		self.nth(num_len - 1);
		match self.peek_nth(0) {
			'%' => {
				self.next();
				Token::new_dimension(is_float, has_sign, num_len as u32, 1, value, atoms.str_to_bits("%") as u8)
			}
			c => {
				let (_, c2, c3) = self.peek3();
				if is_ident_start_sequence(c, c2, c3) {
					let (unit_len, _, _, _, atom_bits, _) = self.consume_ident_sequence(atoms);
					Token::new_dimension(is_float, has_sign, num_len as u32, unit_len, value, atom_bits as u8)
				} else {
					Token::new_number(is_float, has_sign, num_len as u32, value)
				}
			}
		}
	}

	fn consume_hash_token(&mut self, atoms: &dyn DynAtomSet) -> Token {
		self.next();
		let hex_reader = self.clone();
		let first_is_ascii = is_ident(self.peek_nth(0));
		let (len, contains_non_lower_ascii, _, contains_escape, _, _) = self.consume_ident_sequence(atoms);
		let mut hex_value = 0;
		let mut is_hex = false;
		if len == 3 || len == 4 {
			is_hex = true;
			for c in hex_reader.take(len as usize) {
				if let Some(d) = c.to_digit(16) {
					hex_value = (hex_value << 8) | (d << 4) | d;
				} else {
					is_hex = false;
					break;
				}
			}
		} else if len == 6 || len == 8 {
			is_hex = true;
			for c in hex_reader.take(len as usize) {
				if let Some(d) = c.to_digit(16) {
					hex_value = (hex_value << 4) | d;
				} else {
					is_hex = false;
					break;
				}
			}
		}
		if is_hex && (len == 3 || len == 6) {
			hex_value = (hex_value << 8) | 0xFF;
		}
		if !is_hex {
			hex_value = 0;
		}
		Token::new_hash(contains_non_lower_ascii, first_is_ascii, contains_escape, len + 1, hex_value)
	}

	fn consume_ident_like_token(&mut self, atoms: &dyn DynAtomSet) -> Token {
		let (mut len, contains_non_lower_ascii, dashed, contains_escape, atom_bits, is_url) =
			self.consume_ident_sequence(atoms);
		if self.peek_nth(0) == '(' {
			self.next();
			len += 1;
			let token = Token::new_function(contains_non_lower_ascii, dashed, contains_escape, atom_bits, len);
			if is_url {
				let mut chars = self.clone();
				let mut char = chars.next().unwrap_or(EOF);
				for _i in 0..=3 {
					if is_whitespace(char) {
						char = chars.next().unwrap_or(EOF);
					}
				}
				if !is_quote(char) {
					return self.consume_url_sequence(len, contains_escape);
				}
			}
			return token;
		}
		Token::new_ident(contains_non_lower_ascii, dashed, contains_escape, atom_bits, len)
	}

	fn consume_string_token(&mut self) -> Token {
		let delimiter = self.next().unwrap();
		let quotes = if delimiter == '"' { QuoteStyle::Double } else { QuoteStyle::Single };
		let mut contains_escape = false;
		let mut len = 1;
		loop {
			match self.peek_nth(0) {
				c if is_newline(c) => {
					return Token::new_bad_string(len);
				}
				EOF => {
					if self.next().is_some() {
						contains_escape = true;
						len += 1;
					} else {
						return Token::new_string(quotes, false, contains_escape, len);
					}
				}
				c @ ('"' | '\'') => {
					self.next();
					len += 1;
					if c == delimiter {
						return Token::new_string(quotes, true, contains_escape, len);
					}
				}
				c @ '\\' => {
					self.next();
					contains_escape = true;
					match self.peek_nth(0) {
						EOF => {
							len += 1;
							return Token::new_string(quotes, false, contains_escape, len);
						}
						p if is_newline(p) => {
							len += self.consume_newline() + 1;
						}
						p if is_escape_sequence(c, p) => {
							len += self.consume_escape_sequence();
						}
						_ => return Token::new_bad_string(len),
					}
				}
				c => {
					self.next();
					len += c.len_utf8() as u32;
				}
			}
		}
	}

	fn is_number_start(&mut self) -> bool {
		let str = self.as_str();
		if str.is_empty() {
			return false;
		}
		let bytes = str.as_bytes();
		let first = bytes[0];
		if first.is_ascii_digit() {
			return true;
		}
		if (first == b'+' || first == b'-') && bytes.len() >= 2 {
			let second = bytes[1];
			return second.is_ascii_digit() || (second == b'.' && bytes.len() >= 3 && bytes[2].is_ascii_digit());
		}
		first == b'.' && bytes.len() >= 2 && bytes[1].is_ascii_digit()
	}
}

impl<'a> Lexer<'a> {
	#[must_use]
	pub(crate) fn read_next_token(&mut self, offset: u32) -> Token {
		if self.source.len() as u32 == offset {
			return Token::EOF;
		}
		let mut chars = self.source[offset as usize..].chars();
		let c = chars.peek_nth(0);
		// fast path for single character tokens
		// '{'  '}'  '('  ')'  '['  ']'  ';' ',' ':'
		let size = c as usize;
		if size < 128 {
			let token = SINGLE_CHAR_TOKENS[size];
			if token != Token::EOF {
				return token;
			}
			// fast path for identifiers
			if is_ident_ascii_start(c) {
				return chars.consume_ident_like_token(self.atoms);
			}
		}
		match c {
			'\0' => {
				// https://drafts.csswg.org/css-syntax-3/#input-preprocessing
				// The input stream consists of the filtered code points pushed into it as the input byte stream is decoded.
				// To filter code points from a stream of (unfiltered) code points input:
				//  Replace any U+0000 NULL or surrogate code points in input with U+FFFD REPLACEMENT CHARACTER (�).
				//
				if !chars.is_last() {
					let (_, c2, c3) = chars.peek3();
					if is_ident_start_sequence(REPLACEMENT_CHARACTER, c2, c3) {
						return chars.consume_ident_like_token(self.atoms);
					}
				}
				if chars.next().is_some() { Token::REPLACEMENT_CHARACTER } else { Token::EOF }
			}
			c if is_whitespace(c) && !self.features.contains(Feature::SeparateWhitespace) => {
				let (len, style) = chars.consume_whitespace();
				Token::new_whitespace(style, len)
			}
			// Whitespace Range
			TAB => Token::new_whitespace(Whitespace::Tab, chars.consume_same(TAB)),
			SPACE => Token::new_whitespace(Whitespace::Space, chars.consume_same(SPACE)),
			LF | CR | FF => {
				// https://drafts.csswg.org/css-syntax/#input-preprocessing
				//  Replace any U+000D CARRIAGE RETURN (CR) code points, U+000C FORM FEED
				//  (FF) code points, or pairs of U+000D CARRIAGE RETURN (CR) followed by
				//  U+000A LINE FEED (LF) in input by a single U+000A LINE FEED (LF) code
				//  point.
				let mut len = 0;
				loop {
					let c = chars.peek_nth(0);
					if !matches!(c, LF | CR | FF) {
						break;
					}
					chars.next();
					len += 1;
				}
				Token::new_whitespace(Whitespace::Newline, len)
			}
			// Quote Range
			c if is_quote(c) => chars.consume_string_token(),
			// Digit Range
			c if c.is_ascii_digit() => chars.consume_numeric_token(self.atoms),
			// Sign Range
			'-' => {
				if chars.peek_nth(1) == '-' && chars.peek_nth(2) == '>' {
					chars.next();
					chars.next();
					chars.next();
					return Token::CDC;
				}
				if is_ident_start_sequence(c, chars.peek_nth(1), chars.peek_nth(2)) {
					return chars.consume_ident_like_token(self.atoms);
				}
				if chars.is_number_start() {
					return chars.consume_numeric_token(self.atoms);
				}
				chars.next();
				Token::DASH
			}
			// Dot or Plus
			'.' | '+' => {
				if chars.is_number_start() {
					return chars.consume_numeric_token(self.atoms);
				}
				chars.next();
				Token::new_delim(c)
			}
			// Less Than
			'<' => {
				chars.next();
				if chars.peek_nth(0) == '!' && chars.peek_nth(1) == '-' && chars.peek_nth(2) == '-' {
					chars.next();
					chars.next();
					chars.next();
					return Token::CDO;
				}
				Token::LESS_THAN
			}
			// Hash / Pound Sign
			'#' => {
				if is_ident(chars.peek_nth(1)) || is_escape_sequence(chars.peek_nth(1), chars.peek_nth(2)) {
					chars.consume_hash_token(self.atoms)
				} else {
					chars.next();
					Token::HASH
				}
			}
			// Commercial At
			'@' => {
				chars.next();
				let (c1, c2, c3) = chars.peek3();
				if is_ident_start_sequence(c1, c2, c3) {
					let (len, contains_non_lower_ascii, dashed, contains_escape, atom_bits, _) =
						chars.consume_ident_sequence(self.atoms);
					return Token::new_atkeyword(contains_non_lower_ascii, dashed, contains_escape, atom_bits, len + 1);
				}
				Token::AT
			}
			// Reverse Solidus
			'\\' => {
				let (_, c2) = chars.peek2();
				if is_escape_sequence(c, c2) {
					return chars.consume_ident_like_token(self.atoms);
				}
				chars.next();
				Token::BACKSLASH
			}
			// Solidus
			'/' => {
				let (_, c2) = chars.peek2();
				match c2 {
					'*' => {
						chars.next();
						chars.next();
						let mut len = 2;
						let (c1, c2) = chars.peek2();
						let comment_style = match c1 {
							'*' if c2 != '/' => CommentStyle::BlockStar,
							'#' => CommentStyle::BlockPound,
							'!' => CommentStyle::BlockBang,
							'-' | '=' => CommentStyle::BlockHeading,
							_ => CommentStyle::Block,
						};
						while let Some(c) = chars.next() {
							len += c.len_utf8() as u32;
							if c == '*' && chars.peek_nth(0) == '/' {
								chars.next();
								len += 1;
								break;
							}
						}
						Token::new_comment(comment_style, len)
					}
					'/' if self.features.intersects(Feature::SingleLineComments) => {
						chars.next();
						chars.next();
						let mut len = 2;
						let comment_style = match chars.peek_nth(0) {
							'*' => CommentStyle::SingleStar,
							'!' => CommentStyle::SingleBang,
							_ => CommentStyle::Single,
						};
						while !is_newline_or_eof(chars.peek_nth(0)) {
							chars.next();
							len += 1;
						}
						Token::new_comment(comment_style, len)
					}
					_ => {
						chars.next();
						Token::SLASH
					}
				}
			}
			c if is_ident_start(c) => chars.consume_ident_like_token(self.atoms),
			c => {
				chars.next();
				Token::new_delim(c)
			}
		}
	}
}
