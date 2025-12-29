use super::GlobalConfig;
use crate::{CliResult, InputArgs, InputSource, bg, bold, dimmed, fg};
use bumpalo::Bump;
use chromashift::*;
use clap::Args;
use css_ast::{Color as ASTColor, CssAtomSet, StyleSheet, ToChromashift, Visitable};
use css_lexer::Lexer;
use css_parse::{Parser, Span, ToSpan};
use itertools::Itertools;
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
	T: core::fmt::Display + Copy + WcagColorContrast<T> + From<Named>,
	Oklch: From<T>,
	Srgb: From<T>,
{
	if let Some(wcag_color) = color.find_minimum_contrast::<Oklch>(other.into(), level) {
		let ratio = wcag_color.wcag_contrast_ratio(other.into());
		let hex = format!("{}", Hex::from(wcag_color));
		let desc = level.description();

		if conf.colors() {
			print!(" {} ", bg(fg(hex, wcag_color), color));
			println!("  {:.1}:1 {}", ratio, dimmed(format!("({})", desc)));
		} else {
			println!(" {hex:9}   {ratio:.1}:1 ({desc})");
		}
	}
}

fn print_color_block<T>(color: T, config: &GlobalConfig)
where
	T: core::fmt::Display + Copy,
	Srgb: From<T>,
{
	if config.colors() {
		println!(" {}  {color}", bg("          ", color));
	} else {
		println!(" {:10}  {color}", "");
	}
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

	if let Some((file, line, column)) = lc {
		if config.colors() {
			println!(" {}  {} - {file}:{line}:{column}", bg("          ", color), bold(color.to_string()));
		} else {
			println!(" {:10}  {color} - {file}:{line}:{column}", "");
		}
	} else if config.colors() {
		println!(" {}  {}", bg("          ", color), bold(color.to_string()));
	} else {
		println!(" {:10}  {color}", "");
	}
	if config.colors() {
		println!(" {}", bg("          ", color));
	} else {
		println!(" {:10}", "");
	}

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
		if config.colors() {
			println!(" {}", bg("          ", color));
			println!(" {} {}", bg("          ", color), bold("WCAG Contrast Analysis"));
		} else {
			println!(" {:10}", "");
			println!(" {:10} WCAG Contrast Analysis", "");
		}

		let white_ratio = rgb.wcag_contrast_ratio(Named::White);
		let black_ratio = rgb.wcag_contrast_ratio(Named::Black);
		let white_level = rgb.wcag_level(Named::White);
		let black_level = rgb.wcag_level(Named::Black);

		if config.colors() {
			println!(
				" {} vs White    {:.1}:1 {} {}",
				bg(fg(" ", Named::White), color),
				white_ratio,
				dimmed(format!("({})", white_level.description())),
				format_wcag_status(white_level)
			);
			println!(
				" {} vs Black    {:.1}:1 {} {}",
				bg(fg(" ", Named::Black), color),
				black_ratio,
				dimmed(format!("({})", black_level.description())),
				format_wcag_status(black_level)
			);
		} else {
			println!(
				" vs White    {:.1}:1 ({}) {}",
				white_ratio,
				white_level.description(),
				format_wcag_status(white_level)
			);
			println!(
				" vs Black    {:.1}:1 ({}) {}",
				black_ratio,
				black_level.description(),
				format_wcag_status(black_level)
			);
		}

		if white_level != WcagLevel::AA || black_level != WcagLevel::AA {
			if config.colors() {
				println!(" {}", bg("          ", color));
				println!(" {} {}", bg("          ", color), bold("Minimum contrast"));
			} else {
				println!(" {:10}", "");
				println!(" {:10} Minimum contrast", "");
			}

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
			if config.colors() {
				println!(" {}", bg("          ", color));
				println!(" {} {}", bg("          ", color), bold("Named color"));
				println!(" {} {}", bg("          ", color), bold(colors.first().unwrap().to_string()));
			} else {
				println!(" {:10}", "");
				println!(" {:10} Named color", "");
				println!(" {:10} {}", "", colors.first().unwrap());
			}
		} else if !colors.is_empty() {
			if config.colors() {
				println!(" {}", bg("          ", color));
				println!(" {} {}", bg("          ", color), bold("Similar named colors"));
			} else {
				println!(" {:10}", "");
				println!(" {:10} Similar named colors", "");
			}
			for similar_color in colors {
				if config.colors() {
					println!(" {} {} {similar_color}", bg("          ", color), bg("          ", similar_color));
				} else {
					println!(" {:10} {:10} {similar_color}", "", "");
				}
			}
		}
	}

	if config.colors() {
		println!(" {}", bg("          ", color));
	} else {
		println!(" {:10}", "");
	}
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
			let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
			let mut parser = Parser::new(&bump, source_text, lexer);
			let result = parser.parse_entirely::<bumpalo::collections::Vec<ASTColor>>();
			if result.output.is_some() && result.errors.is_empty() {
				result.output.unwrap().accept(&mut color_visitor);
			} else {
				let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
				let mut parser = Parser::new(&bump, source_text, lexer);
				let result = parser.parse_entirely::<StyleSheet>();
				if let Some(stylesheet) = result.output {
					stylesheet.accept(&mut color_visitor);
				} else {
					for compact_err in result.errors {
						let report = crate::commands::format_diagnostic_error(&compact_err, &source_string, file_name);
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
