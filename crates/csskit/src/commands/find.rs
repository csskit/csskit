use crate::{CliError, CliResult, GlobalConfig, InputArgs, commands::format_diagnostic_error};
use anstyle::{AnsiColor, Style};
use bumpalo::Bump;
use clap::{Args, ValueEnum};
use css_ast::{CssAtomSet, StyleSheet, Visitable, visit::NodeId};
use css_lexer::{Cursor, Lexer, SourceOffset};
use css_parse::{NodeWithMetadata, Parser, SourceCursor, SourceCursorSink};
use csskit_ast::{CsskitAtomSet, QuerySelectorList, SelectorMatcher};
use csskit_highlight::{AnsiHighlightCursorStream, DefaultAnsiTheme, TokenHighlighter};
use itertools::Itertools;
use serde::Serialize;
use std::io::Read;
use strsim::levenshtein;

const PATH_STYLE: Style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Magenta)));
const LINE_STYLE: Style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Green)));
const NO_STYLE: Style = Style::new();

#[derive(Serialize)]
struct JsonMatch {
	file: String,
	#[serde(rename = "type")]
	kind: String,
	line: usize,
	column: usize,
	start: usize,
	end: usize,
	text: String,
}

#[derive(Serialize)]
struct JsonCount {
	count: usize,
}

/// Output format for find results.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
	/// Ripgrep-style text output
	#[default]
	Text,
	/// JSON output
	Json,
}

#[derive(Debug, Args)]
#[command(after_help = "Examples:
  csskit find style-rule *.css # Find all style rules
  csskit find ':important' *.css # Find all declarations with `!important`
  csskit find ':prefixed' *.css # Find all vendor prefixed rules and declarations
  csskit find 'media-rule > style-rule' *.css # Find all style-rules within media-rules
  csskit find '[name=color]' *.css # Find all rules or declarations with the name `color`.

Try using `csskit tree file.css` to see what can be selected for.
")]
pub struct Find {
	/// Selector pattern (e.g., "style-rule", ":important", "media-rule > syle-rule")
	selector: String,

	#[command(flatten)]
	input: InputArgs,

	/// Show match count per file instead of matches
	#[arg(long)]
	count: bool,

	/// Output format
	#[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
	format: OutputFormat,
}

/// Returns (line_start, line_end) byte offsets for the line containing `offset`.
fn line_bounds(source: &str, offset: usize) -> (usize, usize) {
	let start = source[..offset].rfind('\n').map_or(0, |i| i + 1);
	let end = source[offset..].find('\n').map_or(source.len(), |i| offset + i);
	(start, end)
}

impl Find {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		let selector_bump = Bump::default();
		let lexer = Lexer::new(&CsskitAtomSet::ATOMS, &self.selector);
		let mut parser = Parser::new(&selector_bump, &self.selector, lexer);
		let result = parser.parse_entirely::<QuerySelectorList>();

		if !result.errors.is_empty() || result.output.as_ref().is_some_and(|n| n.metadata().is_invalid) {
			// Show first error only (subsequent errors may be cascading from the first)
			if let Some(err) = result.errors.first() {
				eprintln!("error: {}", err.message(&self.selector));
			} else {
				eprintln!("Invalid selector '{}'", &self.selector);
			}
			self.suggest_types(&self.selector);
			return Err(CliError::ParseFailed);
		}

		let Some(selectors) = result.output else {
			eprintln!("error: failed to parse selector");
			self.suggest_types(&self.selector);
			return Err(CliError::ParseFailed);
		};

