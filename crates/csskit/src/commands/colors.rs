use super::GlobalConfig;
use crate::{
	CliResult, InputArgs, bg, bold,
	commands::{Extract, OutputFormat, extract::ErrorRecord},
	dimmed, fg,
};
use allocator_api2::alloc::Allocator;
use bumpalo::Bump;
use chromashift::*;
use clap::Args;
use css_ast::{Color as ASTColor, CssAtomSet, StyleSheet, ToChromashift, Visitable};
use css_lexer::{Lexer, LineIndex};
use css_parse::{Diagnostic, Parser, Span, ToSpan};
use itertools::Itertools;
use serde::Serialize;
use std::collections::HashSet;

fn extract_colors(src: &str, bump: &Bump) -> Result<Vec<(Color, Span)>, Vec<Diagnostic>> {
	let mut visitor = ColorExtractor::new();

	// Try parsing as a bare list of colors first.
	let lexer = Lexer::new(&CssAtomSet::ATOMS, src);
	let mut parser = Parser::new(bump, src, lexer);
	let result = parser.parse_entirely::<css_parse::Vec<ASTColor>>();
	if let Some(output) = result.output.filter(|_| result.errors.is_empty()) {
		let _ = output.accept(&mut visitor);
		return Ok(visitor.colors);
	}

	// Fall back to full stylesheet parse.
	let lexer = Lexer::new(&CssAtomSet::ATOMS, src);
	let mut parser = Parser::new(bump, src, lexer);
	let result = parser.parse_entirely::<StyleSheet>();
	if let Some(stylesheet) = result.output {
		let _ = stylesheet.accept(&mut visitor);
		Ok(visitor.colors)
	} else {
		Err(result.errors.to_vec())
	}
}

struct ColorExtractor {
	colors: Vec<(Color, Span)>,
	seen: HashSet<Hex>,
}

impl ColorExtractor {
	fn new() -> Self {
		Self { colors: Vec::new(), seen: HashSet::new() }
	}
}

#[css_ast::visitor]
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

fn suggest_wcag_variant<T>(color: T, other: Named, level: WcagLevel, colors: bool)
where
	T: core::fmt::Display + Copy + WcagColorContrast<T> + From<Named>,
	Oklch: From<T>,
	Srgb: From<T>,
{
	if let Some(wcag_color) = color.find_minimum_contrast::<Oklch>(other.into(), level) {
		let ratio = wcag_color.wcag_contrast_ratio(other.into());
		let hex = format!("{}", Hex::from(wcag_color));
		let desc = level.description();

		if colors {
			print!(" {} ", bg(fg(hex, wcag_color), color));
			println!("  {:.1}:1 {}", ratio, dimmed(format!("({})", desc)));
		} else {
			println!(" {hex:9}   {ratio:.1}:1 ({desc})");
		}
	}
}

fn print_color_block<T>(color: T, colors: bool)
where
	T: core::fmt::Display + Copy,
	Srgb: From<T>,
{
	if colors {
		println!(" {}  {color}", bg("          ", color));
	} else {
		println!(" {:10}  {color}", "");
	}
}

