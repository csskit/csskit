use bumpalo::Bump;
use console::Style;
use glob::glob;
use hdx_atom::atom;
use hdx_lexer::{Include, Kind, Lexer, Token};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use similar::{ChangeTag, TextDiff};
use std::{fs::read_to_string, path::PathBuf};

const FIXTURES_GLOB: &str = "../../tasks/coverage/css-tokenizer-tests/tests/**/source.css";

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
enum Structured {
	Dimension(DimensionStructured),
	Number(NumberStructured),
	String(StringStructured),
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NumberStructured {
	value: f32,
	#[serde(rename = "type")]
	kind: Option<String>,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct DimensionStructured {
	value: f32,
	#[serde(rename = "type")]
	kind: String,
	unit: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct StringStructured {
	value: String,
}

#[derive(PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CSSTokenizerTestToken {
	#[serde(rename(deserialize = "type"))]
	kind: CSSTokenizerTestKind,
	raw: String,
	start_index: u32,
	end_index: u32,
	structured: Option<Structured>,
}

#[derive(PartialEq, Serialize, Deserialize)]
enum CSSTokenizerTestKind {
	#[serde(rename(deserialize = "comment"))]
	Comment,
	#[serde(rename(deserialize = "ident-token"))]
	Ident,
	#[serde(rename(deserialize = "function-token"))]
	Function,
	#[serde(rename(deserialize = "at-keyword-token"))]
	AtKeyword,
	#[serde(rename(deserialize = "hash-token"))]
	Hash,
	#[serde(rename(deserialize = "string-token"))]
	String,
	#[serde(rename(deserialize = "bad-string-token"))]
	BadString,
	#[serde(rename(deserialize = "url-token"))]
	Url,
	#[serde(rename(deserialize = "bad-url-token"))]
	BadUrl,
	#[serde(rename(deserialize = "delim-token"))]
	Delim,
	#[serde(rename(deserialize = "number-token"))]
	Number,
	#[serde(rename(deserialize = "percentage-token"))]
	Percentage,
	#[serde(rename(deserialize = "dimension-token"))]
	Dimension,
	#[serde(rename(deserialize = "whitespace-token"))]
	Whitespace,
	#[serde(rename(deserialize = "CDO-token"))]
	Cdo,
	#[serde(rename(deserialize = "CDC-token"))]
	Cdc,
	#[serde(rename(deserialize = "colon-token"))]
	Colon,
	#[serde(rename(deserialize = "semicolon-token"))]
	Semicolon,
	#[serde(rename(deserialize = "comma-token"))]
	Comma,
	#[serde(rename(deserialize = "[-token"))]
	LeftSquare,
	#[serde(rename(deserialize = "]-token"))]
	RightSquare,
	#[serde(rename(deserialize = "(-token"))]
	LeftParen,
	#[serde(rename(deserialize = ")-token"))]
	RightParen,
	#[serde(rename(deserialize = "{-token"))]
	LeftCurly,
	#[serde(rename(deserialize = "}-token"))]
	RightCurly,
}

pub struct CSSTokenizerTestSuite;

fn get_tests() -> Vec<CSSTokenizerTestCase> {
	let mut files = vec![];
	for path in glob(FIXTURES_GLOB).unwrap().flatten() {
		files.push(CSSTokenizerTestCase::new(path));
	}
	files
}

pub struct CSSTokenizerTestCase {
	name: String,
	source_text: String,
	desired: Vec<CSSTokenizerTestToken>,
}

impl CSSTokenizerTestCase {
	fn new(source_path: PathBuf) -> Self {
		let json_path = source_path.as_path().parent().unwrap().join("tokens.json").to_path_buf();
		let path = source_path.parent().unwrap();
		let name = format!(
			"{}-{}",
			&path.parent().unwrap().file_name().unwrap().to_str().unwrap(),
			&path.file_name().unwrap().to_str().unwrap()
		);
		let source_text = read_to_string(&source_path).unwrap();
		let desired: Vec<CSSTokenizerTestToken> = from_str(read_to_string(json_path.clone()).unwrap().as_str())
			.unwrap_or_else(|e| panic!("{} {}", json_path.display(), e));
		Self { name, source_text, desired }
	}
}

fn convert_token(lexer: &Lexer, token: Token) -> CSSTokenizerTestToken {
	let raw = lexer.parse_raw_str(token);
	let structured = match token.kind() {
		Kind::Number => Some(Structured::Number(NumberStructured {
			value: lexer.parse_number(token),
			kind: Some(String::from(if token.is_int() { "integer" } else { "number" })),
		})),
		Kind::Dimension => {
			if lexer.parse_atom(token) == atom!("%") {
				Some(Structured::Number(NumberStructured { value: lexer.parse_number(token), kind: None }))
			} else {
				Some(Structured::Dimension(DimensionStructured {
					value: lexer.parse_number(token),
					unit: lexer.parse_str(token).to_owned(),
					kind: String::from(if token.is_int() { "integer" } else { "number" }),
				}))
			}
		}
		Kind::Ident | Kind::String | Kind::AtKeyword | Kind::Function | Kind::Url | Kind::Hash => {
			Some(Structured::String(StringStructured { value: lexer.parse_str(token).into() }))
		}
		Kind::Delim => Some(Structured::String(StringStructured { value: token.char().unwrap().to_string() })),
		_ => None,
	};
	// token.start/end count utf8 bytes because rust strings are utf8. The tokenizer tests,
	// however, are JS based and JS strings are utf16. So recalculate the indexes to utf16
	let fake_token = Token::new(Kind::Whitespace, 0, 0, token.offset);
	let start_index = lexer.parse_raw_str(fake_token).encode_utf16().count() as u32;
	let end_index = start_index + raw.encode_utf16().count() as u32;
	CSSTokenizerTestToken {
		kind: match token.kind() {
			Kind::Comment => CSSTokenizerTestKind::Comment,
			Kind::Ident => CSSTokenizerTestKind::Ident,
			Kind::Function => CSSTokenizerTestKind::Function,
			Kind::AtKeyword => CSSTokenizerTestKind::AtKeyword,
			Kind::Hash => CSSTokenizerTestKind::Hash,
			Kind::String => CSSTokenizerTestKind::String,
			Kind::BadString => CSSTokenizerTestKind::BadString,
			Kind::Url => CSSTokenizerTestKind::Url,
			Kind::BadUrl => CSSTokenizerTestKind::BadUrl,
			Kind::Delim => CSSTokenizerTestKind::Delim,
			Kind::Number => CSSTokenizerTestKind::Number,
			Kind::Dimension => {
				if lexer.parse_atom(token) == atom!("%") {
					CSSTokenizerTestKind::Percentage
				} else {
					CSSTokenizerTestKind::Dimension
				}
			}
			Kind::Whitespace => CSSTokenizerTestKind::Whitespace,
			Kind::CdcOrCdo => {
				if token.is_cdc() {
					CSSTokenizerTestKind::Cdc
				} else {
					CSSTokenizerTestKind::Cdo
				}
			}
			Kind::Colon => CSSTokenizerTestKind::Colon,
			Kind::Semicolon => CSSTokenizerTestKind::Semicolon,
			Kind::Comma => CSSTokenizerTestKind::Comma,
			Kind::LeftSquare => CSSTokenizerTestKind::LeftSquare,
			Kind::RightSquare => CSSTokenizerTestKind::RightSquare,
			Kind::LeftParen => CSSTokenizerTestKind::LeftParen,
			Kind::RightParen => CSSTokenizerTestKind::RightParen,
			Kind::LeftCurly => CSSTokenizerTestKind::LeftCurly,
			Kind::RightCurly => CSSTokenizerTestKind::RightCurly,
			_ => unreachable!(),
		},
		raw: raw.to_owned(),
		start_index,
		end_index,
		structured,
	}
}

fn test_case(case: CSSTokenizerTestCase) -> u8 {
	let allocator = Bump::default();
	let mut lexer = Lexer::new(&allocator, &case.source_text, Include::all());
	let mut tokens = vec![];
	loop {
		let token = lexer.advance();
		if token.kind() == Kind::Eof {
			break;
		}
		tokens.push(convert_token(&lexer, token));
	}
	if tokens != *case.desired {
		let left: String = to_string_pretty(&tokens).unwrap_or("".to_string());
		let right: String = to_string_pretty(&case.desired).unwrap_or("".to_string());
		println!("{} {}", Style::new().red().apply_to("✘ FAILED"), case.name);
		println!("{}", Style::new().red().apply_to("- actual"));
		println!("{}", Style::new().green().apply_to("+ expected"));
		let diff = TextDiff::from_lines(&*left, &*right);
		for change in diff.iter_all_changes() {
			let (sign, style) = match change.tag() {
				ChangeTag::Delete => ("-", Style::new().red()),
				ChangeTag::Insert => ("+", Style::new().green()),
				ChangeTag::Equal => (" ", Style::new()),
			};
			print!("{}{}", style.apply_to(sign).bold(), style.apply_to(change));
		}
		return 1;
	}
	0
}

#[test]
fn full_suite() {
	let cases = get_tests();
	let mut fails = 0;
	for case in cases {
		fails += test_case(case);
	}
	assert_eq!(fails, 0);
}
