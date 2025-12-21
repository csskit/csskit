use std::io::Read;

use anstyle::{AnsiColor, Style};
use bumpalo::Bump;
use clap::{Args, ValueEnum};
use css_ast::visit::NodeId;
use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::Lexer;
use css_parse::Parser;
use csskit_ast::{CsskitAtomSet, QuerySelectorList, SelectorMatcher};

use crate::{CliError, CliResult, GlobalConfig, InputArgs};

const PATH_STYLE: Style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Magenta)));
const LINE_STYLE: Style = Style::new().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Green)));
const MATCH_STYLE: Style = Style::new().bold().fg_color(Some(anstyle::Color::Ansi(AnsiColor::Red)));
const NO_STYLE: Style = Style::new();

/// Output format for find results.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
	/// Ripgrep-style text output
	#[default]
	Text,
	/// JSON output
	Json,
}

/// Find CSS nodes using selector syntax
///
/// Query the parsed CSS AST using CSS-like selectors. Supports node types,
/// pseudo-classes, attribute selectors, and combinators.
///
/// Examples:
///   csskit find style-rule *.css
///   csskit find '*:important' src/**/*.css
///   csskit find 'media-rule > style-rule' theme.css
#[derive(Debug, Args)]
pub struct Find {
	/// Selector pattern (e.g., "style-rule", "*:important")
	selector: Option<String>,

	#[command(flatten)]
	input: InputArgs,

	/// Show match count per file instead of matches
	#[arg(long)]
	count: bool,

	/// Output format
	#[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
	format: OutputFormat,

	/// List available node types
	#[arg(long)]
	list_types: bool,
}

/// Returns (line_start, line_end) byte offsets for the line containing `offset`.
fn line_bounds(source: &str, offset: usize) -> (usize, usize) {
	let start = source[..offset].rfind('\n').map_or(0, |i| i + 1);
	let end = source[offset..].find('\n').map_or(source.len(), |i| offset + i);
	(start, end)
}

impl Find {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		if self.list_types {
			return self.list_types();
		}

		let Some(selector_str) = &self.selector else {
			eprintln!("error: selector required (use --list-types to see available types)");
			return Err(CliError::ParseFailed);
		};

		let selector_bump = Bump::default();
		let lexer = Lexer::new(&CsskitAtomSet::ATOMS, selector_str);
		let mut parser = Parser::new(&selector_bump, selector_str, lexer);
		let result = parser.parse_entirely::<QuerySelectorList>();

		if !result.errors.is_empty() {
			// Show first error only (subsequent errors may be cascading from the first)
			if let Some(err) = result.errors.first() {
				eprintln!("error: {}", err.message(selector_str));
			}
			self.suggest_types(selector_str);
			return Err(CliError::ParseFailed);
		}

		let Some(selectors) = result.output else {
			eprintln!("error: failed to parse selector");
			self.suggest_types(selector_str);
			return Err(CliError::ParseFailed);
		};

