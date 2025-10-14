mod helpers;

use css_lexer::{EmptyAtomSet, Kind, Lexer};
use glob::glob;
use std::fs::read_to_string;

const FIXTURES_GLOB: &str = "../../coverage/popular/*.css";

#[cfg(feature = "serde")]
#[test]
fn popular_snapshots() {
	let mut failures = vec![];

	for source_path in glob(FIXTURES_GLOB).unwrap().flatten() {
		let file_name = source_path.file_stem().unwrap().to_str().unwrap();

		let result = std::panic::catch_unwind(|| {
			let source_text = read_to_string(&source_path).unwrap();
			let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, &source_text);
			let mut tokens = vec![];
			loop {
				let cursor = lexer.advance();
				if cursor.kind() == Kind::Eof {
					break;
				}
				if !matches!(cursor.kind(), Kind::Whitespace | Kind::Comment) {
					tokens.push(cursor);
				}
			}
			insta::assert_ron_snapshot!(file_name, tokens);
		});

		if let Err(e) = result {
			failures.push((file_name.to_string(), e));
		}
	}

	if !failures.is_empty() {
		panic!(
			"\n\nLexer snapshot failures in {} files:\n{}",
			failures.len(),
			failures.iter().map(|(name, _)| format!("  - {}", name)).collect::<Vec<_>>().join("\n")
		);
	}
}
