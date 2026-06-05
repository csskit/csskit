use crate::{CliResult, GlobalConfig, InputArgs};
use bumpalo::Bump;
use clap::ValueEnum;
use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::{Lexer, Span};
use css_parse::Parser;
use serde::Serialize;
use std::io::Read;

/// Position metadata for a single result within a file.
#[derive(Serialize, Debug, Clone)]
pub struct Location {
	pub line: u32,
	pub column: u32,
	pub start: usize,
	pub end: usize,
}

impl Location {
	pub fn from_span(span: Span, src: &str) -> Self {
		let (line, col) = span.line_and_column(src);
		Self { line: line + 1, column: col + 1, start: usize::from(span.start()), end: usize::from(span.end()) }
	}
}

/// A single extracted result: position + command-specific payload.
#[derive(Serialize, Debug)]
pub struct Record<T: Serialize> {
	#[serde(flatten)]
	pub location: Location,
	#[serde(flatten)]
	pub data: T,
}

/// Error kinds for machine-readable failure records.
#[derive(Serialize, Debug, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum ErrorKind {
	ParseError,
	Io,
	#[allow(dead_code)]
	Internal,
}

/// Machine-readable error attached to a file envelope on failure.
#[derive(Serialize, Debug)]
pub struct ErrorRecord {
	pub kind: ErrorKind,
	pub message: String,
}

/// Per-file result envelope emitted in JSON mode.
#[derive(Serialize, Debug)]
pub struct FileEnvelope<T: Serialize> {
	pub file: String,
	pub ok: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub error: Option<ErrorRecord>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub results: Option<Vec<Record<T>>>,
}

impl<T: Serialize> FileEnvelope<T> {
	pub fn ok(file: impl Into<String>, results: Vec<Record<T>>) -> Self {
		Self { file: file.into(), ok: true, error: None, results: Some(results) }
	}

	pub fn err(file: impl Into<String>, error: ErrorRecord) -> Self {
		Self { file: file.into(), ok: false, error: Some(error), results: None }
	}
}

/// Top-level JSON output wrapper.
#[derive(Serialize, Debug)]
pub struct OutputEnvelope<T: Serialize> {
	pub files: Vec<FileEnvelope<T>>,
}

/// Output format for extraction commands.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
	/// Human-readable text output
	#[default]
	Text,
	/// JSON array output
	Json,
}

/// Trait for a command that walks a CSS AST and emits records.
///
/// Implementors define:
/// - `Row`: the command-specific data payload
/// - `extract()`: walk one parsed stylesheet and collect (span, row) pairs
/// - `render_text()`: format one row for text output
///
/// The framework (`run()`) handles:
/// - Looping over input files
/// - Parsing each to a StyleSheet
/// - Calling `extract()` and dispatching to text or JSON output
/// - Per-file envelopes in JSON mode (always emitted, even for empty results)
/// - Error handling and file headers (text mode)
pub trait Extract: Sized {
	/// The command-specific payload type.
	type Row: Serialize;

	/// Per-file context computed after parsing, before rendering.
	type FileContext: Default;

	/// Return reference to input args.
	fn input(&self) -> &InputArgs;

	/// Return the chosen output format.
	fn format(&self) -> OutputFormat;

