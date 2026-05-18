use super::prelude::*;
use crate::{Color, Length, NonNegative};

/// <https://drafts.csswg.org/css-backgrounds-3/#typedef-shadow>
///
/// ```text,ignore
/// <shadow> = <color>? && [<length>{2} <length [0,∞]>? <length>?] && inset?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[parse(all_must_occur)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Shadow<'a> {
	pub color: Option<Color<'a>>,
	pub offset: (Length, Length, Option<NonNegative<Length>>, Option<Length>),
	#[atom(CssAtomSet::Inset)]
	pub inset: Option<T![Ident]>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Shadow<'_>>(), 104);
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
	fn test_inset_leading() {
		assert_parse!(CssAtomSet::ATOMS, Shadow, "inset 10px 20px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "inset 0 1px 1px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "inset 0 1px 1px rgba(0,0,0,.075)");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "inset red 10px 20px");
		assert_parse!(CssAtomSet::ATOMS, Shadow, "red inset 10px 20px");
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
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "10px 20px 5px inset 3px");
		assert_parse_error!(CssAtomSet::ATOMS, Shadow, "10px 20px 5px 3px inset extra");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("10px 20px", Shadow, Length, Length);
		assert_visits!("red 10px 20px", Shadow, Color, Length, Length);
		assert_visits!("10px 20px 5px", Shadow, Length, Length, Length);
		assert_visits!("blue 10px 20px 5px 3px", Shadow, Color, Length, Length, Length, Length);
	}
}
