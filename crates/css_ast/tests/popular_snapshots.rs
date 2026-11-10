mod helpers;

use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::Lexer;
use css_parse::{Arena, Parser};
use glob::glob;
use std::fs::read_to_string;

const FIXTURES_GLOB: &str = "../../coverage/popular/*.css";

#[test]
fn popular_snapshots() {
	let mut failures = vec![];

	for source_path in glob(FIXTURES_GLOB).unwrap().flatten() {
		let file_name = source_path.file_stem().unwrap().to_str().unwrap();

		// Skip Tailwind files as AST output is too large
		if file_name.starts_with("tailwind") {
			continue;
		}

		let result = std::panic::catch_unwind(|| {
			let allocator = Arena::default();
			let source_text = read_to_string(&source_path).unwrap();
			let lexer = Lexer::new(&CssAtomSet::ATOMS, &source_text);
			let mut parser = Parser::new(&allocator, &source_text, lexer);
			let result = parser.parse_entirely::<StyleSheet>();
			let Some(output) = &result.output else {
				panic!("\n\nParse {:?} failed. Saw error {:?}", source_path, result.errors[0]);
			};
			#[cfg(feature = "serde")]
			insta::assert_ron_snapshot!(file_name, output);
			if !result.errors.is_empty() {
				insta::assert_debug_snapshot!(
					format!("{file_name}_diagnostics"),
					render_diagnostics!(result.errors, source_text)
				);
			}
		});

		if let Err(e) = result {
			failures.push((file_name.to_string(), e));
		}
	}

	if !failures.is_empty() {
		panic!(
			"\n\nAST snapshot failures in {} files:\n{}",
			failures.len(),
			failures.iter().map(|(name, _)| format!("  - {}", name)).collect::<Vec<_>>().join("\n")
		);
	}
}
