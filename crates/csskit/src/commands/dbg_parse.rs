use crate::{CliResult, GlobalConfig, InputArgs};
use bumpalo::Bump;
use clap::Args;
use css_ast::StyleSheet;
use css_parse::Parser;
use std::io::Read;

/// Show the debug output for a parsed file
#[derive(Debug, Args)]
pub struct DbgParse {
	#[command(flatten)]
	content: InputArgs,
}

impl DbgParse {
	pub fn run(&self, _config: GlobalConfig) -> CliResult {
		let DbgParse { content } = self;
		let bump = Bump::default();
		for (file_name, mut source) in content.sources()? {
			let mut source_string = String::new();
			source.read_to_string(&mut source_string)?;
			let source_text = source_string.as_str();
			let mut parser = Parser::new(&bump, source_text);
			let result = parser.parse_entirely::<StyleSheet>();
			if let Some(stylesheet) = &result.output {
				println!("{stylesheet:#?}");
			} else {
				for compact_err in result.errors {
					let report = crate::commands::format_diagnostic_error(&compact_err, &source_string, file_name);
					println!("{report}");
				}
			}
		}
		Ok(())
	}
}
