use super::GlobalConfig;
use crate::{CliResult, InputArgs, InputSource};
use anstyle::Style;
use bumpalo::Bump;
use chromashift::*;
use clap::Args;
use css_ast::{Color as ASTColor, StyleSheet, ToChromashift, Visitable};
use css_parse::{Parser, Span, ToSpan};
use itertools::Itertools;
use miette::{GraphicalReportHandler, GraphicalTheme, NamedSource};
use std::{collections::HashSet, io::Read};

struct ColorExtractor {
	colors: Vec<(Color, Span)>,
	seen: HashSet<Hex>,
}

impl ColorExtractor {
	fn new() -> Self {
		Self { colors: Vec::new(), seen: HashSet::new() }
	}
}

impl css_ast::Visit for ColorExtractor {
	fn visit_color(&mut self, color: &ASTColor) {
		if let Some(raw_color) = color.to_chromashift() {
			let hex = raw_color.into();
			if !self.seen.contains(&hex) {
				self.seen.insert(hex);
				self.colors.push((raw_color, color.to_span()));
			}
		}
	}
}

fn format_wcag_status(level: WcagLevel) -> &'static str {
	match level {
		WcagLevel::Fail => "❌",
		WcagLevel::AALarge => "⚠️ ",
		WcagLevel::AA => "✅",
		WcagLevel::AAA => "🌟",
	}
}

fn suggest_wcag_variant<T>(color: T, other: Named, level: WcagLevel, conf: &GlobalConfig)
where
	anstyle::RgbColor: From<T>,
	anstyle::Color: From<T>,
	T: core::fmt::Display + Copy + WcagColorContrast<T> + From<Named>,
	Oklch: From<T>,
	Srgb: From<T>,
{
	if let Some(wcag_color) = color.find_minimum_contrast::<Oklch>(other.into(), level) {
		let style = if conf.colors() {
			Style::new().fg_color(Some(wcag_color.into())).bg_color(Some(color.into()))
		} else {
			Style::new()
		};
		let dim = if conf.colors() { Style::new().dimmed() } else { Style::new() };
		let ratio = wcag_color.wcag_contrast_ratio(other.into());
		let hex = format!("{}", Hex::from(wcag_color));
		let desc = level.description();
		println!(" {style} {hex:9}{style:#}   {ratio:.1}:1 {dim}({desc}){dim:#}");
	}
}

fn print_color_block<T>(color: T, config: &GlobalConfig)
where
	anstyle::RgbColor: From<T>,
	anstyle::Color: From<T>,
	T: core::fmt::Display + Copy,
{
	let bg = if config.colors() { Style::new().bg_color(Some(color.into())) } else { Style::new() };
	println!(" {bg}{:10}{bg:#}  {color}", "");
}

