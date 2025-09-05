use super::GlobalConfig;
use crate::{CliError, CliResult};
use bumpalo::Bump;
use clap::Args;
use css_ast::StyleSheet;
use css_parse::{CursorCompactWriteSink, ToCursors, parse};
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};

/// Convert one or more CSS files into production ready CSS.
#[derive(Debug, Args)]
#[command(arg_required_else_help(true))]
pub struct Build {
	/// A list of CSS files to build. Each input will result in one output file.
	#[arg(required = true, value_parser)]
	input: Vec<String>,

	/// Where to save files.
	#[arg(short, long, group = "output_file", value_parser)]
	output: Option<String>,
}

impl Build {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		let GlobalConfig { .. } = config;
		let Build { input, output } = self;
		let bump = Bump::default();
		let mut str = String::new();
		let start = std::time::Instant::now();
		for file_name in input.iter() {
			let source_string = std::fs::read_to_string(file_name)?;
			let source_text = source_string.as_str();
			let mut stream = CursorCompactWriteSink::new(source_text, &mut str);
			let result = parse!(in bump &source_text as StyleSheet);
			if result.output.is_some() {
				result.to_cursors(&mut stream);
			} else {
				let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
				for err in result.errors {
					let mut report = String::new();
					let named = NamedSource::new(file_name, source_string.clone());
					let err = err.with_source_code(named);
					handler.render_report(&mut report, err.as_ref())?;
					println!("{report}");
				}
				Err(CliError::ParseFailed)?;
			}
		}
		if let Some(file) = output {
			std::fs::write(file, str.as_bytes())?;
		} else {
			println!("{str}");
		}
		eprintln!("Slurped up CSS in {:?}! Neat!", start.elapsed());
		Ok(())
	}
}
