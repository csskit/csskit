use super::GlobalConfig;
use crate::CliResult;
use clap::Args;

/// Report potential issues around some CSS files
#[derive(Debug, Args)]
pub struct Check {
	/// A list of CSS files to build. Each input will result in one output file. If no files are provided, reads from STDIN.
	#[arg(value_parser)]
	input: Vec<String>,

	/// Automatically apply suggested fixes
	#[arg(short, long, value_parser)]
	fix: bool,
}

impl Check {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		let GlobalConfig { .. } = config;
		let Self { input, fix } = self;
		todo!("Check ({:?}, {:?})", input, fix);
	}
}
