#![deny(warnings)]
use bitmask_enum::bitmask;
use chromashift::Hex;
use core::fmt;
use css_lexer::Span;
use std::collections::HashMap;
use strum::{Display, VariantNames};

mod css;

#[cfg(any(feature = "anstyle", feature = "owo-colors"))]
mod default_ansi_theme;
#[cfg(any(feature = "anstyle", feature = "owo-colors"))]
pub use default_ansi_theme::{AnsiTheme, DefaultAnsiTheme};

#[cfg(any(feature = "anstyle", feature = "owo-colors"))]
mod ansi_highlight_cursor_stream;
#[cfg(any(feature = "anstyle", feature = "owo-colors"))]
pub use ansi_highlight_cursor_stream::AnsiHighlightCursorStream;

#[cfg(feature = "miette")]
mod highlight;
#[cfg(feature = "miette")]
pub use highlight::CssHighlighter;

#[cfg(test)]
mod test_helpers;
#[cfg(test)]
mod tests;

// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#semanticTokenTypes
#[derive(Display, VariantNames, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SemanticKind {
	None,

	/* Selector Elements */
	Id,
	Tag,
	Class,
	Wildcard,
	Attribute,
	Namespace,
	Combinator,
	PseudoClass,
	PseudoElement,
	LegacyPseudoElement,
	FunctionalPseudoClass,
	FunctionalPseudoElement,

	/* Rule Elements */
	AtKeyword,
	Prelude,

	/* Property Declarations */
	Declaration,
	StyleValueKeyword,
	StyleValueDimension,
	StyleValueNumber,
	StyleValueString,
	StyleValueUrl,
	StyleValueColor,
	StyleValueFunction,
	StyleValueImportant,

	Punctuation,
}

impl SemanticKind {
	pub fn bits(&self) -> u8 {
		*self as u8
	}
}

// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#semanticTokenModifiers
#[derive(VariantNames)]
#[bitmask(u8)]
pub enum SemanticModifier {
	Unknown,
	Deprecated,
	Experimental,
	Vendor,
	Custom,
}

impl fmt::Display for SemanticModifier {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		if self.contains(Self::Unknown) {
			write!(f, " unknown")?;
		}
		if self.contains(Self::Deprecated) {
			write!(f, " deprecated")?;
		}
		if self.contains(Self::Experimental) {
			write!(f, " experimental")?;
		}
		if self.contains(Self::Experimental) {
			write!(f, " vendor")?;
		}
		if self.contains(Self::Custom) {
			write!(f, " custom")?;
		}
		Ok(())
	}
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SemanticDecoration {
	None,
	BackgroundColor(Hex),
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Highlight {
	kind: SemanticKind,
	modifier: SemanticModifier,
	decoration: SemanticDecoration,
	span: Span,
}

impl Highlight {
	#[inline(always)]
	pub fn span(&self) -> Span {
		self.span
	}

	#[inline(always)]
	pub fn modifier(&self) -> SemanticModifier {
		self.modifier
	}

	#[inline(always)]
	pub fn kind(&self) -> SemanticKind {
		self.kind
	}

	#[inline(always)]
	pub fn decoration(&self) -> SemanticDecoration {
		self.decoration
	}
}

#[derive(Default)]
pub struct TokenHighlighter {
	highlights: HashMap<Span, Highlight>,
}

impl TokenHighlighter {
	pub fn new() -> Self {
		Self { highlights: HashMap::new() }
	}

	pub fn get(&self, span: Span) -> Option<&Highlight> {
		self.highlights.get(&span)
	}

	pub fn highlights(&self) -> impl Iterator<Item = &Highlight> {
		self.highlights.values()
	}

	fn insert(&mut self, span: Span, kind: SemanticKind, modifier: SemanticModifier) {
		self.insert_with_decoration(span, kind, modifier, SemanticDecoration::None);
	}

	fn insert_with_decoration(
		&mut self,
		span: Span,
		kind: SemanticKind,
		modifier: SemanticModifier,
		decoration: SemanticDecoration,
	) {
		self.highlights.insert(span, Highlight { span, kind, modifier, decoration });
	}
}
