#![deny(warnings)]
use bumpalo::Bump;
use core::fmt::Write;
use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::{Kind, Lexer};
use css_parse::{CursorCompactWriteSink, Diagnostic, DiagnosticMeta, Parser, ToCursors};
#[cfg(not(feature = "fancy"))]
use miette::JSONReportHandler;
#[cfg(feature = "fancy")]
use miette::{GraphicalReportHandler, GraphicalTheme};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn main() {
	#[cfg(feature = "console_error_panic_hook")]
	console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn lex(source_text: String) -> Result<JsValue, serde_wasm_bindgen::Error> {
	let mut lex = Lexer::new(&CssAtomSet::ATOMS, source_text.as_str());
	let serializer = serde_wasm_bindgen::Serializer::json_compatible();
	let mut tokens = vec![];
	loop {
		let token = lex.advance();
		tokens.push(token);
		if token.kind() == Kind::Eof {
			break;
		}
	}
	Ok(tokens.serialize(&serializer).unwrap())
}

#[wasm_bindgen]
pub fn parse(source_text: String) -> Result<SerializableParserResult, serde_wasm_bindgen::Error> {
	let allocator = Bump::default();
	let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text.as_str());
	let result = Parser::new(&allocator, source_text.as_str(), lexer).parse_entirely::<StyleSheet>();
	let serializer = serde_wasm_bindgen::Serializer::json_compatible();
	let diagnostics = result
		.errors
		.iter()
		.map(|err| {
			let DiagnosticMeta { code, message, help, .. } = (err.formatter)(err, &source_text);
			let span = err.start_cursor.span() + err.end_cursor.span();
			let from = span.start().into();
			let to = span.end().into();
			SerializableDiagnostic {
				from,
				to,
				code: code.to_string(),
				severity: err.severity.to_string(),
				message,
				help,
			}
			.serialize(&serializer)
			.unwrap()
		})
		.collect::<Vec<_>>();
	Ok(SerializableParserResult { ast: result.output.serialize(&serializer).unwrap(), diagnostics })
}

#[wasm_bindgen]
pub fn minify(source_text: String) -> Result<String, serde_wasm_bindgen::Error> {
	let allocator = Bump::default();
	let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text.as_str());
	let result = Parser::new(&allocator, source_text.as_str(), lexer).parse_entirely::<StyleSheet>();
	if !result.errors.is_empty() {
		return Err(serde_wasm_bindgen::Error::new("Parse error"));
	}
	let mut output_string = String::new();
	let mut stream = CursorCompactWriteSink::new(&source_text, &mut output_string);
	result.to_cursors(&mut stream);
	Ok(output_string)
}

#[wasm_bindgen]
pub fn parse_error_report(source_text: String) -> String {
	let allocator = Bump::default();
	let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text.as_str());
	let result = Parser::new(&allocator, source_text.as_str(), lexer).parse_entirely::<StyleSheet>();
	let mut report = String::new();
	for err in result.errors {
		build_error(&err, &source_text, &mut report);
		report += "\n";
	}
	report
}

fn build_error(err: &Diagnostic, source: &str, w: &mut impl Write) {
	#[cfg(feature = "miette")]
	{
		#[cfg(feature = "fancy")]
		let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
		#[cfg(not(feature = "fancy"))]
		let handler = JSONReportHandler::new();

		let miette_err = err.into_diagnostic(source);
		let err_with_source = miette::Report::new(miette_err);
		if handler.render_report(w, &*err_with_source).is_ok() {
			return;
		}
	}
	let DiagnosticMeta { code, message, help, .. } = (err.formatter)(err, source);
	write!(w, "Error [{code}]: {message}\nHelp: {help}\n").unwrap();
}

#[wasm_bindgen]
#[derive(Debug)]
pub struct SerializableParserResult {
	ast: JsValue,
	diagnostics: Vec<JsValue>,
}

#[derive(Default, Clone, Serialize)]
pub struct SerializableDiagnostic {
	pub from: usize,
	pub to: usize,
	pub code: String,
	pub severity: String,
	pub message: String,
	pub help: String,
}

#[wasm_bindgen]
impl SerializableParserResult {
	#[wasm_bindgen(getter)]
	pub fn ast(&self) -> JsValue {
		self.ast.clone()
	}

	#[wasm_bindgen(getter)]
	pub fn diagnostics(&self) -> Vec<JsValue> {
		self.diagnostics.clone()
	}
}
