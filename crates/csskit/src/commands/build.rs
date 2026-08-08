use super::prelude::*;
use css_parse::{CursorCompactWriteSink, ToCursors};

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
		let alloc = Arena::default();
		let mut str = css_parse::String::new_in(&alloc);
		let start = std::time::Instant::now();
		for (file_name, source) in content.sources()? {
			let source_text = css_parse::String::from_reader_in(source, &alloc)?.into_str();
			let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
			let mut parser = Parser::new(&alloc, source_text, lexer);
			let result = parser.parse_entirely::<StyleSheet>();
			if result.output.is_some() {
				let mut stream = CursorCompactWriteSink::new(source_text, &mut str);
				result.with_trivia().to_cursors(&mut stream);
			} else {
				for compact_err in result.errors {
					let report = crate::commands::format_diagnostic_error(&compact_err, source_text, file_name);
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
