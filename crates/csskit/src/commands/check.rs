use crate::{
	CliError, CliResult, GlobalConfig, bold_green, bold_red,
	commands::{
		OutputFormat,
		extract::{ErrorKind, ErrorRecord, Location},
		format_diagnostic_error,
	},
};
use clap::Args;
use css_ast::CssAtomSet;
use css_lexer::{DynAtomSet, Lexer, LineIndex, RegisteredAtomSet};
use css_parse::{Arena, Parser};
use csskit_ast::{Collector, CsskitAtomSet, ResolvedDiagnosticLevel, StatType, sheet::Sheet};
use csskit_highlight::CssHighlighter;
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource, Report};
use serde::Serialize;
use std::{collections::HashMap, fs};

/// A single stat record in JSON output.
#[derive(Serialize)]
struct StatRecord {
	name: String,
	value: usize,
	unit: Option<&'static str>,
}

impl StatRecord {
	fn new(name: String, stat_type: StatType, count: usize) -> Self {
		let unit = match stat_type {
			StatType::Counter => None,
			StatType::Bytes => Some("bytes"),
			StatType::Lines => Some("lines"),
		};
		Self { name, value: count, unit }
	}
}

/// JSON data payload for a single diagnostic.
#[derive(Serialize)]
struct DiagnosticData {
	severity: String,
	message: String,
	#[serde(flatten)]
	location: Location,
}

/// Per-file JSON envelope for check output.
#[derive(Serialize)]
struct CheckFileEnvelope {
	file: String,
	ok: bool,
	#[serde(skip_serializing_if = "Option::is_none")]
	error: Option<ErrorRecord>,
	diagnostics: Vec<DiagnosticData>,
	stats: Vec<StatRecord>,
}

/// Top-level JSON output for check command.
#[derive(Serialize)]
struct CheckOutput {
	files: Vec<CheckFileEnvelope>,
	aggregate_stats: Vec<StatRecord>,
}

/// Report potential issues around some CSS files
#[derive(Debug, Args)]
pub struct Check {
	/// The csskit sheet file (.cks)
	#[arg(value_parser)]
	sheet: String,

	/// A list of CSS files to check
	#[arg(value_parser)]
	input: Vec<String>,

	/// Automatically apply suggested fixes
	#[arg(short, long, value_parser)]
	fix: bool,

	/// Treat warnings as errors
	#[arg(long)]
	deny_warnings: bool,

	/// Output format
	#[arg(long, value_enum, default_value_t = OutputFormat::Text)]
	format: OutputFormat,
}

impl Check {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		let Self { sheet, input, fix, deny_warnings, format } = self;

		if *fix {
			todo!()
		}

		if input.is_empty() {
			hint_no_input(sheet, &config);
			return Err(CliError::ParseFailed);
		}

		let alloc = Arena::new();

		// Read and parse the csskit sheet
		let rule_source = css_parse::String::from_reader_in(fs::File::open(sheet)?, &alloc)?.into_str();
		let rule_lexer = Lexer::new(CsskitAtomSet::get_dyn_set(), rule_source);
		let mut rule_parser = Parser::new(&alloc, rule_source, rule_lexer);
		let rule_result = rule_parser.parse_entirely::<Sheet>();
		let parsed_rules = rule_result.output.ok_or_else(|| {
			if let Some(e) = rule_result.errors.first() {
				eprintln!("{}", format_diagnostic_error(e, rule_source, sheet));
			}
			hint_bad_sheet(sheet, input, &config);
			CliError::ParseFailed
		})?;

		let mut aggregated_stats: HashMap<_, (StatType, usize)> = HashMap::new();
		let mut error_count = 0;
		let mut file_envelopes: Vec<CheckFileEnvelope> = Vec::new();

