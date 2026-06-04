use crate::{CliResult, GlobalConfig, InputArgs};
use bumpalo::Bump;
use clap::ValueEnum;
use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::{Lexer, Span};
use css_parse::Parser;
use serde::Serialize;
use std::io::Read;

/// Envelope for a single extracted result from any record-emitting command.
#[derive(Serialize, Debug)]
pub struct Record<T: Serialize> {
	#[serde(flatten)]
	pub location: Location,
	#[serde(flatten)]
	pub data: T,
}

/// Position metadata for a record.
#[derive(Serialize, Debug, Clone)]
pub struct Location {
	pub file: String,
	pub line: u32,
	pub column: u32,
	pub start: usize,
	pub end: usize,
}

impl Location {
	/// Build a Location from a filename, span, and source text.
	/// Line and column are 1-based.
	pub fn from_span(file: impl Into<String>, span: Span, src: &str) -> Self {
		let (line, col) = span.line_and_column(src);
		Self {
			file: file.into(),
			line: line + 1,
			column: col + 1,
			start: usize::from(span.start()),
			end: usize::from(span.end()),
		}
	}
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
/// - `Row`: the command-specific data payload (what goes in `Record.data`)
/// - `extract()`: walk one parsed stylesheet and collect (span, row) pairs
/// - `render_text()`: format one row for text output
///
/// The framework (`run()`) handles:
/// - Looping over input files
/// - Parsing each to a StyleSheet
/// - Calling `extract()` and dispatching to text or JSON output
/// - Error handling and file headers (text mode)
pub trait Extract: Sized {
	/// The command-specific payload type.
	type Row: Serialize;

	/// Per-file context computed after parsing, before rendering.
	/// Use this to build derived data (e.g. a syntax highlighter) that
	/// `render_text` needs but can only be constructed from the stylesheet.
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
	/// Called once per file in text mode, before the first `render_text` call.
	/// Default: returns `FileContext::default()`.
	fn build_context<'a>(&self, _stylesheet: &StyleSheet<'a>, _src: &str) -> Self::FileContext {
		Self::FileContext::default()
	}

	/// Whether to print a filename header before each file's rows in text mode.
	/// Default: true.
	fn show_file_header(&self) -> bool {
		true
	}

	/// Called in text mode just before a file's rows are rendered, after the
	/// file header. Use for per-file preamble output (e.g. "Found N items").
	/// Default: no-op.
	fn render_file_preamble(&self, _file: &str, _row_count: usize, _color: bool) {}

	/// Called in text mode when a file yields no rows.
	/// Default: silent.
	fn on_no_results(&self, _file: &str) {}

	/// Try parsing source as raw content (e.g. a bare selector list or color list)
	/// before falling back to StyleSheet. Populate `out` and return true if the
	/// content parse succeeded; return false to fall through to StyleSheet parsing.
	fn try_content(&self, _src: &str, _bump: &Bump, _out: &mut Vec<(Span, Self::Row)>) -> bool {
		false
	}

	/// Override to handle custom parsing (e.g., find's selector validation).
	/// Default: try try_content first, then parse as StyleSheet.
	/// `on_stylesheet` is called with the stylesheet while it is still in scope,
	/// allowing `build_context` to run before the stylesheet is dropped.
	/// If the file fails to parse, print errors (unless JSON) and return Err(()).
	fn parse_and_extract_file(
		&self,
		file: &str,
		src: &str,
		bump: &Bump,
		on_stylesheet: &mut dyn for<'a> FnMut(&StyleSheet<'a>),
	) -> Result<Vec<(Span, Self::Row)>, ()> {
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
			if matches!(self.format(), OutputFormat::Text) {
				for err in result.errors {
					eprintln!("{}", crate::commands::format_diagnostic_error(&err, src, file));
				}
			}
			Err(())
		}
	}

	/// Run the command: loop files, parse, extract, render.
	fn run(&self, config: GlobalConfig) -> CliResult {
		let bump = Bump::default();
		let mut json_records: Vec<Record<Self::Row>> = Vec::new();
		let mut first_file = true;

		for (filename, mut source) in self.input().sources()? {
			let mut src = String::new();
			source.read_to_string(&mut src)?;

			let mut ctx = Self::FileContext::default();
			let mut on_stylesheet = |ss: &StyleSheet| {
				if matches!(self.format(), OutputFormat::Text) {
					ctx = self.build_context(ss, &src);
				}
			};

			let rows = match self.parse_and_extract_file(filename, &src, &bump, &mut on_stylesheet) {
				Ok(rows) => rows,
				Err(()) => continue,
			};

			if rows.is_empty() {
				if matches!(self.format(), OutputFormat::Text) {
					self.on_no_results(filename);
				}
				continue;
			}

			match self.format() {
				OutputFormat::Text => {
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
					for (span, row) in rows {
						self.render_text(&ctx, filename, &src, span, &row, config.colors());
					}
				}
				OutputFormat::Json => {
					for (span, row) in rows {
						let location = Location::from_span(filename, span, &src);
						json_records.push(Record { location, data: row });
					}
				}
			}
		}

		if matches!(self.format(), OutputFormat::Json) {
			println!("{}", serde_json::to_string_pretty(&json_records)?);
		}

		Ok(())
	}
}
