use crate::syntax::{SURROGATE_RANGE, is_whitespace};
use std::{char::REPLACEMENT_CHARACTER, str::Chars};

pub trait ParseEscape {
	/// Convert raw codepoint to char, handling 0 and surrogates -> REPLACEMENT_CHARACTER
	fn codepoint_to_char(value: u32) -> char {
		if value == 0 || SURROGATE_RANGE.contains(&value) {
			REPLACEMENT_CHARACTER
		} else {
			char::from_u32(value).unwrap_or(REPLACEMENT_CHARACTER)
		}
	}

	/// Consume up to 6 hex digits, return (codepoint_value, bytes_consumed).
	/// The first hex digit must already be consumed and passed as `first_digit`.
	fn consume_hex_escape(&mut self, first_digit: char) -> (u32, u8);

	/// Consume optional trailing whitespace after hex escape, return bytes consumed.
	fn consume_escape_whitespace(&mut self) -> u8;

	/// Full escape sequence parsing.
	fn parse_escape_sequence(&mut self) -> (char, u8);
}

impl<'a> ParseEscape for Chars<'a> {
	fn consume_hex_escape(&mut self, first_digit: char) -> (u32, u8) {
		let mut value = first_digit.to_digit(16).unwrap();
		let mut i = 1u8; // We already consumed one hex digit

		// Continue consuming up to 5 more hex digits (total of 6)
		while i < 6 {
			if let Some(next_char) = self.as_str().chars().next() {
				if let Some(hex_value) = next_char.to_digit(16) {
					self.next();
					value = (value << 4) | hex_value;
					i += 1;
				} else {
					break;
				}
			} else {
				break;
			}
		}

		(value, i)
	}

	fn consume_escape_whitespace(&mut self) -> u8 {
		if let Some(next_char) = self.as_str().chars().next()
			&& is_whitespace(next_char)
		{
			self.next();
			// https://drafts.csswg.org/css-syntax/#input-preprocessing
			// Replace any U+000D CARRIAGE RETURN (CR) code points, U+000C FORM FEED (FF) code points,
			// or pairs of U+000D CARRIAGE RETURN (CR) followed by U+000A LINE FEED (LF) in input by
			// single U+000A LINE FEED (LF) code point.
			if next_char == '\r' && self.as_str().starts_with('\n') {
				self.next();
				2
			} else {
				1
			}
		} else {
			0
		}
	}

	fn parse_escape_sequence(&mut self) -> (char, u8) {
		if let Some(c) = self.next() {
			if !c.is_ascii_hexdigit() {
				return (c, c.len_utf8() as u8);
			}

			let (value, hex_len) = self.consume_hex_escape(c);
			let ws_len = self.consume_escape_whitespace();

			(Self::codepoint_to_char(value), hex_len + ws_len)
		} else {
			(REPLACEMENT_CHARACTER, 0)
		}
	}
}