	/// Walk the stylesheet and emit (span, row) pairs into `out`.
	fn extract<'a>(&self, stylesheet: &StyleSheet<'a>, src: &str, out: &mut Vec<(Span, Self::Row)>);

	/// Format one row for text output.
	fn render_text(&self, ctx: &Self::FileContext, file: &str, src: &str, span: Span, row: &Self::Row, color: bool);

	/// Build the per-file context from the parsed stylesheet and source.
	fn build_context<'a>(&self, _stylesheet: &StyleSheet<'a>, _src: &str) -> Self::FileContext {
		Self::FileContext::default()
	}

	/// Whether to print a filename header before each file's rows in text mode.
	fn show_file_header(&self) -> bool {
		true
	}

	/// Called in text mode just before a file's rows are rendered, after the file header.
	fn render_file_preamble(&self, _file: &str, _row_count: usize, _color: bool) {}

	/// Called in text mode when a file yields no rows.
	fn on_no_results(&self, _file: &str) {}

	/// Try parsing source as raw content before falling back to StyleSheet.
	/// Return true if handled.
	fn try_content(&self, _src: &str, _bump: &Bump, _out: &mut Vec<(Span, Self::Row)>) -> bool {
		false
	}

	/// Parse and extract from one file. Returns Ok(rows) or Err(error_record).
	/// `on_stylesheet` is called with the stylesheet while still in scope.
	fn parse_and_extract_file(
		&self,
		file: &str,
		src: &str,
		bump: &Bump,
		on_stylesheet: &mut dyn for<'a> FnMut(&StyleSheet<'a>),
	) -> Result<Vec<(Span, Self::Row)>, ErrorRecord> {
		let mut rows = Vec::new();
		if self.try_content(src, bump, &mut rows) {
			return Ok(rows);
		}

		let lexer = Lexer::new(&CssAtomSet::ATOMS, src);
		let mut parser = Parser::new(bump, src, lexer);
		let result = parser.parse_entirely::<StyleSheet>();

		if let Some(stylesheet) = result.output {
			on_stylesheet(&stylesheet);
			self.extract(&stylesheet, src, &mut rows);
			Ok(rows)
		} else {
			let message = result
				.errors
				.first()
				.map(|e| {
					if matches!(self.format(), OutputFormat::Text) {
						eprintln!("{}", crate::commands::format_diagnostic_error(e, src, file));
					}
					e.message(src).to_string()
				})
				.unwrap_or_else(|| "parse failed".to_string());
			Err(ErrorRecord { kind: ErrorKind::ParseError, message })
		}
	}

	/// Run the command: loop files, parse, extract, render.
	fn run(&self, config: GlobalConfig) -> CliResult {
		let bump = Bump::default();
		let mut envelopes: Vec<FileEnvelope<Self::Row>> = Vec::new();
		let mut first_file = true;

		for (filename, mut source) in self.input().sources()? {
			let mut src = String::new();
			if let Err(e) = source.read_to_string(&mut src) {
				match self.format() {
					OutputFormat::Json => {
						envelopes.push(FileEnvelope::err(
							filename,
							ErrorRecord { kind: ErrorKind::Io, message: e.to_string() },
						));
					}
					OutputFormat::Text => {
						eprintln!("{filename}: {e}");
					}
				}
				continue;
			}

			let mut ctx = Self::FileContext::default();
			let mut on_stylesheet = |ss: &StyleSheet| {
				if matches!(self.format(), OutputFormat::Text) {
					ctx = self.build_context(ss, &src);
				}
			};

			match self.parse_and_extract_file(filename, &src, &bump, &mut on_stylesheet) {
				Ok(rows) => match self.format() {
					OutputFormat::Text => {
						if rows.is_empty() {
							self.on_no_results(filename);
						} else {
							if !first_file {
								println!();
							}
							first_file = false;
							if self.show_file_header() {
								if config.colors() {
									println!("{}", crate::magenta(filename));
								} else {
									println!("{}", filename);
								}
							}
							self.render_file_preamble(filename, rows.len(), config.colors());
							for (span, row) in &rows {
								self.render_text(&ctx, filename, &src, *span, row, config.colors());
							}
						}
					}
					OutputFormat::Json => {
						let records = rows
							.into_iter()
							.map(|(span, data)| Record { location: Location::from_span(span, &src), data })
							.collect();
						envelopes.push(FileEnvelope::ok(filename, records));
					}
				},
				Err(err) => {
					match self.format() {
						OutputFormat::Json => envelopes.push(FileEnvelope::err(filename, err)),
						OutputFormat::Text => {} // already printed in parse_and_extract_file
					}
				}
			}
		}

		if matches!(self.format(), OutputFormat::Json) {
			println!("{}", serde_json::to_string_pretty(&OutputEnvelope { files: envelopes })?);
		}

		Ok(())
	}
}
