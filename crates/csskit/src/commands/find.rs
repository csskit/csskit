use crate::{
	CliError, CliResult, GlobalConfig, InputArgs,
	commands::{Extract, OutputFormat},
	green,
};
use bumpalo::Bump;
use clap::Args;
use css_ast::{CssAtomSet, StyleSheet, Visitable, visit::NodeId};
use css_lexer::{Cursor, Lexer, LineIndex, SourceOffset, Span};
use css_parse::{NodeWithMetadata, Parser, SourceCursor, SourceCursorSink};
use csskit_ast::{CsskitAtomSet, QuerySelectorList, SelectorMatcher};
use csskit_highlight::{AnsiHighlightCursorStream, DefaultAnsiTheme, TokenHighlighter};
use itertools::Itertools;
use serde::Serialize;
use std::io::Read;
use strsim::levenshtein;

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

#[derive(Serialize)]
pub struct FindData {
	#[serde(rename = "type")]
	kind: String,
	text: String,
}

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

		// In JSON mode, --count is ignored: consumers derive counts from results array length.
		if self.count && !matches!(self.format, OutputFormat::Json) {
			self.output_count(&selectors)
		} else {
			Extract::run(self, config)
		}
	}

	fn output_count(&self, selectors: &QuerySelectorList) -> CliResult {
		let bump = Bump::default();
		let mut total = 0;
		let mut files = 0;

		for (filename, mut source) in self.input.sources()? {
			let mut src = String::new();
			source.read_to_string(&mut src)?;

			let lexer = Lexer::new(&CssAtomSet::ATOMS, &src);
			let mut parser = Parser::new(&bump, &src, lexer);
			let Some(stylesheet) = parser.parse_entirely::<StyleSheet>().output else { continue };
			let count = SelectorMatcher::new(selectors, &self.selector, &src).run(&stylesheet).count();
			if count == 0 {
				continue;
			}

			println!("{filename}:{count}");
			files += 1;
			total += count;
		}

		if files > 1 {
			println!("\nTotal: {total}");
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

	fn parsed_selectors<'b>(&'b self, bump: &'b Bump) -> Option<QuerySelectorList<'b>> {
		let lexer = Lexer::new(&CsskitAtomSet::ATOMS, &self.selector);
		let mut parser = Parser::new(bump, &self.selector, lexer);
		parser.parse_entirely::<QuerySelectorList>().output
	}
}

impl Extract for Find {
	type Row = FindData;
	type FileContext = TokenHighlighter;

	fn input(&self) -> &InputArgs {
		&self.input
	}

	fn format(&self) -> OutputFormat {
		self.format
	}

	fn build_context<'a>(&self, stylesheet: &StyleSheet<'a>, _src: &str) -> TokenHighlighter {
		let mut highlighter = TokenHighlighter::new();
		let _ = stylesheet.accept(&mut highlighter);
		highlighter
	}

	fn extract<'a>(&self, stylesheet: &StyleSheet<'a>, src: &str, out: &mut Vec<(Span, FindData)>) {
		let bump = Bump::default();
		let Some(selectors) = self.parsed_selectors(&bump) else { return };
		for m in SelectorMatcher::new(&selectors, &self.selector, src).run(stylesheet) {
			let start = usize::from(m.span.start());
			let end = usize::from(m.span.end());
			let text = src[start..end].to_string();
			out.push((m.span, FindData { kind: m.node_id.tag_name().to_string(), text }));
		}
	}

	fn render_text(
		&self,
		ctx: &TokenHighlighter,
		_file: &str,
		src: &str,
		index: &LineIndex,
		span: Span,
		_row: &FindData,
		color: bool,
	) {
		let (line, col) = index.line_and_column(span);
		let (start, end) = line_bounds(src, span.start().into());

		if color {
			print!("{}:{}:", green(line + 1), green(col + 1));
			let line_text = &src[start..end];
			let line_lexer = Lexer::new(&CssAtomSet::ATOMS, line_text);
			let mut line_output = String::new();
			let mut cursor_stream = AnsiHighlightCursorStream::new(&mut line_output, ctx, DefaultAnsiTheme);
			for cursor in line_lexer {
				let global_offset = SourceOffset(cursor.offset().0 + start as u32);
				let global_cursor = Cursor::new(global_offset, cursor.token());
				let sc = SourceCursor::from(global_cursor, cursor.str_slice(line_text));
				cursor_stream.append(sc);
			}
			println!("{}", line_output);
		} else {
			println!("{}:{}:{}", line + 1, col + 1, &src[start..end]);
		}
	}
}
