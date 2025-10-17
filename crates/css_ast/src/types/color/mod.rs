mod named;
mod system;

use crate::{ColorFunction, CssAtomSet};
use css_parse::T;
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

pub use named::*;
pub use system::*;

#[derive(Peek, Parse, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum Color {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Currentcolor)]
	Currentcolor(T![Ident]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Transparent)]
	Transparent(T![Ident]),
	System(SystemColor),
	#[cfg_attr(feature = "visitable", visit(skip))]
	Hex(T![Hash]),
	#[cfg_attr(feature = "visitable", visit(skip))]
	Named(NamedColor),
	Function(ColorFunction),
	// TODO: need bumpalo::Box PartialEq, or bumpalo::Box serde
	// Relative(Box<'a, Color<'a>>, ColorFunction),
	// Mix(ColorMixSyntax, Box<'a, Color<'a>>, u8, Box<'a, Color<'a>>),
}

impl Color {
	// Alias CanvasText for #[initial()]
	// #[allow(non_upper_case_globals)]
	// pub const Canvastext: Color = Color::System(SystemColor::CanvasText);
}

#[cfg(feature = "chromashift")]
pub trait ToChromashift {
	fn to_chromashift(&self) -> Option<chromashift::Color>;
}

#[cfg(feature = "chromashift")]
impl ToChromashift for T![Hash] {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::{Color, Hex};
		use css_parse::Token;
		Some(Color::Hex(Hex::new(Token::from(*self).hex_value())))
	}
}

#[cfg(feature = "chromashift")]
impl ToChromashift for Color {
	fn to_chromashift(&self) -> Option<chromashift::Color> {
		use chromashift::Srgb;

		match self {
			Color::Named(named) => named.to_chromashift(),
			Color::Hex(hex) => hex.to_chromashift(),

			// Transparent is black with 0 alpha
			Color::Transparent(_) => Some(chromashift::Color::Srgb(Srgb::new(0, 0, 0, 0.0))),

			// CurrentColor and System colors don't have fixed values
			// They depend on context/system settings
			Color::Currentcolor(_) => None,
			Color::System(_) => None,

			// Color functions - convert based on function type
			Color::Function(func) => func.to_chromashift(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Color>(), 140);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Color, "currentcolor");
		assert_parse!(CssAtomSet::ATOMS, Color, "#fff");
		assert_parse!(CssAtomSet::ATOMS, Color, "red");
		assert_parse!(CssAtomSet::ATOMS, Color, "#ababab");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255 255 255)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255,255,255)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgba(255,255,255,0.5)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(29 164 192/95%)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255 255 255/0.5)");
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255 20% 12.2/0.5)");
		assert_parse!(CssAtomSet::ATOMS, Color, "lab(63.673% 51.577 5.811)");
		assert_parse!(CssAtomSet::ATOMS, Color, "lab(63.673% 51.577 5.811)");
		assert_parse!(CssAtomSet::ATOMS, Color, "hwb(740deg 20% 30%/50%)");
		assert_parse!(CssAtomSet::ATOMS, Color, "lch(20% 30% 740deg/50%)");
	}

	#[test]
	fn test_recoverable_writes() {
		// Missing /
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255 255 255 0.5)");
		// Mixed legacy values
		assert_parse!(CssAtomSet::ATOMS, Color, "rgba(255,20%,255,0.5)");
		// Trailing comma
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255,255,255,)");
		// Using legacy comma syntax but with /
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(255,255,255/0.5)");
		// Missing a comma
		assert_parse!(CssAtomSet::ATOMS, Color, "rgb(29,164 192,95%)");
	}

	#[test]
	fn test_errors() {
		// Using degrees for RGB
		assert_parse_error!(CssAtomSet::ATOMS, Color, "rgba(250deg, 255, 255 / 0.5)");
		// Using % for first component in hsl
		assert_parse_error!(CssAtomSet::ATOMS, Color, "hsl(250%, 255, 255)");
		// Using % for first component in lch
		assert_parse_error!(CssAtomSet::ATOMS, Color, "lch(250%, 255, 255)");
		// Using degrees for wrong component in hsl
		assert_parse_error!(CssAtomSet::ATOMS, Color, "hsl(250, 255deg, 255)");
		// Using degrees for wrong component in lch
		assert_parse_error!(CssAtomSet::ATOMS, Color, "lch(250, 255deg, 255)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("#fff", Color);
		assert_visits!("black", Color);
		assert_visits!("rgb(255 255 255)", Color, ColorFunction);
		assert_visits!("rgba(255,255,255,0.5)", Color, ColorFunction);
		assert_visits!("lab(63.673% 51.577 5.811)", Color, ColorFunction);
		assert_visits!("hwb(740deg 20% 30%/50%)", Color, ColorFunction);
	}

	#[test]
	#[cfg(feature = "chromashift")]
	fn test_chromashift() {
		use super::ToChromashift;
		use crate::CssAtomSet;
		use bumpalo::Bump;
		use chromashift::{Hex, Named, Srgb};
		use css_lexer::Lexer;
		use css_parse::Parser;
		let bump = Bump::default();

		let source_text = "red";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&bump, source_text, lexer);
		let color = p.parse_entirely::<Color>().output.unwrap().to_chromashift();
		assert_eq!(color, Some(chromashift::Color::Named(Named::Red)));

		let source_text = "#f00";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&bump, source_text, lexer);
		let color = p.parse_entirely::<Color>().output.unwrap().to_chromashift();
		assert_eq!(color, Some(chromashift::Color::Hex(Hex::new(0xFF0000FF))));

		let source_text = "rgb(255, 0, 0)";
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source_text);
		let mut p = Parser::new(&bump, source_text, lexer);
		let color = p.parse_entirely::<Color>().output.unwrap().to_chromashift();
		assert_eq!(color, Some(chromashift::Color::Srgb(Srgb::new(255, 0, 0, 100.0))));
	}
}
