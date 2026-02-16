use bumpalo::Bump;
use console::Style;
use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::Lexer;
use css_parse::{CursorCompactWriteSink, CursorOverlaySink, Parser, ToCursors};
use csskit_transform::{CssMinifierFeature, Transformer};
use glob::glob;
use similar::{ChangeTag, TextDiff};
use std::{fs::read_to_string, panic::catch_unwind, path::PathBuf};

const FIXTURES_GLOB: &str = "../../coverage/css-minify-tests/tests/**/source.css";

struct CssMinifyTestCase {
	name: String,
	source_text: String,
	expected: String,
}

impl CssMinifyTestCase {
	fn new(source_path: PathBuf) -> Self {
		let expected_path = source_path.as_path().parent().unwrap().join("expected.css").to_path_buf();
		let path = source_path.parent().unwrap();
		let name = format!(
			"{}/{}",
			&path.parent().unwrap().file_name().unwrap().to_str().unwrap(),
			&path.file_name().unwrap().to_str().unwrap()
		);
		let source_text = read_to_string(&source_path).unwrap();
		let expected = read_to_string(expected_path).unwrap().trim_end().to_string();
		Self { name, source_text, expected }
	}
}

fn get_tests() -> Vec<CssMinifyTestCase> {
	let mut files = vec![];
	for path in glob(FIXTURES_GLOB).unwrap().flatten() {
		files.push(CssMinifyTestCase::new(path));
	}
	files.sort_by(|a, b| a.name.cmp(&b.name));
	files
}

fn minify(source_text: &str) -> String {
	let bump = Bump::default();
	let mut transformer = Transformer::new_in(&bump, CssMinifierFeature::all_bits(), &CssAtomSet::ATOMS, source_text);
	let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
	let mut parser = Parser::new(&bump, source_text, lexer);
	let mut result = parser.parse_entirely::<StyleSheet>();
	let mut output = String::new();
	if let Some(ref mut node) = result.output {
		transformer.transform(node);
		let overlays = transformer.overlays();
		let mut overlay_stream =
			CursorOverlaySink::new(source_text, &*overlays, CursorCompactWriteSink::new(source_text, &mut output));
		result.output.to_cursors(&mut overlay_stream);
	} else {
		panic!("Could not parse source");
	}
	output
}

enum TestResult {
	Pass,
	Fail,
	Panic,
}

fn test_case(case: &CssMinifyTestCase) -> TestResult {
	let source = case.source_text.clone();
	match catch_unwind(move || minify(&source)) {
		Err(e) => {
			let msg = if let Some(s) = e.downcast_ref::<&str>() {
				s.to_string()
			} else if let Some(s) = e.downcast_ref::<String>() {
				s.clone()
			} else {
				"unknown panic".to_string()
			};
			println!("{} {} ({})", Style::new().yellow().apply_to("PANIC"), case.name, msg);
			TestResult::Panic
		}
		Ok(actual) => {
			if actual == case.expected {
				return TestResult::Pass;
			}
			println!("{} {}", Style::new().red().apply_to("FAILED"), case.name);
			println!("{}", Style::new().red().apply_to("- actual"));
			println!("{}", Style::new().green().apply_to("+ expected"));
			let diff = TextDiff::from_lines(&*actual, &*case.expected);
			for change in diff.iter_all_changes() {
				let (sign, style) = match change.tag() {
					ChangeTag::Delete => ("-", Style::new().red()),
					ChangeTag::Insert => ("+", Style::new().green()),
					ChangeTag::Equal => (" ", Style::new()),
				};
				print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
			}
			TestResult::Fail
		}
	}
}

#[test]
fn full_suite() {
	let cases = get_tests();
	assert!(!cases.is_empty(), "No test cases found; is the css-minify-tests submodule checked out?");
	let mut fails = 0;
	let mut passes = 0;
	let mut panics = 0;
	for case in &cases {
		match test_case(case) {
			TestResult::Pass => passes += 1,
			TestResult::Fail => fails += 1,
			TestResult::Panic => panics += 1,
		}
	}

	// XXX: If these fail because the numbers go down, great! If they go up, investigate why.
	println!("\ncss-minify-tests: {} passed, {} failed, {} panicked, {} total", passes, fails, panics, cases.len());
	assert_eq!(fails + panics, 257, "Should have zero failures but {fails} tests failed and {panics} panicked");
}
