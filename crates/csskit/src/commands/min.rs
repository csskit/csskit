use super::GlobalConfig;
use crate::{CliError, CliResult};
use bumpalo::Bump;
use clap::Args;
use css_ast::{StyleSheet, Visitable};
use css_parse::{CursorCompactWriteSink, ToCursors, parse};
use csskit_highlight::{AnsiHighlightCursorStream, DefaultAnsiTheme, TokenHighlighter};
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};
use std::io::{Read, stdin};

/// Minify CSS files to compress them optimized delivery.
#[derive(Debug, Args)]
pub struct Min {
	/// A list of CSS files to build. Each input will result in one output file.
	#[arg(value_parser)]
	input: Vec<String>,

	/// Where to save files.
	#[arg(short, long, group = "output_file", value_parser)]
	output: Option<String>,

	/// Don't write any files, instead report each change that would have been made.
	/// This will exit with a non-zero status code if any changes need to be made. Useful for CI.
	#[arg(short, long, value_parser)]
	check: bool,
}

impl Min {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		let GlobalConfig { mut color, .. } = config;
		let Min { input, output, check } = self;
		color = color && output.is_none() && !*check;
		let bump = Bump::default();
		let start = std::time::Instant::now();
		if *check && output.is_some() {
			eprintln!("Ignoring output option, because check was passed");
		}
		let mut checks = 0;
		let files = if input.is_empty() { &vec!["-".into()] } else { input };
		for file_name in files.iter() {
			let source_string = if file_name == "-" {
				if files.len() != 1 {
					Err(CliError::FilesAndStdin)?;
				}
				let mut buffer = String::new();
				stdin().read_to_string(&mut buffer)?;
				buffer
			} else {
				std::fs::read_to_string(file_name)?
			};
			let source_text = source_string.as_str();
			let result = parse!(in bump &source_text as StyleSheet);
			if let Some(ref stylesheet) = result.output {
				let mut str = String::new();
				if color {
					let mut highlighter = TokenHighlighter::new();
					stylesheet.accept(&mut highlighter);
					let ansi = AnsiHighlightCursorStream::new(&mut str, highlighter, DefaultAnsiTheme);
					let mut stream = CursorCompactWriteSink::new(source_text, ansi);
					result.to_cursors(&mut stream);
				} else {
					let mut stream = CursorCompactWriteSink::new(source_text, &mut str);
					result.to_cursors(&mut stream);
				};
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