fn print_color_info(
	color: Color,
	config: &GlobalConfig,
	all: bool,
	wcag: bool,
	named: bool,
	lc: Option<(&'_ str, u32, u32)>,
) {
	let a98 = A98Rgb::from(color);
	let hex = Hex::from(color);
	let hsv = Hsv::from(color);
	let hsl = Hsl::from(color);
	let hwb = Hwb::from(color);
	let lab = Lab::from(color);
	let lch = Lch::from(color);
	let linear = LinearRgb::from(color);
	let oklab = Oklab::from(color);
	let oklch = Oklch::from(color);
	let rgb = Srgb::from(color);
	let d50 = XyzD50::from(color);
	let d65 = XyzD65::from(color);

	// let a: Named = n.into();

	let (bg, bold, dim) = if config.colors() {
		(Style::new().bg_color(Some(color.into())), Style::new().bold(), Style::new().dimmed())
	} else {
		(Style::new(), Style::new(), Style::new())
	};

	if let Some((file, line, column)) = lc {
		println!(" {bg}{:10}{bg:#}  {bold}{color}{bold} - {file}:{line}:{column}", "");
	} else {
		println!(" {bg}{:10}{bg:#}  {bold}{color}{bold}", "");
	}
	println!(" {bg}{:10}{bg:#}", "");

	if !matches!(color, Color::Hex(_)) {
		print_color_block(hex, config);
	}
	if !matches!(color, Color::Srgb(_)) {
		print_color_block(rgb, config);
	}
	if !matches!(color, Color::Oklab(_)) {
		print_color_block(oklab, config);
	}
	if !matches!(color, Color::Oklch(_)) {
		print_color_block(oklch, config);
	}
	if all {
		if !matches!(color, Color::A98Rgb(_)) {
			print_color_block(a98, config);
		}
		if !matches!(color, Color::Hsv(_)) {
			print_color_block(hsv, config);
		}
		if !matches!(color, Color::Hsl(_)) {
			print_color_block(hsl, config);
		}
		if !matches!(color, Color::Hwb(_)) {
			print_color_block(hwb, config);
		}
		if !matches!(color, Color::Lab(_)) {
			print_color_block(lab, config);
		}
		if !matches!(color, Color::Lch(_)) {
			print_color_block(lch, config);
		}
		if !matches!(color, Color::LinearRgb(_)) {
			print_color_block(linear, config);
		}
		if !matches!(color, Color::XyzD50(_)) {
			print_color_block(d50, config);
		}
		if !matches!(color, Color::XyzD65(_)) {
			print_color_block(d65, config);
		}
	}

	// Show WCAG contrast information
	if wcag {
		println!(" {bg}{:10}{bg:#}", "");
		println!(" {bg}{:10}{bg:#} {bold}WCAG Contrast Analysis{bold:#}", "");

		let white_ratio = rgb.wcag_contrast_ratio(Named::White);
		let black_ratio = rgb.wcag_contrast_ratio(Named::Black);
		let white_level = rgb.wcag_level(Named::White);
		let black_level = rgb.wcag_level(Named::Black);

		let (wcag_white, wcag_black) = if config.colors() {
			(
				Style::new().fg_color(Some(Named::White.into())).bg_color(Some(color.into())),
				Style::new().fg_color(Some(Named::Black.into())).bg_color(Some(color.into())),
			)
		} else {
			(Style::new(), Style::new())
		};

		println!(
			" {wcag_white} vs White {wcag_white:#}   {:.1}:1 {dim}({}){dim:#} {}",
			white_ratio,
			white_level.description(),
			format_wcag_status(white_level)
		);
		println!(
			" {wcag_black} vs Black {wcag_black:#}   {:.1}:1 {dim}({}){dim:#} {}",
			black_ratio,
			black_level.description(),
			format_wcag_status(black_level)
		);

		if white_level != WcagLevel::AA || black_level != WcagLevel::AA {
			println!(" {bg}{:10}{bg:#}", "");
			println!(" {bg}{:10}{bg:#} {bold}Minimum contrast{bold:#}", "");

			suggest_wcag_variant(rgb, Named::White, WcagLevel::AA, config);
			suggest_wcag_variant(rgb, Named::White, WcagLevel::AAA, config);
			suggest_wcag_variant(rgb, Named::Black, WcagLevel::AA, config);
			suggest_wcag_variant(rgb, Named::Black, WcagLevel::AAA, config);
		}
	}

	if named && !matches!(color, Color::Named(_)) {
		let colors: Vec<Named> = Named::iter()
			.filter(|named| named.close_to(color, 10.0))
			.sorted_by(|a, b| {
				((a.delta_e(color) * 1000.0).round() as u64).cmp(&((b.delta_e(color) * 1000.0).round() as u64))
			})
			.take(2)
			.collect::<Vec<Named>>();
		// We have one (near enough) identical colour...
		if colors.first().is_some_and(|named| named.close_to(color, COLOR_EPSILON)) {
			println!(" {bg}{:10}{bg:#}", "");
			println!(" {bg}{:10}{bg:#} {bold}Named color{bold:#}", "");
			println!(" {bg}{:10}{bg:#} {bold}{}{bold:#}", "", colors.first().unwrap());
		} else if !colors.is_empty() {
			println!(" {bg}{:10}{bg:#}", "");
			println!(" {bg}{:10}{bg:#} {bold}Similar named colors{bold:#}", "");
			for similar_color in colors {
				let similar_bg =
					if config.colors() { Style::new().bg_color(Some(similar_color.into())) } else { Style::new() };
				println!(" {bg}{:10}{bg:#} {similar_bg}{:10}{similar_bg:#} {similar_color}", "", "");
			}
		}
	}

	println!(" {bg}{:10}{bg:#}", "");
}

/// Extract the colours from a CSS file.
#[derive(Debug, Args)]
pub struct ColorCommand {
	#[command(flatten)]
	content: InputArgs,

	/// Print every known syntax for each colour
	#[arg(short, long, value_parser)]
	all: bool,

	/// Print WCAG contrast analysis for each colour
	#[arg(long, value_parser)]
	wcag: bool,

	/// Print similar Named colours for each colour
	#[arg(long, value_parser)]
	named: bool,
}

impl ColorCommand {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		let bump = Bump::default();
		let ColorCommand { content, all, wcag, named } = self;
		let wcag = *wcag || *all;
		let named = *named || *all;
		for (file_name, mut source) in content.sources()? {
			let mut source_string = String::new();
			source.read_to_string(&mut source_string)?;
			let source_text = source_string.as_str();
			let mut color_visitor = ColorExtractor::new();
			let mut parser = Parser::new(&bump, source_text);
			let result = parser.parse_entirely::<bumpalo::collections::Vec<ASTColor>>();
			if result.output.is_some() && result.errors.is_empty() {
				result.output.unwrap().accept(&mut color_visitor);
			} else {
				let mut parser = Parser::new(&bump, source_text);
				let result = parser.parse_entirely::<StyleSheet>();
				if let Some(stylesheet) = result.output {
					stylesheet.accept(&mut color_visitor);
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
			if color_visitor.colors.is_empty() {
				eprintln!("No colors found in {file_name}");
			} else {
				let i = color_visitor.colors.len();
				println!();
				eprintln!("Found {i} color{}", if i > 0 { "s" } else { "" });
				println!();
				for (color, span) in color_visitor.colors {
					let lc = if matches!(source, InputSource::File(_)) {
						let (line, col) = span.line_and_column(source_text);
						Some((file_name, line, col))
					} else {
						None
					};
					print_color_info(color, &config, *all, wcag, named, lc);
					println!();
				}
				println!();
			}
		}
		Ok(())
	}
}
