use crate::{CliError, CliResult, GlobalConfig, InputArgs};
use bumpalo::Bump;
use clap::Args;
use css_ast::{StyleSheet, Visitable};
use css_lexer::QuoteStyle;
use css_parse::{CursorPrettyWriteSink, ToCursors, parse};
use csskit_highlight::{AnsiHighlightCursorStream, DefaultAnsiTheme, TokenHighlighter};
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};
use std::io::Read;

/// Format CSS files to make them more readable.
#[derive(Debug, Args)]
pub struct Fmt {
	#[command(flatten)]
	content: InputArgs,

	/// Where to save files.
	#[arg(short, long, group = "output_file", value_parser)]
	output: Option<String>,

	/// Don't write any files, instead report each change that would have been made.
	/// This will exit with a non-zero status code if any changes need to be made. Useful for CI.
	#[arg(long, value_parser)]
	check: bool,

	/// Expand tab characters into a number of spaces.
	#[arg(short('t'), long, value_parser)]
	expand_tab: Option<u8>,

	/// Rewrite quotes to single quotes
	#[arg(long, value_parser)]
	single_quotes: bool,
}

impl Fmt {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		let Fmt { content, output, check, expand_tab, single_quotes } = self;
		let color = config.colors() && output.is_none() && !*check;
		let bump = Bump::default();
		let start = std::time::Instant::now();
		let quotes = if *single_quotes { QuoteStyle::Single } else { QuoteStyle::Double };
		if *check && output.is_some() {
			eprintln!("Ignoring output option, because check was passed");
		}
		let mut checks = 0;
		for (file_name, mut source) in content.sources()? {
			let mut source_string = String::new();
			source.read_to_string(&mut source_string)?;
			let source_text = source_string.as_str();
			let result = parse!(in bump &source_text as StyleSheet);
			if let Some(stylesheet) = result.output.as_ref() {
				let mut str = String::new();
				if color {
					let mut highlighter = TokenHighlighter::new();
					stylesheet.accept(&mut highlighter);
					let ansi = AnsiHighlightCursorStream::new(&mut str, highlighter, DefaultAnsiTheme);
					let mut stream = CursorPrettyWriteSink::new(source_text, ansi, *expand_tab, quotes);
					result.to_cursors(&mut stream);
				} else {
					let mut stream = CursorPrettyWriteSink::new(source_text, &mut str, *expand_tab, quotes);
					result.to_cursors(&mut stream);
				}
				if *check {
					if str != source_text {
						println!("{str}");
						checks += 1;
					}
				} else if let Some(file) = output {
					std::fs::write(file, str.as_bytes())?;
				} else {
					println!("{str}");
				}
			} else {
				let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
				for err in result.errors {
					let mut report = String::new();
					let named = NamedSource::new(file_name, source_string.clone());
					let err = err.with_source_code(named);
					handler.render_report(&mut report, err.as_ref())?;
					println!("{report}");
				}
			}
		}
		eprintln!("Slurped up CSS in {:?}! Neat!", start.elapsed());
		if checks > 0 { Err(CliError::Checks(checks))? } else { Ok(()) }
	}
}
