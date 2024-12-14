use bumpalo::Bump;
use clap::{Parser, Subcommand};
use hdx_ast::css::StyleSheet;
use hdx_parser::{CursorStream, ToCursors};
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};

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

		/// Don't write any files, instead report each change that would have been made.
		/// This will exit with a non-zero status code if any changes need to be made. Useful for CI.
		#[arg(short, long, value_parser)]
		check: bool,
	},

	/// Convert one or more CSS files into production ready CSS.
	#[command(arg_required_else_help(true))]
	Build {
		/// A list of CSS files to build. Each input will result in one output file.
		#[arg(required = true, value_parser)]
		input: Vec<String>,

		/// Shorthand to apply all safe transforms, such as removing whitespace or redundant code.
		#[arg(short, long, value_parser)]
		minify: bool,

		/// Where to save files.
		#[arg(short, long, group = "output_file", value_parser)]
		output: Option<String>,
	},
}

fn main() {
	let cli = Cli::parse();

	match &cli.command {
		Commands::Check { input, fix } => {
			todo!()
		}
		Commands::Fmt { input, check } => {
			todo!()
		}
		Commands::Build { input, minify, output } => {
			if input.len() > 1 {
				todo!("Can't handle multiple files yet")
			}

			let file_name = input.first().unwrap();
			let source_text = std::fs::read_to_string(file_name).unwrap();
			let allocator = Bump::default();
			let start = std::time::Instant::now();
			let result = hdx_parser::Parser::new(&allocator, source_text.as_str(), hdx_parser::Features::default())
				.parse_entirely::<StyleSheet>();
			{
				if let Some(stylesheet) = &result.output {
					let mut str = String::new();
					let mut stream = CursorStream::new(&allocator);
					result.to_cursors(&mut stream);
					if let Err(e) = result.write(&mut stream, &mut str) {
						println!("{}", e);
					}
					if let Some(file) = output {
						std::fs::write(file, str.as_bytes()).unwrap();
					} else {
						println!("{}", str);
						eprintln!("Slurped up CSS in {:?}! Neat!", start.elapsed());
					}
				} else {
					let handler = GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor());
					for err in result.errors {
						let mut report = String::new();
						let named = NamedSource::new(file_name, source_text.clone());
						let err = err.with_source_code(named);
						handler.render_report(&mut report, err.as_ref()).unwrap();
						println!("{}", report);
					}
				}
			}
		}
	}
}
