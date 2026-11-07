use lsp_types::{ClientCapabilities, Position, PositionEncodingKind};
use ropey::{Rope, RopeSlice};

/// The unit that LSP [`Position::character`] offsets count, agreed with the client during `initialize`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum PositionEncoding {
	/// Offsets count UTF-8 bytes.
	Utf8,
	/// Offsets count UTF-16 code units. This is the LSP default for clients that do not advertise an encoding.
	#[default]
	Utf16,
}

impl PositionEncoding {
	/// Picks UTF-8 when the client lists it in `general.positionEncodings`, otherwise UTF-16.
	pub fn negotiate(capabilities: &ClientCapabilities) -> Self {
		let offers_utf8 = capabilities
			.general
			.as_ref()
			.and_then(|general| general.position_encodings.as_ref())
			.is_some_and(|encodings| encodings.contains(&PositionEncodingKind::UTF8));
		if offers_utf8 { Self::Utf8 } else { Self::Utf16 }
	}

	/// The value to report in `ServerCapabilities::position_encoding`.
	pub fn kind(self) -> PositionEncodingKind {
		match self {
			Self::Utf8 => PositionEncodingKind::UTF8,
			Self::Utf16 => PositionEncodingKind::UTF16,
		}
	}

	/// The length of `text` in this encoding's units.
	pub fn len_of(self, text: &str) -> u32 {
		match self {
			Self::Utf8 => text.len() as u32,
			Self::Utf16 => text.encode_utf16().count() as u32,
		}
	}

	/// Converts `position` to a char index into `rope`.
	///
	/// A line past the end of the document resolves to the end of the document. A character past the end of a line,
	/// or inside a multi-unit character, resolves to the end of that line or the start of that character.
	pub fn char_index(self, rope: &Rope, position: Position) -> usize {
		let line_idx = position.line as usize;
		if line_idx >= rope.len_lines() {
			return rope.len_chars();
		}
		let line = without_line_break(rope.line(line_idx));
		let character = position.character as usize;
		let offset = match self {
			Self::Utf8 if character < line.len_bytes() => line.byte_to_char(character),
			Self::Utf16 if character < line.len_utf16_cu() => line.utf16_cu_to_char(character),
			_ => line.len_chars(),
		};
		rope.line_to_char(line_idx) + offset
	}

	/// Converts a char index into `rope` to a [`Position`], clamping indexes past the end to the end of the document.
	pub fn position(self, rope: &Rope, char_idx: usize) -> Position {
		let char_idx = char_idx.min(rope.len_chars());
		let line_idx = rope.char_to_line(char_idx);
		let before = rope.slice(rope.line_to_char(line_idx)..char_idx);
		let character = match self {
			Self::Utf8 => before.len_bytes(),
			Self::Utf16 => before.len_utf16_cu(),
		};
		Position::new(line_idx as u32, character as u32)
	}
}

fn without_line_break(line: RopeSlice<'_>) -> RopeSlice<'_> {
	let mut end = line.len_chars();
	if end > 0 && line.char(end - 1) == '\n' {
		end -= 1;
	}
	if end > 0 && line.char(end - 1) == '\r' {
		end -= 1;
	}
	line.slice(..end)
}

#[cfg(test)]
mod tests {
	use super::*;
	use lsp_types::GeneralClientCapabilities;

	const ASTRAL: &str = "a🎨b\nc😀d\r\ne";

	fn caps(encodings: Option<Vec<PositionEncodingKind>>) -> ClientCapabilities {
		ClientCapabilities {
			general: Some(GeneralClientCapabilities { position_encodings: encodings, ..Default::default() }),
			..Default::default()
		}
	}

	#[test]
	fn negotiates_utf8_only_when_offered() {
		assert_eq!(PositionEncoding::negotiate(&ClientCapabilities::default()), PositionEncoding::Utf16);
		assert_eq!(PositionEncoding::negotiate(&caps(None)), PositionEncoding::Utf16);
		assert_eq!(
			PositionEncoding::negotiate(&caps(Some(vec![PositionEncodingKind::UTF32, PositionEncodingKind::UTF16]))),
			PositionEncoding::Utf16
		);
		assert_eq!(
			PositionEncoding::negotiate(&caps(Some(vec![PositionEncodingKind::UTF16, PositionEncodingKind::UTF8]))),
			PositionEncoding::Utf8
		);
	}

	#[test]
	fn len_of_counts_units() {
		assert_eq!(PositionEncoding::Utf8.len_of("a🎨b"), 6);
		assert_eq!(PositionEncoding::Utf16.len_of("a🎨b"), 4);
		assert_eq!(PositionEncoding::Utf16.len_of("é"), 1);
	}

	#[test]
	fn utf16_char_index_after_astral() {
		let rope = Rope::from_str(ASTRAL);
		let enc = PositionEncoding::Utf16;
		assert_eq!(rope.char(enc.char_index(&rope, Position::new(0, 3))), 'b');
		assert_eq!(rope.char(enc.char_index(&rope, Position::new(1, 3))), 'd');
		assert_eq!(rope.char(enc.char_index(&rope, Position::new(2, 0))), 'e');
	}

	#[test]
	fn utf8_char_index_after_astral() {
		let rope = Rope::from_str(ASTRAL);
		let enc = PositionEncoding::Utf8;
		assert_eq!(rope.char(enc.char_index(&rope, Position::new(0, 5))), 'b');
		assert_eq!(rope.char(enc.char_index(&rope, Position::new(1, 5))), 'd');
	}

	#[test]
	fn char_index_inside_astral_resolves_to_its_start() {
		let rope = Rope::from_str(ASTRAL);
		assert_eq!(PositionEncoding::Utf16.char_index(&rope, Position::new(0, 2)), 1);
		assert_eq!(PositionEncoding::Utf8.char_index(&rope, Position::new(0, 3)), 1);
	}

	#[test]
	fn char_index_clamps_to_line_and_document_end() {
		let rope = Rope::from_str(ASTRAL);
		for enc in [PositionEncoding::Utf8, PositionEncoding::Utf16] {
			assert_eq!(enc.char_index(&rope, Position::new(0, 99)), 3);
			assert_eq!(enc.char_index(&rope, Position::new(1, 99)), 7);
			assert_eq!(enc.char_index(&rope, Position::new(9, 0)), rope.len_chars());
		}
	}

	#[test]
	fn unicode_line_separators_are_not_line_breaks() {
		let rope = Rope::from_str("a\u{2028}b\nc");
		assert_eq!(rope.char(PositionEncoding::Utf16.char_index(&rope, Position::new(1, 0))), 'c');
	}

	#[test]
	fn position_round_trips() {
		let rope = Rope::from_str(ASTRAL);
		for enc in [PositionEncoding::Utf8, PositionEncoding::Utf16] {
			let inside_crlf = |i: usize| i > 0 && rope.get_char(i - 1) == Some('\r') && rope.get_char(i) == Some('\n');
			for char_idx in (0..=rope.len_chars()).filter(|&i| !inside_crlf(i)) {
				let position = enc.position(&rope, char_idx);
				assert_eq!(enc.char_index(&rope, position), char_idx, "{enc:?} {char_idx} {position:?}");
			}
		}
	}

	#[test]
	fn position_counts_encoding_units() {
		let rope = Rope::from_str(ASTRAL);
		assert_eq!(PositionEncoding::Utf16.position(&rope, 2), Position::new(0, 3));
		assert_eq!(PositionEncoding::Utf8.position(&rope, 2), Position::new(0, 5));
		assert_eq!(PositionEncoding::Utf16.position(&rope, 99), Position::new(2, 1));
	}
}
