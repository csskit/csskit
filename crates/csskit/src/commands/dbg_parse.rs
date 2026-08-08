use super::prelude::*;

/// Show the debug output for a parsed file
#[derive(Debug, Args)]
pub struct DbgParse {
	#[command(flatten)]
	content: InputArgs,
}

impl DbgParse {
	pub fn run(&self, _config: GlobalConfig) -> CliResult {
		let DbgParse { content } = self;
		let alloc = Arena::default();
		for (file_name, source) in content.sources()? {
			let source_text = css_parse::String::from_reader_in(source, &alloc)?.into_str();
			let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
			let mut parser = Parser::new(&alloc, source_text, lexer);
			let result = parser.parse_entirely::<StyleSheet>();
			if let Some(stylesheet) = &result.output {
				println!("{stylesheet:#?}");
			} else {
				for compact_err in result.errors {
					let report = crate::commands::format_diagnostic_error(&compact_err, source_text, file_name);
					println!("{report}");
				}
			}
		}
		Ok(())
	}
}
