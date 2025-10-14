pub mod identifier;
pub mod parse_escape;
pub mod tables;

pub use parse_escape::*;

pub const SURROGATE_RANGE: std::ops::RangeInclusive<u32> = 0xd800..=0xdfff;

pub const EOF: char = '\0';

pub const FF: char = '\u{c}';

pub const CR: char = '\u{d}';

pub const LF: char = '\u{a}';

pub const TAB: char = '\u{9}';

pub const SPACE: char = ' ';

use tables::{ASCII_NEWLINE, ASCII_NEWLINE_OR_EOF, ASCII_NON_PRINTABLE, ASCII_WHITESPACE};

#[inline(always)]
pub const fn is_whitespace(c: char) -> bool {
	c.is_ascii() && ASCII_WHITESPACE.0[c as usize]
}

#[inline(always)]
pub const fn is_newline(c: char) -> bool {
	c.is_ascii() && ASCII_NEWLINE.0[c as usize]
}

#[inline(always)]
pub const fn is_newline_or_eof(c: char) -> bool {
	c.is_ascii() && ASCII_NEWLINE_OR_EOF.0[c as usize]
}

#[inline(always)]
pub const fn is_quote(c: char) -> bool {
	c == '\'' || c == '"'
}

#[inline(always)]
pub const fn is_escape_sequence(c: char, c2: char) -> bool {
	c == '\\' && !is_newline(c2)
}

#[inline(always)]
pub const fn is_url_ident(str: &str) -> bool {
	let str = str.as_bytes();
	str.len() == 3 && matches!(str[0], b'u' | b'U') && matches!(str[1], b'r' | b'R') && matches!(str[2], b'l' | b'L')
}

#[inline(always)]
pub fn is_non_printable(c: char) -> bool {
	c.is_ascii() && ASCII_NON_PRINTABLE.0[c as usize]
}

#[cfg(test)]
mod tests {
	use super::{CR, FF, LF, identifier::is_ident_start_sequence};

	#[test]
	fn test_is_ident_start_sequence() {
		assert!(is_ident_start_sequence('-', '-', 'a'));
		assert!(!is_ident_start_sequence('\0', '\0', '\0'));
		assert!(is_ident_start_sequence('\u{FFFD}', '\u{FFFD}', '\u{FFFD}'));
		assert!(!is_ident_start_sequence(CR, LF, FF));
	}
}
