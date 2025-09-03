#![deny(warnings)]
use bumpalo::Bump;
use clap::{Parser, Subcommand, crate_version};
use css_ast::StyleSheet;
use css_lexer::QuoteStyle;
use css_parse::{CursorCompactWriteSink, CursorPrettyWriteSink, ToCursors, parse};
use csskit_lsp::{LSPService, Server};
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};
use std::{io::stderr, process::ExitCode};
use tracing::{level_filters::LevelFilter, trace};
use tracing_subscriber::{Layer, fmt, layer::SubscriberExt, registry, util::SubscriberInitExt};

#[derive(Debug, Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
	#[command(subcommand)]
	command: Commands,

	#[arg(short, long)]
	debug: bool,
}

#[derive(Subcommand, Debug)]
enum Commands {
	/// Report potential issues around some CSS files
	Check {
		/// A list of CSS files to build. Each input will result in one output file.
		#[arg(required = true, value_parser)]
		input: Vec<String>,

		/// Automatically apply suggested fixes
		#[arg(short, long, value_parser)]
		fix: bool,
	},

	/// Format CSS files to make them more readable.
	Fmt {
		/// A list of CSS files to build. Each input will result in one output file.
		#[arg(required = true, value_parser)]
		input: Vec<String>,

		/// Where to save files.
		#[arg(short, long, group = "output_file", value_parser)]
		output: Option<String>,

		/// Don't write any files, instead report each change that would have been made.
		/// This will exit with a non-zero status code if any changes need to be made. Useful for CI.
		#[arg(short, long, value_parser)]
		check: bool,

		/// Expand tab characters into a number of spaces.
		#[arg(short('t'), long, value_parser)]
		expand_tab: Option<u8>,

		/// Rewrite quotes to single quotes
		#[arg(long, value_parser)]
		single_quotes: bool,
	},

	/// Minify CSS files to compress them optimized delivery.
	Min {
		/// A list of CSS files to build. Each input will result in one output file.
		#[arg(required = true, value_parser)]
		input: Vec<String>,

		/// Where to save files.
		#[arg(short, long, group = "output_file", value_parser)]
		output: Option<String>,

		/// Don't write any files, instead report each change that would have been made.
		/// This will exit with a non-zero status code if any changes need to be made. Useful for CI.
		#[arg(short, long, value_parser)]
		check: bool,
	},

	#[command(hide = true)]
	/// Show the debug output for a parsed file
	DbgParse {
		/// A CSS file to parse.
		#[arg(required = true, value_parser)]
		input: String,
	},

	/// Convert one or more CSS files into production ready CSS.
	#[command(arg_required_else_help(true))]
	Build {
		/// A list of CSS files to build. Each input will result in one output file.
		#[arg(required = true, value_parser)]
		input: Vec<String>,

		/// Where to save files.
		#[arg(short, long, group = "output_file", value_parser)]
		output: Option<String>,
	},

	/// Run the LSP server. It's unlikely you want to run this, but your IDE might!
	Lsp {},
}

enum CliError {
	ParseFailed,
	Checks(usize),
	#[allow(dead_code)]
	Io(std::io::Error),
	Fmt(std::fmt::Error),
}

impl From<std::io::Error> for CliError {
	fn from(err: std::io::Error) -> Self {
		Self::Io(err)
	}
}

impl From<std::fmt::Error> for CliError {
	fn from(err: std::fmt::Error) -> Self {
		Self::Fmt(err)
	}
}

impl std::fmt::Debug for CliError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::ParseFailed => write!(f, "Parsing Failed!"),
			Self::Checks(i) => f.write_str(&format!("{i} files failed check!")),
			Self::Io(arg0) => f.debug_tuple("::io::Error").field(arg0).finish(),
			Self::Fmt(arg0) => f.debug_tuple("::fmt::Error").field(arg0).finish(),
		}
	}
}

impl From<CliError> for ExitCode {
	fn from(val: CliError) -> Self {
		match val {
			CliError::Checks(i) => (i as u8).into(),
			_ => ExitCode::FAILURE,
		}
	}
}

type CliResult = Result<(), CliError>;

fn main() -> CliResult {
	let cli = Cli::parse();
	let debug = cli.debug;

	match &cli.command {
		Commands::Check { input, fix } => {
			todo!("Check ({:?}, {:?})", input, fix);
		}

		Commands::Fmt { input, output, check, expand_tab, single_quotes } => {
			let bump = Bump::default();
			let start = std::time::Instant::now();
			let quotes = if *single_quotes { QuoteStyle::Single } else { QuoteStyle::Double };
			if *check && output.is_some() {
				eprintln!("Ignoring output option, because check was passed");
			}
			let mut checks = 0;
			for file_name in input.iter() {
				let source_string = std::fs::read_to_string(file_name)?;
				let source_text = source_string.as_str();
				let result = parse!(in bump &source_text as StyleSheet);
				if result.output.is_some() {
					let mut str = String::new();
					let mut stream = CursorPrettyWriteSink::new(source_text, &mut str, *expand_tab, quotes);
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
			eprintln!("Slurped up CSS in {:?}! Neat!", start.elapsed());
			if checks > 0 {
				Err(CliError::Checks(checks))?;
			}
		}

		Commands::Min { input, output, check } => {
			let bump = Bump::default();
			let start = std::time::Instant::now();
			if *check && output.is_some() {
				eprintln!("Ignoring output option, because check was passed");
			}
			// Handle multiple files
			let mut checks = 0;
			for file_name in input.iter() {
				let source_string = std::fs::read_to_string(file_name)?;
				let source_text = source_string.as_str();
				let result = parse!(in bump &source_text as StyleSheet);
				if result.output.is_some() {
					let mut str = String::new();
					let mut stream = CursorCompactWriteSink::new(source_text, &mut str);
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
			eprintln!("Slurped up CSS in {:?}! Neat!", start.elapsed());
			if checks > 0 {
				Err(CliError::Checks(checks))?;
			}
		}

		Commands::DbgParse { input } => {
			let source_string = std::fs::read_to_string(input)?;
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
					let named = NamedSource::new(input, source_string.clone());
					let err = err.with_source_code(named);
					handler.render_report(&mut report, err.as_ref())?;
					println!("{report}");
				}
			}
		}

		Commands::Build { input, output } => {
			let bump = Bump::default();
			let mut str = String::new();
			let start = std::time::Instant::now();
			for file_name in input.iter() {
				let source_string = std::fs::read_to_string(file_name)?;
				let source_text = source_string.as_str();
				let mut stream = CursorCompactWriteSink::new(source_text, &mut str);
				let result = parse!(in bump &source_text as StyleSheet);
				if result.output.is_some() {
					result.to_cursors(&mut stream);
				} else {
					let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
					for err in result.errors {
						let mut report = String::new();
						let named = NamedSource::new(file_name, source_string.clone());
						let err = err.with_source_code(named);
						handler.render_report(&mut report, err.as_ref())?;
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
		}

		Commands::Lsp {} => {
			let server = Server::new(LSPService::new(crate_version!()));
			let stderr_log = fmt::layer().with_writer(stderr).with_filter(if debug {
				LevelFilter::TRACE
			} else {
				LevelFilter::WARN
			});
			registry().with(stderr_log).with(server.tracer()).init();
			let thread = server.listen_stdio()?;
			trace!("Listening on stdin/stdout");
			thread.sender.join().expect("Couldn't start server").ok();
		}
	}
	Ok(())
}
