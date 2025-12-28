use crate::{SemanticDecoration, SemanticKind, SemanticModifier, TokenHighlighter};
use anstyle::{AnsiColor, Color, Effects, Style};
use chromashift::{Named, WcagColorContrast};
use css_lexer::{ToSpan, Token};
use css_parse::{SourceCursor, SourceCursorSink};
use std::fmt::{Result, Write};

pub struct AnsiHighlightCursorStream<'h, W: Write, T: AnsiTheme> {
	writer: W,
	theme: T,
	highlighter: &'h TokenHighlighter,
	last_token: Option<Token>,
	err: Result,
}

impl<'h, W: Write, T: AnsiTheme> AnsiHighlightCursorStream<'h, W, T> {
	pub fn new(writer: W, highlighter: &'h TokenHighlighter, theme: T) -> Self {
		Self { writer, highlighter, theme, last_token: None, err: Ok(()) }
	}
}

impl<'a, 'h, W: Write, T: AnsiTheme> SourceCursorSink<'a> for AnsiHighlightCursorStream<'h, W, T> {
	fn append(&mut self, c: SourceCursor<'a>) {
		if self.err.is_err() {
			return;
		}
		if let Some(last) = self.last_token
			&& last.needs_separator_for(c.token())
		{
			self.err = self.writer.write_char(' ');
		}
		self.last_token = Some(c.token());
		if let Some(highlight) = self.highlighter.get(c.to_span()) {
			if let SemanticDecoration::BackgroundColor(bg) = highlight.decoration() {
				let fg = if bg.wcag_contrast_ratio(Named::White) > bg.wcag_contrast_ratio(Named::Black) {
					Named::White
				} else {
					Named::Black
				};
				let color_style = Style::new().bg_color(Some(bg.into())).fg_color(Some(fg.into()));
				self.err = write!(&mut self.writer, "{color_style}{c}{color_style:#}");
			} else {
				let style = self.theme.get_style(highlight.kind(), highlight.modifier());
				self.err = write!(&mut self.writer, "{style}{c}{style:#}");
			}
		} else {
			self.err = write!(&mut self.writer, "{c}");
		}
	}
}

pub trait AnsiTheme {
	fn get_style(&self, kind: SemanticKind, modifier: SemanticModifier) -> Style;
}

pub struct DefaultAnsiTheme;
impl AnsiTheme for DefaultAnsiTheme {
	fn get_style(&self, kind: SemanticKind, modifier: SemanticModifier) -> Style {
		let color = match kind {
			SemanticKind::None => Color::Ansi(AnsiColor::White),
			SemanticKind::Id => Color::Ansi256(214.into()),
			SemanticKind::Tag => Color::Ansi256(203.into()),
			// Bright green
			SemanticKind::Class => Color::Ansi256(149.into()),
			// Salmon/pink
			SemanticKind::Wildcard => Color::Ansi256(203.into()),
			// Bright green
			SemanticKind::Attribute => Color::Ansi256(149.into()),
			// Cyan
			SemanticKind::Namespace => Color::Ansi256(81.into()),
			// White
			SemanticKind::Combinator => Color::Ansi(AnsiColor::White),
			// Bright green
			SemanticKind::PseudoClass => Color::Ansi256(149.into()),
			// Bright green
			SemanticKind::PseudoElement => Color::Ansi256(149.into()),
			// Bright green
			SemanticKind::LegacyPseudoElement => Color::Ansi256(149.into()),
			// Bright green
			SemanticKind::FunctionalPseudoClass => Color::Ansi256(149.into()),
			// Bright green
			SemanticKind::FunctionalPseudoElement => Color::Ansi256(149.into()),

			// Rule Elements
			// Salmon/pink
			SemanticKind::AtKeyword => Color::Ansi256(203.into()),
			// Bright green
			SemanticKind::Prelude => Color::Ansi256(149.into()),

			// Property Declarations
			// Cyan
			SemanticKind::Declaration => Color::Ansi256(81.into()),
			// Cyan
			SemanticKind::StyleValueKeyword => Color::Ansi256(81.into()),
			// Purple
			SemanticKind::StyleValueDimension => Color::Ansi256(141.into()),
			// Purple
			SemanticKind::StyleValueNumber => Color::Ansi256(141.into()),
			// Yellow/Gold for strings
			SemanticKind::StyleValueString => Color::Ansi256(220.into()),
			// Blue for URLs
			SemanticKind::StyleValueUrl => Color::Ansi256(39.into()),
			// Magenta for colors
			SemanticKind::StyleValueColor => Color::Ansi256(201.into()),
			// Cyan for functions
			SemanticKind::StyleValueFunction => Color::Ansi256(51.into()),
			// Red for !important
			SemanticKind::StyleValueImportant => Color::Ansi256(196.into()),
			SemanticKind::Punctuation => Color::Ansi(AnsiColor::White),
		};

		let mut effects = Effects::new();
		if modifier.contains(SemanticModifier::Deprecated) {
			effects |= Effects::STRIKETHROUGH;
		}
		if modifier.contains(SemanticModifier::Experimental) {
			effects |= Effects::UNDERLINE;
		}
		if modifier.contains(SemanticModifier::Unknown) {
			effects |= Effects::CURLY_UNDERLINE;
		}
		Style::new().fg_color(Some(color)).effects(effects)
	}
}