		for css_file_path in input.iter() {
			let css_source =
				match fs::File::open(css_file_path).and_then(|file| css_parse::String::from_reader_in(file, &alloc)) {
					Ok(source) => source.into_str(),
					Err(e) => {
						match format {
							OutputFormat::Json => {
								file_envelopes.push(CheckFileEnvelope {
									file: css_file_path.clone(),
									ok: false,
									error: Some(ErrorRecord { kind: ErrorKind::Io, message: e.to_string() }),
									diagnostics: Vec::new(),
									stats: Vec::new(),
								});
							}
							OutputFormat::Text => eprintln!("{css_file_path}: {e}"),
						}
						error_count += 1;
						continue;
					}
				};

			let css_lexer = Lexer::new(&CssAtomSet::ATOMS, css_source);
			let mut css_parser = Parser::new(&alloc, css_source, css_lexer);
			let css_result = css_parser.parse_entirely();

			let stylesheet = match css_result.output {
				Some(s) => s,
				None => {
					let message = css_result
						.errors
						.first()
						.map(|e| {
							eprintln!("{}", format_diagnostic_error(e, css_source, css_file_path));
							e.message(css_source).to_string()
						})
						.unwrap_or_else(|| "parse failed".to_string());
					match format {
						OutputFormat::Json => {
							file_envelopes.push(CheckFileEnvelope {
								file: css_file_path.clone(),
								ok: false,
								error: Some(ErrorRecord { kind: ErrorKind::ParseError, message }),
								diagnostics: Vec::new(),
								stats: Vec::new(),
							});
						}
						OutputFormat::Text => {}
					}
					error_count += 1;
					continue;
				}
			};

			let mut collector = Collector::new(&parsed_rules, rule_source, &alloc);
			collector.collect(&stylesheet, css_source);

			let mut file_failed = false;
			let mut file_diagnostics: Vec<DiagnosticData> = Vec::new();
			let line_index = matches!(format, OutputFormat::Json).then(|| LineIndex::new_in(css_source, &alloc));

			for diagnostic in collector.diagnostics(css_source) {
				let is_error = matches!(diagnostic.severity, ResolvedDiagnosticLevel::Error);
				let is_warning = matches!(diagnostic.severity, ResolvedDiagnosticLevel::Warning);
				let counts_as_failure = is_error || (*deny_warnings && is_warning);

				if counts_as_failure && !file_failed {
					error_count += 1;
					file_failed = true;
				}

				match format {
					OutputFormat::Json => {
						let location = line_index
							.as_ref()
							.map(|index| Location::from_span(diagnostic.span, index))
							.expect("line index is built for JSON output");
						file_diagnostics.push(DiagnosticData {
							severity: diagnostic.severity.to_string(),
							message: diagnostic.message.clone(),
							location,
						});
					}
					OutputFormat::Text => {
						let handler = if config.colors() {
							let highlighter = CssHighlighter::new(css_source.to_string(), &stylesheet);
							GraphicalReportHandler::new_themed(GraphicalTheme::unicode())
								.with_syntax_highlighting(highlighter)
						} else {
							GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor())
						};

						let miette_diag = diagnostic.into_miette();
						let named_source = NamedSource::new(css_file_path, css_source.to_string());
						let report = Report::new(miette_diag).with_source_code(named_source);
						let mut output = String::new();
						if handler.render_report(&mut output, &*report).is_ok() {
							eprint!("{}", output);
						}
					}
				}
			}

			let mut file_stats: Vec<StatRecord> = Vec::new();
			for (stat_name, (stat_type, count)) in collector.stats() {
				let name = CsskitAtomSet::get_dyn_set().bits_to_str(stat_name.as_bits()).to_string();
				let entry = aggregated_stats.entry(*stat_name).or_insert((*stat_type, 0));
				entry.1 += count;
				file_stats.push(StatRecord::new(name, *stat_type, *count));
			}
			file_stats.sort_by(|a, b| a.name.cmp(&b.name));

