use crate::{CliError, CliResult, GlobalConfig, InputArgs};
use bumpalo::Bump;
use clap::Args;
use css_ast::StyleSheet;
use css_parse::{CursorCompactWriteSink, Parser, ToCursors};
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};
use std::io::Read;

/// Convert one or more CSS files into production ready CSS.
#[derive(Debug, Args)]
#[command(arg_required_else_help(true))]
pub struct Build {
	#[command(flatten)]
	content: InputArgs,

	/// Where to save files.
	#[arg(short, long, group = "output_file", value_parser)]
	output: Option<String>,
}

impl Build {
	pub fn run(&self, _config: GlobalConfig) -> CliResult {
		let Build { content, output } = self;
		let bump = Bump::default();
		let mut str = String::new();
		let start = std::time::Instant::now();
		for (file_name, mut source) in content.sources()? {
			let mut source_string = String::new();
			source.read_to_string(&mut source_string)?;
			let source_text = source_string.as_str();
			let mut stream = CursorCompactWriteSink::new(source_text, &mut str);
			let mut parser = Parser::new(&bump, source_text);
			let result = parser.parse_entirely::<StyleSheet>();
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
