/// Renders each diagnostic as `start..end Code: message` so a snapshot can record what the parser
/// could not accept.
#[macro_export]
macro_rules! render_diagnostics {
	($errors: expr, $source_text: expr) => {
		$errors
			.iter()
			.map(|d| {
				let span = d.span();
				format!("{}..{} {}: {}", span.start().0, span.end().0, d.code(&$source_text), d.message(&$source_text))
			})
			.collect::<Vec<String>>()
	};
}

/// Parses the file at `$source_path` as a `StyleSheet` and snapshots the AST. Parsing never fails;
/// when the parser recovered from bad input, the diagnostics are snapshotted too, so the snapshot
/// shows what csskit does not accept.
#[macro_export]
macro_rules! assert_snap_ast {
	($source_path: literal) => {{
		use css_ast::{CssAtomSet, StyleSheet};
		use css_lexer::Lexer;
		use css_parse::{Arena, Parser};
		use std::fs::read_to_string;

		let allocator = Arena::default();
		let source_text = read_to_string($source_path).unwrap();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, &source_text);
		let mut parser = Parser::new(&allocator, &source_text, lexer);
		let result = parser.parse_entirely::<StyleSheet>();
		let Some(output) = &result.output else {
			panic!("\n\nParse {:?} failed. Saw error {:?}", $source_path, result.errors[0]);
		};
		#[cfg(feature = "serde")]
		insta::assert_ron_snapshot!(output);
		if !result.errors.is_empty() {
			insta::assert_debug_snapshot!($crate::render_diagnostics!(result.errors, source_text));
		}
	}};
}