			match format {
				OutputFormat::Json => {
					file_envelopes.push(CheckFileEnvelope {
						file: css_file_path.clone(),
						ok: !file_failed,
						error: None,
						diagnostics: file_diagnostics,
						stats: file_stats,
					});
				}
				OutputFormat::Text => {
					if !aggregated_stats.is_empty() && input.len() == 1 {
						// Per-file stats in text mode when only one file
						print_stats_text(&file_stats);
					}
				}
			}
		}

		match format {
			OutputFormat::Json => {
				let mut aggregate: Vec<StatRecord> = aggregated_stats
					.iter()
					.map(|(name, (stat_type, count))| {
						let key = CsskitAtomSet::get_dyn_set().bits_to_str(name.as_bits()).to_string();
						StatRecord::new(key, *stat_type, *count)
					})
					.collect();
				aggregate.sort_by(|a, b| a.name.cmp(&b.name));
				println!(
					"{}",
					serde_json::to_string_pretty(&CheckOutput { files: file_envelopes, aggregate_stats: aggregate })?
				);
			}
			OutputFormat::Text => {
				if !aggregated_stats.is_empty() && input.len() > 1 {
					println!("\nStatistics:");
					let mut stat_entries: Vec<_> = aggregated_stats
						.iter()
						.map(|(name, val)| (CsskitAtomSet::get_dyn_set().bits_to_str(name.as_bits()), val))
						.collect();
					stat_entries.sort_by_key(|(name, _)| *name);
					for (name, (stat_type, count)) in stat_entries {
						let type_label = match stat_type {
							StatType::Counter => "",
							StatType::Bytes => " bytes",
							StatType::Lines => " lines",
						};
						println!("  --{}: {}{}", name, count, type_label);
					}
				}
			}
		}

		if error_count > 0 { Err(CliError::Checks(error_count)) } else { Ok(()) }
	}
}

fn print_stats_text(stats: &[StatRecord]) {
	if stats.is_empty() {
		return;
	}
	println!("\nStatistics:");
	for stat in stats {
		let unit = stat.unit.map(|u| format!(" {u}")).unwrap_or_default();
		println!("  --{}: {}{}", stat.name, stat.value, unit);
	}
}

/// Find the first `.cks` file in cwd, or fall back to `rules.cks`.
fn find_cks_hint() -> String {
	if let Ok(entries) = fs::read_dir(".") {
		let mut names: Vec<String> = entries
			.flatten()
			.filter_map(|e| {
				let name = e.file_name().to_string_lossy().into_owned();
				name.ends_with(".cks").then_some(name)
			})
			.collect();
		names.sort();
		if let Some(name) = names.into_iter().next() {
			return name;
		}
	}
	"rules.cks".to_string()
}

fn maybe_color<F: Fn(&str) -> String>(colors: bool, s: &str, f: F) -> String {
	if colors { f(s) } else { s.to_string() }
}

fn hint_no_input(sheet: &str, config: &GlobalConfig) {
	let colors = config.colors();
	let error_label = maybe_color(colors, "error", |s| bold_red(s));
	let help_label = maybe_color(colors, "help", |s| bold_green(s));
	eprintln!("{}: no CSS files to check", error_label);
	eprintln!();
	eprintln!("{}: usage: csskit check <rules.cks> <file1.css> [more.css...]", help_label);

	if sheet.ends_with(".css") {
		let cks = find_cks_hint();
		let cmd = format!("csskit check {cks} {sheet}");
		let help_label = maybe_color(colors, "help", |s| bold_green(s));
		eprintln!(
			"{}: `{}` looks like a CSS file, did you mean `{}`?",
			help_label,
			sheet,
			maybe_color(colors, &cmd, |s| bold_green(s))
		);
	}
}

fn hint_bad_sheet(sheet: &str, input: &[String], config: &GlobalConfig) {
	if !sheet.ends_with(".css") {
		return;
	}
	let colors = config.colors();
	let help_label = maybe_color(colors, "help", |s| bold_green(s));
	let cks = find_cks_hint();
	let all_css = std::iter::once(sheet).chain(input.iter().map(String::as_str)).collect::<Vec<_>>().join(" ");
	let cmd = format!("csskit check {cks} {all_css}");
	eprintln!();
	eprintln!(
		"{}: `{}` looks like a CSS file, did you mean `{}`?",
		help_label,
		sheet,
		maybe_color(colors, &cmd, |s| bold_green(s))
	);
}