		match self.format {
			OutputFormat::Text => self.output_text(&selectors, &self.selector, config.colors()),
			OutputFormat::Json => self.output_json(&selectors, &self.selector),
		}
	}

	fn output_text(&self, selectors: &QuerySelectorList, selector_str: &str, color: bool) -> CliResult {
		let mut total = 0;
		let mut files = 0;
		let (path_style, line_style) = if color { (PATH_STYLE, LINE_STYLE) } else { (NO_STYLE, NO_STYLE) };

		self.process_files(selectors, selector_str, |filename, src, stylesheet, matches| {
			if files > 0 && !self.count {
				println!();
			}
			files += 1;
			total += matches.len();

			if self.count {
				println!("{filename}:{}", matches.len());
				return;
			}

			// Build highlighter once for the entire file
			let mut highlighter = TokenHighlighter::new();
			stylesheet.accept(&mut highlighter);

			// Print filename header
			println!("{path_style}{filename}{path_style:#}");

			for m in matches {
				let (line, col) = m.span.line_and_column(src);
				let (start, end) = line_bounds(src, m.span.start().into());

				print!("{line_style}{}{line_style:#}:{line_style}{}{line_style:#}:", line + 1, col + 1);

				if color {
					// Use lexer to walk through all tokens in the line, including whitespace
					let line_text = &src[start..end];
					let line_lexer = Lexer::new(&CssAtomSet::ATOMS, line_text);
					let mut line_output = String::new();
					let mut cursor_stream =
						AnsiHighlightCursorStream::new(&mut line_output, &highlighter, DefaultAnsiTheme);

					// Process each cursor in the line
					for cursor in line_lexer {
						// Adjust cursor offset to global coordinates
						let global_offset = SourceOffset(cursor.offset().0 + start as u32);
						let global_cursor = Cursor::new(global_offset, cursor.token());
						let sc = SourceCursor::from(global_cursor, cursor.str_slice(line_text));
						cursor_stream.append(sc);
					}

					println!("{}", line_output);
				} else {
					let line_text = &src[start..end];
					println!("{}", line_text);
				}
			}
		})?;

		if self.count && files > 1 {
			println!("\nTotal: {total}");
		}

		Ok(())
	}

	fn output_json(&self, selectors: &QuerySelectorList, selector_str: &str) -> CliResult {
		if self.count {
			let mut count = 0;
			self.process_files(selectors, selector_str, |_filename, _src, _stylesheet, matches| {
				count += matches.len();
			})?;
			println!("{}", serde_json::to_string(&JsonCount { count })?);
		} else {
			let mut all_matches = Vec::new();
			self.process_files(selectors, selector_str, |filename, src, _stylesheet, matches| {
				for m in matches {
					let (line, col) = m.span.line_and_column(src);

					all_matches.push(JsonMatch {
						file: filename.to_string(),
						kind: m.node_id.tag_name().to_string(),
						line: (line + 1) as usize,
						column: (col + 1) as usize,
						start: usize::from(m.span.start()),
						end: usize::from(m.span.end()),
						text: src[m.span.start().into()..m.span.end().into()].to_string(),
					});
				}
			})?;
			println!("{}", serde_json::to_string_pretty(&all_matches)?);
		}

		Ok(())
	}

	fn process_files<F>(&self, selectors: &QuerySelectorList, selector_str: &str, mut callback: F) -> CliResult
	where
		F: FnMut(&str, &str, &StyleSheet, &[csskit_ast::MatchOutput]),
	{
		let bump = Bump::default();

		for (filename, mut source) in self.input.sources()? {
			let mut src = String::new();
			source.read_to_string(&mut src)?;

			let lexer = Lexer::new(&CssAtomSet::ATOMS, &src);
			let mut parser = Parser::new(&bump, &src, lexer);
			let result = parser.parse_entirely::<StyleSheet>();

			let Some(stylesheet) = result.output.as_ref() else {
				// Only show errors in text mode
				if matches!(self.format, OutputFormat::Text) {
					for err in result.errors {
						eprintln!("{}", format_diagnostic_error(&err, &src, filename));
					}
				}
				continue;
			};

			let matches: Vec<_> = SelectorMatcher::new(selectors, selector_str, &src).run(stylesheet).collect();
			if matches.is_empty() {
				continue;
			}

			callback(filename, &src, stylesheet, &matches);
		}

		Ok(())
	}

	fn suggest_types(&self, input: &str) {
		let type_name = input.split(|c: char| c == ':' || c.is_whitespace()).next().unwrap_or(input);
		if type_name == "*" || type_name.is_empty() {
			return;
		}

		let suggestions: Vec<_> = NodeId::all_variants()
			.map(|id| id.tag_name())
			.map(|name| (name, levenshtein(name, type_name)))
			.sorted_by(|(_, a), (_, b)| a.cmp(b))
			.enumerate()
			.take_while_inclusive(|(i, (_, score))| *i < 4 && *score < 4)
			.collect();

		if !suggestions.is_empty() {
			eprintln!("\nDid you mean:");
			for (_, (s, _)) in suggestions {
				eprintln!("  {s}");
			}
		}
		eprintln!("\nRun 'csskit tree' to see all node types.");
	}
}
