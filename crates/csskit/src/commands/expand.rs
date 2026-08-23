use super::prelude::*;
use css_parse::{CursorExpandedWriteSink, ToCursors};

/// Expand CSS files to their most verbose form (the opposite of minify).
#[derive(Debug, Args)]
pub struct Expand {
	/// A list of CSS files to expand. Each input will result in one output file.
	#[command(flatten)]
	content: InputArgs,

	/// Where to save files.
	#[arg(short, long, group = "output_file", value_parser)]
	output: Option<String>,

	/// Don't write any files, instead report each change that would have been made.
	/// This will exit with a non-zero status code if any changes need to be made. Useful for CI.
	#[arg(long, value_parser)]
	check: bool,

	/// Number of extra semicolons to add after each semicolon (0-255).
	#[arg(long, default_value = "0", value_parser)]
	semicolons: u8,

	/// Escape all identifier characters as hex codes (e.g. foo -> \66\6f\6f).
	#[arg(long, value_parser)]
	escape_idents: bool,
}

impl Expand {
	pub fn run(&self, _config: GlobalConfig) -> CliResult {
		let Expand { content, output, check, semicolons, escape_idents } = self;
		let alloc = Arena::default();
		let start = std::time::Instant::now();
		if *check && output.is_some() {
			eprintln!("Ignoring output option, because check was passed");
		}
		let mut checks = 0;
		let mut parse_failures = 0;
		for (file_name, source) in content.sources()? {
			let source_text = css_parse::String::from_reader_in(source, &alloc)?.into_str();
			let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
			let mut parser = Parser::new(&alloc, source_text, lexer);
			let result = parser.parse_entirely::<StyleSheet>();
			if let Some(ref _stylesheet) = result.output {
				let mut str = css_parse::String::new_in(&alloc);
				let mut stream = CursorExpandedWriteSink::new(source_text, &mut str)
					.with_extra_semicolons(*semicolons)
					.with_escape_idents(*escape_idents);
				result.to_cursors(&mut stream);
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
				parse_failures += 1;
				for compact_err in result.errors {
					let report = crate::commands::format_diagnostic_error(&compact_err, source_text, file_name);
					println!("{report}");
				}
			}
		}
		eprintln!("Bloated your CSS in {:?}! Chunky!", start.elapsed());
		if parse_failures > 0 {
			Err(CliError::ParseFailed)?
		} else if checks > 0 {
			Err(CliError::Checks(checks))?
		} else {
			Ok(())
		}
	}
}
