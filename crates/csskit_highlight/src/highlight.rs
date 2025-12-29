use crate::{AnsiHighlightCursorStream, AnsiTheme, DefaultAnsiTheme, SemanticDecoration, TokenHighlighter};
use css_ast::{CssAtomSet, StyleSheet, Visitable};
use css_lexer::{Lexer, SourceOffset, Span};
use css_parse::{SourceCursor, SourceCursorSink};
use miette::SpanContents;
use miette::highlighters::{Highlighter, HighlighterState};
use owo_colors::Style;

/// Highlight CSS source code with ANSI color codes
///
/// This function is used by the `find` command to highlight matched lines.
pub fn highlight_css(source: &str, stylesheet: &StyleSheet) -> String {
	let mut highlighter = TokenHighlighter::new();
	stylesheet.accept(&mut highlighter);

	let lexer = Lexer::new(&CssAtomSet::ATOMS, source);
	let mut highlighted = String::new();
	let mut stream = AnsiHighlightCursorStream::new(&mut highlighted, &highlighter, DefaultAnsiTheme);

	for cursor in lexer {
		stream.append(SourceCursor::from(cursor, cursor.str_slice(source)));
	}

	highlighted
}

/// Miette highlighter for CSS syntax highlighting
pub struct CssHighlighter {
	source: String,
	token_colors: TokenHighlighter,
}

impl CssHighlighter {
	pub fn new(source: String, stylesheet: &StyleSheet) -> Self {
		let mut token_colors = TokenHighlighter::new();
		stylesheet.accept(&mut token_colors);
		Self { source, token_colors }
	}
}

impl Highlighter for CssHighlighter {
	fn start_highlighter_state<'h>(&'h self, _source: &dyn SpanContents<'_>) -> Box<dyn HighlighterState + 'h> {
		Box::new(CssHighlighterState::new(&self.source, &self.token_colors))
	}
}

struct CssHighlighterState<'a> {
	source: &'a str,
	token_colors: &'a TokenHighlighter,
	current_line: usize,
	theme: DefaultAnsiTheme,
}

impl<'a> CssHighlighterState<'a> {
	fn new(source: &'a str, token_colors: &'a TokenHighlighter) -> Self {
		Self { source, token_colors, current_line: 0, theme: DefaultAnsiTheme }
	}
}

impl<'a> HighlighterState for CssHighlighterState<'a> {
	fn highlight_line<'s>(&mut self, line: &'s str) -> Vec<owo_colors::Styled<&'s str>> {
		let mut result = Vec::new();

		// Find the byte offset of this line in the source
		let line_offset: usize = self.source.lines().take(self.current_line).map(|l| l.len() + 1).sum();
		self.current_line += 1;

		// Lex just this line
		let lexer = Lexer::new(&CssAtomSet::ATOMS, line);
		let mut last_end = 0;

		for cursor in lexer {
			let start = cursor.offset().0 as usize;
			let token_str = cursor.str_slice(line);
			let end = start + token_str.len();

			// Get the color for this token based on its global position
			let global_offset = line_offset + start;
			let span =
				Span::new(SourceOffset(global_offset as u32), SourceOffset((global_offset + token_str.len()) as u32));
			let color = self.token_colors.get(span);

			// Add any whitespace before this token
			if start > last_end {
				result.push(Style::default().style(&line[last_end..start]));
			}

			// Add the colored token - use csskit_highlight color if available
			let token_str = &line[start..end];

			// Check for background color decoration first (for color values)
			let style = if let Some(highlight) = color
				&& let SemanticDecoration::BackgroundColor(bg) = highlight.decoration()
			{
				use chromashift::{Named, Srgb, WcagColorContrast};

				// Choose contrasting foreground color
				let fg = if bg.wcag_contrast_ratio(Named::White) > bg.wcag_contrast_ratio(Named::Black) {
					Named::White
				} else {
					Named::Black
				};

				// Convert to Srgb to get RGB components
				let bg_srgb: Srgb = bg.into();
				let fg_srgb: Srgb = fg.into();

				Style::new().truecolor(fg_srgb.red, fg_srgb.green, fg_srgb.blue).on_truecolor(
					bg_srgb.red,
					bg_srgb.green,
					bg_srgb.blue,
				)
			} else if let Some(highlight) = color {
				// Use the theme to get the owo-colors style
				self.theme.get_owo_style(highlight.kind(), highlight.modifier())
			} else {
				// Fallback: just return unstyled
				Style::default()
			};

			result.push(style.style(token_str));
			last_end = end;
		}

		// Add any remaining text
		if last_end < line.len() {
			result.push(Style::default().style(&line[last_end..]));
		}

		result
	}
}
