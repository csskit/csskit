#![deny(warnings)]
use clap::{Args, ColorChoice, Parser};
pub use errors::{CliError, CliResult};
use std::io::{IsTerminal, stderr};

mod color_ext;
mod commands;
mod errors;
mod input;

pub use color_ext::{bg, bold, dimmed, fg, green, magenta};
pub use input::{InputArgs, InputSource};

#[derive(Debug, Args)]
#[group(required = false)]
pub struct GlobalConfig {
	#[arg(short, long)]
	pub debug: bool,

	#[arg(long, value_enum, default_value = "auto")]
	color: ColorChoice,
}

impl GlobalConfig {
	pub fn colors(&self) -> bool {
		match self.color {
			ColorChoice::Auto => stderr().is_terminal(),
			ColorChoice::Always => true,
			ColorChoice::Never => false,
		}
	}
}

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: commands::Commands,

	#[command(flatten)]
	config: GlobalConfig,
}

fn main() -> CliResult {
	let Cli { config, command } = Cli::parse();
	command.run(config)
}
