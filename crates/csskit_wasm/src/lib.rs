#![deny(warnings)]
use bumpalo::Bump;
use core::fmt::Write;
use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::{Kind, Lexer, QuoteStyle};
use css_parse::{
	CursorCompactWriteSink, CursorOverlaySink, CursorPrettyWriteSink, Diagnostic, DiagnosticMeta, Parser, ToCursors,
};
use csskit_transform::{CssMinifierFeature, Transformer};
#[cfg(not(feature = "fancy"))]
use miette::JSONReportHandler;
#[cfg(feature = "fancy")]
use miette::{GraphicalReportHandler, GraphicalTheme};
use serde::{Deserialize, Serialize};
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
	let mut result = Parser::new(&allocator, source_text.as_str(), lexer).parse_entirely::<StyleSheet>();
	if !result.errors.is_empty() {
		return Err(serde_wasm_bindgen::Error::new("Parse error"));
	}
	let mut output_string = String::new();
	if let Some(ref mut stylesheet) = result.output {
		let mut transformer =
			Transformer::new_in(&allocator, CssMinifierFeature::all_bits(), &CssAtomSet::ATOMS, &source_text);
		transformer.transform(stylesheet);
		let overlays = transformer.overlays();
		let mut stream = CursorOverlaySink::new(
			&source_text,
			&overlays,
			CursorCompactWriteSink::new(&source_text, &mut output_string),
		);
		result.to_cursors(&mut stream);
	}
	Ok(output_string)
}

#[wasm_bindgen]
pub fn format(source_text: String, options: JsValue) -> Result<String, serde_wasm_bindgen::Error> {
	let options: FormatOptions = match serde_wasm_bindgen::from_value(options) {
		Ok(opts) => opts,
		Err(e) => {
			#[cfg(feature = "console_error_panic_hook")]
			web_sys::console::warn_1(&format!("Failed to parse format options: {}. Using defaults.", e).into());
			FormatOptions::default()
		}
	};
	format_with_options(&source_text, options).map_err(serde_wasm_bindgen::Error::new)
}

fn format_with_options(source_text: &str, options: FormatOptions) -> Result<String, String> {
	let allocator = Bump::default();
	let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
	let result = Parser::new(&allocator, source_text, lexer).parse_entirely::<StyleSheet>();
	if !result.errors.is_empty() {
		let first_error = &result.errors[0];
		let DiagnosticMeta { code, message, help, .. } = (first_error.formatter)(first_error, source_text);
		return Err(format!("Parse error [{}]: {} (Help: {})", code, message, help));
	}
	let mut output_string = String::new();
	if result.output.is_some() {
		let indent_width = options.indent_width.unwrap_or(2).clamp(1, 8);
		let expand_tab = match options.indent_style.unwrap_or(IndentStyle::Spaces) {
			IndentStyle::Spaces => Some(indent_width),
			IndentStyle::Tabs => None,
		};
		let quote_style = match options.quote_style.unwrap_or(QuoteStyleOption::Double) {
			QuoteStyleOption::Double => QuoteStyle::Double,
			QuoteStyleOption::Single => QuoteStyle::Single,
		};
		let mut stream = CursorPrettyWriteSink::new(source_text, &mut output_string, expand_tab, quote_style);
		result.to_cursors(&mut stream);
	}
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum IndentStyle {
	Spaces,
	Tabs,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "snake_case")]
enum QuoteStyleOption {
	Double,
	Single,
}

#[derive(Default, Deserialize)]
#[serde(default, rename_all = "snake_case")]
struct FormatOptions {
	#[serde(alias = "indent-style", alias = "indentStyle")]
	indent_style: Option<IndentStyle>,
	#[serde(alias = "indent-width", alias = "indentWidth")]
	indent_width: Option<u8>,
	#[serde(alias = "quote-style", alias = "quoteStyle")]
	quote_style: Option<QuoteStyleOption>,
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn format_respects_quote_style() {
		let source = r#".a { content: "foo"; }"#;
		let options = FormatOptions { quote_style: Some(QuoteStyleOption::Single), ..FormatOptions::default() };
		let output = format_with_options(source, options).expect("format should succeed");
		assert!(output.contains("content: 'foo'"));
	}
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
