use super::prelude::*;
use crate::{Color, Length, NonNegative, NoneOr};

/// <https://drafts.csswg.org/css-borders-4/#typedef-spread-shadow>
///
/// ```text,ignore
/// <spread-shadow> = <'box-shadow-color'>? && [ [ none | <length>{2} ] [ <'box-shadow-blur'> <'box-shadow-spread'>? ]? ] && <'box-shadow-position'>?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[parse(all_must_occur)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SpreadShadow<'a> {
	pub color: Option<Color<'a>>,
	pub offset: NoneOr<(Length, Length)>,
	pub blur: Option<NonNegative<Length>>,
	pub spread: Option<Length>,
	pub position: Option<ShadowPosition>,
}

/// The position of a shadow: `inset` or `outset`.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ShadowPosition {
	#[atom(CssAtomSet::Inset)]
	Inset(T![Ident]),
	#[atom(CssAtomSet::Outset)]
	Outset(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SpreadShadow>(), 104);
	}

	#[test]
	fn test_offset_none() {
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "none");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "none 5px");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "none 5px 3px");
	}

	#[test]
	fn test_lengths() {
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "0 0");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "0 0 0");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "0 0 0 0");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "10px 20px");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "10px 20px 5px");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "10px 20px 5px 3px");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "red 10px 20px");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "10px 20px inset");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "10px 20px outset");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "10px 20px 5px inset");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "10px 20px 5px 3px inset");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "red 10px 20px inset");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "0 0 0 .2rem rgba(0,123,255,.25)");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "0 1px 1px rgba(0,0,0,.075)inset");
	}

	#[test]
	fn test_leading_position() {
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "inset 10px 20px");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "inset 0 1px 1px rgba(0,0,0,.075)");
		assert_parse!(CssAtomSet::ATOMS, SpreadShadow, "outset 10px 20px");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, SpreadShadow, "");
		assert_parse_error!(CssAtomSet::ATOMS, SpreadShadow, "10px");
		assert_parse_error!(CssAtomSet::ATOMS, SpreadShadow, "red");
		assert_parse_error!(CssAtomSet::ATOMS, SpreadShadow, "inset");
		assert_parse_error!(CssAtomSet::ATOMS, SpreadShadow, "outset");
		assert_parse_error!(CssAtomSet::ATOMS, SpreadShadow, "10px 20px -5px");
		assert_parse_error!(CssAtomSet::ATOMS, SpreadShadow, "10px 20px 5px 3px 7px");
	}
}
