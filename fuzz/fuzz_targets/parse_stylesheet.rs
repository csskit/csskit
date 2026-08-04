#![no_main]
use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::Lexer;
use css_parse::{Arena, CursorOrderedSink, CursorWriteSink, Parser, String, ToCursors};
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
	if data.len() > 4096 {
		return;
	}
	let Ok(source) = std::str::from_utf8(data) else { return };

	let alloc = Arena::default();
	let lexer = Lexer::new(&CssAtomSet::ATOMS, source);
	let mut parser = Parser::new(&alloc, source, lexer);
	let result = parser.parse_entirely::<StyleSheet>().with_trivia();
	if let Some(node) = result.output {
		let mut out = String::new_in(&alloc);
		let mut write_sink = CursorWriteSink::new(source, &mut out);
		let mut ordered_sink = CursorOrderedSink::new(&alloc, &mut write_sink);
		node.to_cursors(&mut ordered_sink);
	}
});
