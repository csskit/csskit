pub trait AnsiTheme: Sized {
	#[cfg(feature = "anstyle")]
	fn get_anstyle(&self, kind: crate::SemanticKind, modifier: crate::SemanticModifier) -> anstyle::Style;

	#[cfg(feature = "owo-colors")]
	fn get_owo_style(&self, kind: crate::SemanticKind, modifier: crate::SemanticModifier) -> owo_colors::Style;
}

pub struct DefaultAnsiTheme;

impl AnsiTheme for DefaultAnsiTheme {
	#[cfg(feature = "anstyle")]
	fn get_anstyle(&self, kind: crate::SemanticKind, modifier: crate::SemanticModifier) -> anstyle::Style {
		use crate::SemanticKind;
		use anstyle::{AnsiColor, Color, Effects, Style};

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
		if modifier.contains(crate::SemanticModifier::Deprecated) {
			effects |= Effects::STRIKETHROUGH;
		}
		if modifier.contains(crate::SemanticModifier::Experimental) {
			effects |= Effects::UNDERLINE;
		}
		if modifier.contains(crate::SemanticModifier::Unknown) {
			effects |= Effects::CURLY_UNDERLINE;
		}
		Style::new().fg_color(Some(color)).effects(effects)
	}

	#[cfg(feature = "owo-colors")]
	fn get_owo_style(&self, kind: crate::SemanticKind, modifier: crate::SemanticModifier) -> owo_colors::Style {
		use crate::SemanticKind;
		use owo_colors::{Style, XtermColors};

		// Map semantic kind to owo-colors style (matching anstyle colors)
		let mut style = match kind {
			SemanticKind::None => Style::new(),
			SemanticKind::Id => Style::new().color(XtermColors::UserBrightYellow),
			SemanticKind::Tag => Style::new().color(XtermColors::UserBrightMagenta),
			SemanticKind::Class => Style::new().color(XtermColors::UserBrightCyan),
			SemanticKind::Wildcard => Style::new().color(XtermColors::UserBrightMagenta),
			SemanticKind::Attribute => Style::new().color(XtermColors::UserBrightGreen),
			SemanticKind::Namespace => Style::new().color(XtermColors::UserCyan),
			SemanticKind::Combinator => Style::new(),
			SemanticKind::PseudoClass => Style::new().color(XtermColors::UserBrightGreen),
			SemanticKind::PseudoElement => Style::new().color(XtermColors::UserBrightGreen),
			SemanticKind::LegacyPseudoElement => Style::new().color(XtermColors::UserBrightGreen),
			SemanticKind::FunctionalPseudoClass => Style::new().color(XtermColors::UserBrightGreen),
			SemanticKind::FunctionalPseudoElement => Style::new().color(XtermColors::UserBrightGreen),

			// Rule Elements
			SemanticKind::AtKeyword => Style::new().color(XtermColors::UserBrightMagenta),
			SemanticKind::Prelude => Style::new().color(XtermColors::UserBrightGreen),

			// Property Declarations
			SemanticKind::Declaration => Style::new().color(XtermColors::UserCyan),
			SemanticKind::StyleValueKeyword => Style::new().color(XtermColors::UserCyan),
			SemanticKind::StyleValueDimension => Style::new().color(XtermColors::UserMagenta),
			SemanticKind::StyleValueNumber => Style::new().color(XtermColors::UserMagenta),
			SemanticKind::StyleValueString => Style::new().color(XtermColors::UserYellow),
			SemanticKind::StyleValueUrl => Style::new().color(XtermColors::UserBlue),
			SemanticKind::StyleValueColor => Style::new().color(XtermColors::UserBrightMagenta),
			SemanticKind::StyleValueFunction => Style::new().color(XtermColors::UserCyan),
			SemanticKind::StyleValueImportant => Style::new().color(XtermColors::UserRed),
			SemanticKind::Punctuation => Style::new(),
		};

		// Apply effects based on modifiers
		if modifier.contains(crate::SemanticModifier::Deprecated) {
			style = style.strikethrough();
		}
		if modifier.contains(crate::SemanticModifier::Experimental) {
			style = style.underline();
		}
		if modifier.contains(crate::SemanticModifier::Unknown) {
			style = style.underline(); // TODO: owo-colors doesn't have curly underline
		}

		style
	}
}