fn print_color_info(color: Color, colors: bool, all: bool, wcag: bool, named: bool, lc: Option<(&'_ str, u32, u32)>) {
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
		if colors {
			println!(" {}  {} - {file}:{line}:{column}", bg("          ", color), bold(color.to_string()));
		} else {
			println!(" {:10}  {color} - {file}:{line}:{column}", "");
		}
	} else if colors {
		println!(" {}  {}", bg("          ", color), bold(color.to_string()));
	} else {
		println!(" {:10}  {color}", "");
	}
	if colors {
		println!(" {}", bg("          ", color));
	} else {
		println!(" {:10}", "");
	}

	if !matches!(color, Color::Hex(_)) {
		print_color_block(hex, colors);
	}
	if !matches!(color, Color::Srgb(_)) {
		print_color_block(rgb, colors);
	}
	if !matches!(color, Color::Oklab(_)) {
		print_color_block(oklab, colors);
	}
	if !matches!(color, Color::Oklch(_)) {
		print_color_block(oklch, colors);
	}
	if all {
		if !matches!(color, Color::A98Rgb(_)) {
			print_color_block(a98, colors);
		}
		if !matches!(color, Color::Hsv(_)) {
			print_color_block(hsv, colors);
		}
		if !matches!(color, Color::Hsl(_)) {
			print_color_block(hsl, colors);
		}
		if !matches!(color, Color::Hwb(_)) {
			print_color_block(hwb, colors);
		}
		if !matches!(color, Color::Lab(_)) {
			print_color_block(lab, colors);
		}
		if !matches!(color, Color::Lch(_)) {
			print_color_block(lch, colors);
		}
		if !matches!(color, Color::LinearRgb(_)) {
			print_color_block(linear, colors);
		}
		if !matches!(color, Color::XyzD50(_)) {
			print_color_block(d50, colors);
		}
		if !matches!(color, Color::XyzD65(_)) {
			print_color_block(d65, colors);
		}
	}

	// Show WCAG contrast information
	if wcag {
		if colors {
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

		if colors {
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
			if colors {
				println!(" {}", bg("          ", color));
				println!(" {} {}", bg("          ", color), bold("Minimum contrast"));
			} else {
				println!(" {:10}", "");
				println!(" {:10} Minimum contrast", "");
			}

			suggest_wcag_variant(rgb, Named::White, WcagLevel::AA, colors);
			suggest_wcag_variant(rgb, Named::White, WcagLevel::AAA, colors);
			suggest_wcag_variant(rgb, Named::Black, WcagLevel::AA, colors);
			suggest_wcag_variant(rgb, Named::Black, WcagLevel::AAA, colors);
		}
	}

	if named && !matches!(color, Color::Named(_)) {
		let named_colors: Vec<Named> = Named::iter()
			.filter(|named| named.close_to(color, 10.0))
			.sorted_by(|a, b| {
				((a.delta_e(color) * 1000.0).round() as u64).cmp(&((b.delta_e(color) * 1000.0).round() as u64))
			})
			.take(2)
			.collect::<Vec<Named>>();
		// We have one (near enough) identical colour...
		if named_colors.first().is_some_and(|n| n.close_to(color, COLOR_EPSILON)) {
			if colors {
				println!(" {}", bg("          ", color));
				println!(" {} {}", bg("          ", color), bold("Named color"));
				println!(" {} {}", bg("          ", color), bold(named_colors.first().unwrap().to_string()));
			} else {
				println!(" {:10}", "");
				println!(" {:10} Named color", "");
				println!(" {:10} {}", "", named_colors.first().unwrap());
			}
		} else if !named_colors.is_empty() {
			if colors {
				println!(" {}", bg("          ", color));
				println!(" {} {}", bg("          ", color), bold("Similar named colors"));
			} else {
				println!(" {:10}", "");
				println!(" {:10} Similar named colors", "");
			}
			for similar_color in named_colors {
				if colors {
					println!(" {} {} {similar_color}", bg("          ", color), bg("          ", similar_color));
				} else {
					println!(" {:10} {:10} {similar_color}", "", "");
				}
			}
		}
	}

	if colors {
		println!(" {}", bg("          ", color));
	} else {
		println!(" {:10}", "");
	}
}

/// JSON data payload for a single extracted colour.
#[derive(Serialize)]
pub struct ColorData {
	source: String,
	hex: String,
	srgb: String,
	hsl: String,
	oklch: String,
	oklab: String,
	/// The resolved colour value for text rendering; not serialised.
	#[serde(skip)]
	color: Color,
}

/// Per-file context for colour rendering (unused; location derived from filename).
#[derive(Default)]
pub struct ColorContext;

/// Extract the colours from a CSS file.
#[derive(Debug, Args)]
pub struct ColorCommand {
	#[command(flatten)]
	input: InputArgs,

	/// Print every known syntax for each colour
	#[arg(short, long, value_parser)]
	all: bool,

	/// Print WCAG contrast analysis for each colour
	#[arg(long, value_parser)]
	wcag: bool,

	/// Print similar Named colours for each colour
	#[arg(long, value_parser)]
	named: bool,

	/// Output format
	#[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
	format: OutputFormat,
}

impl ColorCommand {
	pub fn run(&self, config: GlobalConfig) -> CliResult {
		Extract::run(self, config)
	}
}

impl Extract for ColorCommand {
	type Row = ColorData;
	type FileContext = ColorContext;

	fn input(&self) -> &InputArgs {
		&self.input
	}

	fn format(&self) -> OutputFormat {
		self.format
	}

	fn show_file_header(&self) -> bool {
		false
	}

	fn on_no_results(&self, file: &str) {
		eprintln!("No colors found in {file}");
	}

	fn render_file_preamble(&self, _file: &str, row_count: usize, _color: bool) {
		println!();
		eprintln!("Found {row_count} color{}", if row_count == 1 { "" } else { "s" });
		println!();
	}

	fn parse_and_extract_file(
		&self,
		file: &str,
		src: &str,
		bump: &Bump,
		_on_stylesheet: &mut dyn for<'a> FnMut(&StyleSheet<'a>),
	) -> Result<Vec<(Span, ColorData)>, ErrorRecord> {
		use crate::commands::extract::ErrorKind;
		match extract_colors(src, bump) {
			Ok(colors) => {
				let mut seen: HashSet<Hex> = HashSet::new();
				Ok(colors
					.into_iter()
					.filter_map(|(color, span)| {
						let hex = Hex::from(color);
						if seen.insert(hex) {
							let start = usize::from(span.start());
							let end = usize::from(span.end());
							Some((
								span,
								ColorData {
									source: src[start..end].to_string(),
									hex: hex.to_string(),
									srgb: Srgb::from(color).to_string(),
									hsl: Hsl::from(color).to_string(),
									oklch: Oklch::from(color).to_string(),
									oklab: Oklab::from(color).to_string(),
									color,
								},
							))
						} else {
							None
						}
					})
					.collect())
			}
			Err(errors) => {
				let message = errors
					.first()
					.map(|e| {
						eprintln!("{}", crate::commands::format_diagnostic_error(e, src, file));
						e.message(src).to_string()
					})
					.unwrap_or_else(|| "parse failed".to_string());
				Err(ErrorRecord { kind: ErrorKind::ParseError, message })
			}
		}
	}

	fn extract<'a>(&self, _stylesheet: &StyleSheet<'a>, _src: &str, _out: &mut Vec<(Span, ColorData)>) {
		// Unreachable: parse_and_extract_file is fully overridden.
	}

	fn render_text<A: Allocator>(
		&self,
		_ctx: &ColorContext,
		file: &str,
		_src: &str,
		index: &LineIndex<'_, A>,
		span: Span,
		row: &ColorData,
		color: bool,
	) {
		let lc = if file != "<content>" && file != "-" {
			let (line, col) = index.line_and_column(span);
			Some((file, line, col))
		} else {
			None
		};
		let wcag = self.wcag || self.all;
		let named = self.named || self.all;
		print_color_info(row.color, color, self.all, wcag, named, lc);
		println!();
	}
}
