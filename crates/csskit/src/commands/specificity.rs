use crate::{CliResult, GlobalConfig, InputArgs};
use bumpalo::Bump;
use clap::{Args, ValueEnum};
use css_ast::specificity::ToSpecificity;
use css_ast::{CompoundSelector, CssAtomSet, SelectorList, StyleRule, StyleSheet, Visit, Visitable};
use css_lexer::Lexer;
use css_parse::{Parser, ToSpan};
use serde::Serialize;
use std::io::Read;

#[derive(Serialize)]
struct JsonEntry {
	selector: String,
	a: u8,
	b: u8,
	c: u8,
	line: u32,
	col: u32,
}

/// Output format for specificity results.
#[derive(Debug, Clone, Copy, Default, ValueEnum)]
pub enum OutputFormat {
	/// One `selector (a,b,c)` per line
	#[default]
	Text,
	/// JSON array
	Json,
}

#[derive(Debug, Args)]
pub struct Specificity {
	#[command(flatten)]
	input: InputArgs,

	/// Output format
	#[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
	format: OutputFormat,
}

struct SpecificityVisitor<'a> {
	source: &'a str,
	entries: Vec<JsonEntry>,
}

impl<'a> SpecificityVisitor<'a> {
	fn new(source: &'a str) -> Self {
		Self { source, entries: Vec::new() }
	}
}

impl<'src> Visit for SpecificityVisitor<'src> {
	fn visit_style_rule<'a>(&mut self, rule: &StyleRule<'a>) {
		let selector_list: &SelectorList = &rule.rule.prelude;
		for (compound, _) in &selector_list.0 {
			self.emit(compound);
		}
	}
}

impl<'src> SpecificityVisitor<'src> {
	fn emit(&mut self, compound: &CompoundSelector) {
		let span = compound.to_span();
		let (line, col) = span.line_and_column(self.source);
		let s = compound.specificity();
		let selector = self.source[span.start().into()..span.end().into()].trim().to_string();
		self.entries.push(JsonEntry { selector, a: s.0, b: s.1, c: s.2, line: line + 1, col: col + 1 });
	}
}

impl Specificity {
	pub fn run(&self, _config: GlobalConfig) -> CliResult {
		let bump = Bump::default();

		let mut all_entries: Vec<JsonEntry> = Vec::new();

		for (_filename, mut source) in self.input.sources()? {
			let mut src = String::new();
			source.read_to_string(&mut src)?;

			let lexer = Lexer::new(&CssAtomSet::ATOMS, &src);
			let mut parser = Parser::new(&bump, &src, lexer);
			let result = parser.parse_entirely::<StyleSheet>();

			let Some(stylesheet) = result.output.as_ref() else {
				continue;
			};

			let mut visitor = SpecificityVisitor::new(&src);
			stylesheet.accept(&mut visitor);
			all_entries.extend(visitor.entries);
		}

		match self.format {
			OutputFormat::Text => {
				for e in &all_entries {
					println!("{} ({},{},{})", e.selector, e.a, e.b, e.c);
				}
			}
			OutputFormat::Json => {
				println!("{}", serde_json::to_string_pretty(&all_entries)?);
			}
		}

		Ok(())
	}
}
