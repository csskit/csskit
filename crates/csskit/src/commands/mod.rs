use crate::CliResult;
use clap::Subcommand;

mod build;
mod check;
mod dbg_parse;
mod fmt;
mod lsp;
mod min;

#[derive(Debug)]
pub struct GlobalConfig {
	pub debug: bool,
	pub color: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
	/// Report potential issues around some CSS files
	Check(check::Check),

	/// Format CSS files to make them more readable.
	Fmt(fmt::Fmt),

	/// Minify CSS files to compress them optimized delivery.
	Min(min::Min),

	#[command(hide = true)]
	/// Show the debug output for a parsed file
	DbgParse(dbg_parse::DbgParse),

	/// Convert one or more CSS files into production ready CSS.
	#[command(arg_required_else_help(true))]
	Build(build::Build),

	/// Run the LSP server. It's unlikely you want to run this, but your IDE might!
	Lsp(lsp::Lsp),
}

impl Commands {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		match self {
			Commands::Check(cmd) => cmd.run(config),
			Commands::Fmt(cmd) => cmd.run(config),
			Commands::Min(cmd) => cmd.run(config),
			Commands::DbgParse(cmd) => cmd.run(config),
			Commands::Build(cmd) => cmd.run(config),
			Commands::Lsp(cmd) => cmd.run(config),
		}
	}
}
