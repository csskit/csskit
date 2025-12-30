use crate::{CliError, CliResult, GlobalConfig, commands::format_diagnostic_error};
use bumpalo::Bump;
use clap::Args;
use css_ast::CssAtomSet;
use css_lexer::{DynAtomSet, Lexer, RegisteredAtomSet};
use css_parse::Parser;
use csskit_ast::{Collector, CsskitAtomSet, ResolvedDiagnosticLevel, StatType, sheet::Sheet};
use csskit_highlight::CssHighlighter;
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource, Report};
use std::{collections::HashMap, fs};

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
}

impl Check {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		let Self { sheet, input, fix } = self;

		if *fix {
			todo!()
		}

		let bump = Bump::new();

		// Read and parse the csskit sheet
		let rule_source = fs::read_to_string(sheet)?;
		let rule_lexer = Lexer::new(CsskitAtomSet::get_dyn_set(), &rule_source);
		let mut rule_parser = Parser::new(&bump, &rule_source, rule_lexer);
		let rule_result = rule_parser.parse_entirely::<Sheet>();
		let parsed_rules = rule_result.output.ok_or_else(|| {
			if let Some(e) = rule_result.errors.first() {
				eprintln!("{}", format_diagnostic_error(e, &rule_source, sheet));
			}
			CliError::ParseFailed
		})?;

		// Aggregate statistics across all files
		let mut aggregated_stats = HashMap::new();
		let mut error_count = 0;

		for css_file_path in input.iter() {
			let css_source = fs::read_to_string(css_file_path)?;
			let css_lexer = Lexer::new(&CssAtomSet::ATOMS, &css_source);
			let mut css_parser = Parser::new(&bump, &css_source, css_lexer);
			let css_result = css_parser.parse_entirely();

			let stylesheet = css_result.output.ok_or_else(|| {
				if let Some(e) = css_result.errors.first() {
					eprintln!("{}", format_diagnostic_error(e, &css_source, css_file_path));
				}
				CliError::ParseFailed
			})?;

			let mut collector = Collector::new(&parsed_rules, &rule_source, &bump);
			collector.collect(&stylesheet, &css_source);

			let mut file_failed = false;

			for diagnostic in collector.diagnostics(&css_source) {
				if matches!(diagnostic.severity, ResolvedDiagnosticLevel::Error) && !file_failed {
					error_count += 1;
					file_failed = true;
				}

				let handler = if config.colors() {
					let highlighter = CssHighlighter::new(css_source.clone(), &stylesheet);
					GraphicalReportHandler::new_themed(GraphicalTheme::unicode()).with_syntax_highlighting(highlighter)
				} else {
					GraphicalReportHandler::new_themed(GraphicalTheme::unicode_nocolor())
				};

				let miette_diag = diagnostic.into_miette();
				let named_source = NamedSource::new(css_file_path, css_source.clone());
				let report = Report::new(miette_diag).with_source_code(named_source);
				let mut output = String::new();
				if handler.render_report(&mut output, &*report).is_ok() {
					eprint!("{}", output);
				}
			}

			for (stat_name, (stat_type, count)) in collector.stats() {
				let entry = aggregated_stats.entry(*stat_name).or_insert((*stat_type, 0));
				entry.1 += count;
			}
		}

		// Output aggregated statistics summary
		if !aggregated_stats.is_empty() {
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

		// Return error if we encountered any error-level diagnostics
		if error_count > 0 { Err(CliError::Checks(error_count)) } else { Ok(()) }
	}
}
