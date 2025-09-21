use crate::syntax::{SURROGATE_RANGE, is_whitespace};
use std::{char::REPLACEMENT_CHARACTER, str::Chars};

pub trait ParseEscape {
	fn parse_escape_sequence(&mut self) -> (char, u8);
}

impl<'a> ParseEscape for Chars<'a> {
	fn parse_escape_sequence(&mut self) -> (char, u8) {
		if let Some(c) = self.next() {
			if !c.is_ascii_hexdigit() {
				return (c, c.len_utf8() as u8);
			}
			let mut value = 0;
			let mut i = 1; // We already consumed one hex digit
			let current_char = c;

			// Process the first hex digit we already consumed
			value = (value << 4) | current_char.to_digit(16).unwrap();

			// Continue consuming up to 5 more hex digits (total of 6)
			while i < 6 {
				if let Some(next_char) = self.as_str().chars().next() {
					if let Some(hex_value) = next_char.to_digit(16) {
						self.next(); // Only consume if it's a hex digit
						value = (value << 4) | hex_value;
						i += 1;
					} else {
						break;
					}
				} else {
					break;
				}
			}

			// Check if the next character is whitespace and consume it if so
			if let Some(next_char) = self.as_str().chars().next()
				&& is_whitespace(next_char)
			{
				self.next();
				i += 1;
				// https://drafts.csswg.org/css-syntax/#input-preprocessing
				// Replace any U+000D CARRIAGE RETURN (CR) code points, U+000C FORM FEED (FF) code points,
				// or pairs of U+000D CARRIAGE RETURN (CR) followed by U+000A LINE FEED (LF) in input by
				// single U+000A LINE FEED (LF) code point.
				if next_char == '\r' && self.as_str().starts_with('\n') {
					self.next();
					i += 1;
				}
			}

			if value == 0 || SURROGATE_RANGE.contains(&value) {
				return (REPLACEMENT_CHARACTER, i);
			}
			(char::from_u32(value).unwrap_or(REPLACEMENT_CHARACTER), i)
		} else {
			(REPLACEMENT_CHARACTER, 0)
		}
	}
}
