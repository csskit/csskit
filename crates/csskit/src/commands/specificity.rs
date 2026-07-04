use crate::InputArgs;
use crate::commands::{Extract, OutputFormat};
use bumpalo::Bump;
use clap::Args;
use css_ast::specificity::ToSpecificity;
use css_ast::{CssAtomSet, SelectorList, StyleRule, StyleSheet, Visit, Visitable, visitor};
use css_lexer::{Lexer, Span};
use css_parse::{Parser, ToSpan};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct SpecificityRow {
	selector: String,
	a: u8,
	b: u8,
	c: u8,
}

fn extract_selector_list(selector_list: &SelectorList, src: &str, out: &mut Vec<(Span, SpecificityRow)>) {
	for (compound, _) in &selector_list.0 {
		let span = compound.to_span();
		let s = compound.specificity();
		let selector = src[span.start().into()..span.end().into()].trim().to_string();
		out.push((span, SpecificityRow { selector, a: s.0, b: s.1, c: s.2 }));
	}
}

struct SpecificityVisitor<'a, 'out> {
	source: &'a str,
	out: &'out mut Vec<(Span, SpecificityRow)>,
}

impl<'a, 'out> SpecificityVisitor<'a, 'out> {
	fn new(source: &'a str, out: &'out mut Vec<(Span, SpecificityRow)>) -> Self {
		Self { source, out }
	}
}

#[visitor]
impl<'src, 'out> Visit for SpecificityVisitor<'src, 'out> {
	fn visit_style_rule<'a>(&mut self, rule: &StyleRule<'a>) {
		extract_selector_list(&rule.rule.prelude, self.source, self.out);
	}
}

#[derive(Debug, Args)]
pub struct Specificity {
	#[command(flatten)]
	input: InputArgs,

	/// Output format
	#[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
	format: OutputFormat,
}

impl Extract for Specificity {
	type Row = SpecificityRow;
	type FileContext = ();

	fn input(&self) -> &InputArgs {
		&self.input
	}

	fn format(&self) -> OutputFormat {
		self.format
	}

	fn try_content(&self, src: &str, bump: &Bump, out: &mut Vec<(Span, Self::Row)>) -> bool {
		let lexer = Lexer::new(&CssAtomSet::ATOMS, src);
		let mut parser = Parser::new(bump, src, lexer);
		let result = parser.parse_entirely::<SelectorList>();
		if let Some(selector_list) = result.output.filter(|_| result.errors.is_empty()) {
			extract_selector_list(&selector_list, src, out);
			return true;
		}
		false
	}

	fn extract<'a>(&self, stylesheet: &StyleSheet<'a>, src: &str, out: &mut Vec<(Span, Self::Row)>) {
		let mut visitor = SpecificityVisitor::new(src, out);
		let _ = stylesheet.accept(&mut visitor);
	}

	fn render_text(&self, _ctx: &(), _file: &str, _src: &str, _span: Span, row: &Self::Row, _color: bool) {
		println!("{} ({},{},{})", row.selector, row.a, row.b, row.c);
	}
}
