#![deny(warnings)]
use crate::commands::GlobalConfig;
use clap::{ColorChoice, Parser};
pub use errors::{CliError, CliResult};
use std::io::{IsTerminal, stderr};

mod commands;
mod errors;

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: commands::Commands,

	#[arg(short, long)]
	debug: bool,

	#[arg(long, value_enum, default_value = "auto")]
	color: ColorChoice,
}

fn main() -> CliResult {
	let Cli { debug, color, command } = Cli::parse();
	let config = GlobalConfig {
		debug,
		color: match color {
			ColorChoice::Auto => stderr().is_terminal(),
			ColorChoice::Always => true,
			ColorChoice::Never => false,
		},
	};
	command.run(config)
}
