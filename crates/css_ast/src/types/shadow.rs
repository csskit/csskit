use super::prelude::*;
use crate::{Color, Length};

// https://drafts.csswg.org/css-backgrounds-3/#typedef-shadow
// <shadow> = <color>? && [<length>{2} <length [0,∞]>? <length>?] && inset?
#[derive(ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct Shadow {
	pub color: Option<Color>,
	pub offset: (Length, Length),
	pub blur_radius: Option<Length>,
	pub spread_radius: Option<Length>,
	#[visit(skip)]
	pub inset: Option<T![Ident]>,
}

impl<'a> Peek<'a> for Shadow {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		Color::peek(p, c) || Length::peek(p, c)
	}
}

impl<'a> Parse<'a> for Shadow {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let color = p.parse_if_peek::<Color>()?;

		let x = p.parse::<Length>()?;
		let y = p.parse::<Length>()?;

		let blur_radius = p.parse_if_peek::<Length>()?;
		if let Some(blur) = blur_radius
			&& 0.0f32 > blur.into()
		{
			Err(Diagnostic::new(blur.into(), Diagnostic::number_too_small))?
		}

		let spread_radius = p.parse_if_peek::<Length>()?;

		let inset = p.parse_if_peek::<T![Ident]>()?;
		if let Some(ident) = inset
			&& !p.equals_atom(ident.into(), &CssAtomSet::Inset)
		{
			let c: Cursor = x.into();
			Err(Diagnostic::new(c, Diagnostic::unexpected_ident))?
		}

		Ok(Self { color, offset: (x, y), blur_radius, spread_radius, inset })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use crate::assert_visits;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Shadow>(), 220);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Shadow, "10px 20px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "10px 20px 5px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "10px 20px 5px 3px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "red 10px 20px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "#ff0000 10px 20px 5px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "rgba(255,0,0,0.5)10px 20px 5px 3px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "10px 20px inset");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "10px 20px 5px inset");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "10px 20px 5px 3px inset");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "red 10px 20px inset");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "blue 10px 20px 5px 3px inset");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "-10px -20px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "red -10px -20px 5px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "0 0");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "0 0 0");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "0 0 0 0");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "1em 2em");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "1rem 2rem 0.5rem");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "");
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "10% 20%");
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "10px");
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "red");
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "inset");
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "10px 20px -5px");
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "10px 20px 5px 3px 7px");
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "10px 20px notinset");
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "10px 20px 5px inset 3px");
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "10px 20px 5px 3px inset extra");
	}

	#[test]
	fn test_visits() {
		assert_visits!("10px 20px", Shadow, Length, Length);
		assert_visits!("red 10px 20px", Shadow, Color, Length, Length);
		assert_visits!("10px 20px 5px", Shadow, Length, Length, Length);
		assert_visits!("blue 10px 20px 5px 3px", Shadow, Color, Length, Length, Length, Length);
	}
}