		match self.format {
			OutputFormat::Text => self.output_text(&selectors, selector_str, config.colors()),
			OutputFormat::Json => self.output_json(&selectors, selector_str),
		}
	}

	fn output_text(&self, selectors: &QuerySelectorList, selector_str: &str, color: bool) -> CliResult {
		let bump = Bump::default();
		let mut total = 0;
		let mut files = 0;
		let (path_style, line_style, match_style) =
			if color { (PATH_STYLE, LINE_STYLE, MATCH_STYLE) } else { (NO_STYLE, NO_STYLE, NO_STYLE) };

		for (filename, mut source) in self.input.sources()? {
			let mut src = String::new();
			source.read_to_string(&mut src)?;

			let lexer = Lexer::new(&CssAtomSet::ATOMS, &src);
			let mut parser = Parser::new(&bump, &src, lexer);
			let result = parser.parse_entirely::<StyleSheet>();

			let Some(stylesheet) = result.output.as_ref() else {
				for err in result.errors {
					eprintln!("{}", crate::commands::format_diagnostic_error(&err, &src, filename));
				}
				continue;
			};

			let matches = SelectorMatcher::new(selectors, selector_str, &src).run(stylesheet);
			if matches.is_empty() {
				continue;
			}

			if files > 0 && !self.count {
				println!();
			}
			files += 1;
			total += matches.len();

			if self.count {
				println!("{filename}:{}", matches.len());
				continue;
			}

			for m in &matches {
				let (line, col) = m.span.line_and_column(&src);
				let (ls, le) = line_bounds(&src, m.span.start().into());
				let line_text = &src[ls..le];
				let ms = m.span.start().0 as usize - ls;
				let me = (m.span.end().0 as usize).min(le) - ls;

				println!(
					"{path_style}{filename}{path_style:#}:{line_style}{}{line_style:#}:{line_style}{}{line_style:#}:{}{match_style}{}{match_style:#}{}",
					line + 1,
					col + 1,
					&line_text[..ms],
					&line_text[ms..me],
					&line_text[me..]
				);
			}
		}

		if self.count && files > 1 {
			println!("\nTotal: {total}");
		}

		Ok(())
	}

	fn output_json(&self, selectors: &QuerySelectorList, selector_str: &str) -> CliResult {
		let bump = Bump::default();
		let mut count = 0;
		let mut first = true;

		if !self.count {
			print!("[");
		}

		for (filename, mut source) in self.input.sources()? {
			let mut src = String::new();
			source.read_to_string(&mut src)?;

			let lexer = Lexer::new(&CssAtomSet::ATOMS, &src);
			let mut parser = Parser::new(&bump, &src, lexer);
			let result = parser.parse_entirely::<StyleSheet>();

			let Some(stylesheet) = result.output.as_ref() else {
				continue;
			};

			let matches = SelectorMatcher::new(selectors, selector_str, &src).run(stylesheet);
			if matches.is_empty() {
				continue;
			}

			count += matches.len();
			if self.count {
				continue;
			}

			let filename_json = json_str(filename);
			for m in matches {
				let (line, col) = m.span.line_and_column(&src);
				let (ls, le) = line_bounds(&src, m.span.start().into());

				if first {
					first = false;
					print!("\n  ");
				} else {
					print!(",\n  ");
				}
				print!(
					"{{\"file\":{},\"type\":{},\"line\":{},\"column\":{},\"start\":{},\"end\":{},\"text\":{},\"context\":{}}}",
					filename_json,
					json_str(m.node_id.tag_name()),
					line + 1,
					col + 1,
					usize::from(m.span.start()),
					usize::from(m.span.end()),
					json_str(&src[m.span.start().into()..m.span.end().into()]),
					json_str(&src[ls..le]),
				);
			}
		}

		if self.count {
			println!("{{\"count\":{count}}}");
		} else if first {
			println!("]");
		} else {
			println!("\n]");
		}

		Ok(())
	}

	fn list_types(&self) -> CliResult {
		let mut types: Vec<_> = NodeId::all_variants().map(|id| id.tag_name()).collect();
		types.sort_unstable();

		println!("Node types:");
		let mut prev_prefix = "";
		for name in &types {
			let prefix = name.split('-').next().unwrap_or(name);
			if prefix != prev_prefix && !prev_prefix.is_empty() {
				println!();
			}
			prev_prefix = prefix;
			println!("  {name}");
		}

		println!("\nExamples:");
		println!("  csskit find style-rule *.css");
		println!("  csskit find '*:important' src/**/*.css");
		println!("  csskit find 'media-rule > style-rule' theme.css");

		Ok(())
	}

	fn suggest_types(&self, input: &str) {
		let type_name = input.split(|c: char| c == ':' || c.is_whitespace()).next().unwrap_or(input);
		if type_name == "*" || type_name.is_empty() {
			return;
		}

		let suggestions: Vec<_> = NodeId::all_variants()
			.map(|id| id.tag_name())
			.filter(|name| {
				name.contains(type_name) || type_name.contains(name) || strsim::levenshtein(name, type_name) <= 2
			})
			.take(5)
			.collect();

		if !suggestions.is_empty() {
			eprintln!("\nDid you mean:");
			for s in suggestions {
				eprintln!("  {s}");
			}
		}
		eprintln!("\nRun 'csskit find --list-types' for all types.");
	}
}

/// Escape and quote a string for JSON output.
fn json_str(s: &str) -> String {
	use std::fmt::Write;
	let mut out = String::with_capacity(s.len() + 2);
	out.push('"');
	for c in s.chars() {
		match c {
			'"' => out.push_str("\\\""),
			'\\' => out.push_str("\\\\"),
			'\n' => out.push_str("\\n"),
			'\r' => out.push_str("\\r"),
			'\t' => out.push_str("\\t"),
			c if c.is_control() => {
				let _ = write!(out, "\\u{:04x}", c as u32);
			}
			c => out.push(c),
		}
	}
	out.push('"');
	out
}
