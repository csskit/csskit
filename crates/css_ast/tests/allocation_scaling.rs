use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::Lexer;
use css_parse::{Arena, Parser};

fn assert_linear(wrap: impl Fn(usize) -> String) {
	let used = |n: usize| {
		let source = wrap(n);
		let arena = Arena::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, &source);
		let mut parser = Parser::new(&arena, &source, lexer);
		parser.parse_entirely::<StyleSheet>();
		arena.used_bytes()
	};
	let small = used(200);
	let large = used(400);
	assert!(large < small * 3, "200 items: {small} bytes, 400 items: {large} bytes");
}

#[test]
fn unknown_declarations_allocate_linearly() {
	assert_linear(|n| format!("b{{{}}}", "c:1;".repeat(n)));
}

#[test]
fn bad_rules_in_block_allocate_linearly() {
	assert_linear(|n| format!("b{{{}}}", "c d;".repeat(n)));
}

#[test]
fn bad_rules_at_top_level_allocate_linearly() {
	assert_linear(|n| "c d;".repeat(n));
}

#[test]
fn bad_rules_in_media_allocate_linearly() {
	assert_linear(|n| format!("@media x{{{}}}", "c;".repeat(n)));
}

#[test]
fn block_declarations_in_media_allocate_linearly() {
	assert_linear(|n| format!("@media x{{{}}}", "b:{}".repeat(n)));
}
