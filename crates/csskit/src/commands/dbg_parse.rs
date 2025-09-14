use crate::{CliResult, GlobalConfig, InputArgs};
use bumpalo::Bump;
use clap::Args;
use css_ast::StyleSheet;
use css_parse::Parser;
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};
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
		Ok(())
	}
}
