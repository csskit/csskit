use super::GlobalConfig;
use crate::CliResult;
use bumpalo::Bump;
use clap::Args;
use css_ast::StyleSheet;
use css_parse::parse;
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};
use std::io::{Read, stdin};

/// Show the debug output for a parsed file
#[derive(Debug, Args)]
pub struct DbgParse {
	/// A CSS file to parse.
	#[arg(value_parser)]
	input: Option<String>,
}

impl DbgParse {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		let GlobalConfig { .. } = config;
		let DbgParse { input } = self;
		let file = if let Some(f) = input { f } else { &"-".to_owned() };
		let source_string = if file == "-" {
			let mut buffer = String::new();
			stdin().read_to_string(&mut buffer)?;
			buffer
		} else {
			std::fs::read_to_string(file)?
		};
		let source_text = source_string.as_str();
		println!("{source_text}");
		let bump = Bump::default();
		let result = parse!(in bump &source_text as StyleSheet);
		if let Some(stylesheet) = &result.output {
			println!("{stylesheet:#?}");
		} else {
			let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
			for err in result.errors {
				let mut report = String::new();
				let named = NamedSource::new(file, source_string.clone());
				let err = err.with_source_code(named);
				handler.render_report(&mut report, err.as_ref())?;
				println!("{report}");
			}
		}
		Ok(())
	}
}
