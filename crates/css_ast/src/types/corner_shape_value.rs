#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Function, Parse, Peek, Result as ParserResult, T, ToCursors, function_set, keyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::NumberOrInfinity;

keyword_set!(pub enum CornerShapeKeyword {
	Round: "round",
	Scoop: "scoop",
	Bevel: "bevel",
	Notch: "notch",
	Square: "square",
	Squircle: "squircle",
});

function_set!(pub struct SuperellipseFunction "superellipse");

// https://drafts.csswg.org/css-borders-4/#typedef-corner-shape-value
// <corner-shape-value> = round | scoop | bevel | notch | square | squircle | <superellipse()>
// superellipse() = superellipse(<number [-∞,∞]> | infinity | -infinity)
#[derive(ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum CornerShapeValue<'a> {
	Round(T![Ident]),
	Scoop(T![Ident]),
	Bevel(T![Ident]),
	Notch(T![Ident]),
	Square(T![Ident]),
	Squircle(T![Ident]),
	Superellipse(Function<'a, SuperellipseFunction, NumberOrInfinity>),
}

impl<'a> Peek<'a> for CornerShapeValue<'a> {
	fn peek(p: &css_parse::Parser<'a>, c: Cursor) -> bool {
		CornerShapeKeyword::peek(p, c) || SuperellipseFunction::peek(p, c)
	}
}

impl<'a> Parse<'a> for CornerShapeValue<'a> {
	fn parse(p: &mut css_parse::Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Function]>() {
			p.parse::<Function<SuperellipseFunction, NumberOrInfinity>>().map(Self::Superellipse)
		} else {
			Ok(match p.parse::<CornerShapeKeyword>()? {
				CornerShapeKeyword::Round(t) => Self::Round(t),
				CornerShapeKeyword::Scoop(t) => Self::Scoop(t),
				CornerShapeKeyword::Bevel(t) => Self::Bevel(t),
				CornerShapeKeyword::Notch(t) => Self::Notch(t),
				CornerShapeKeyword::Square(t) => Self::Square(t),
				CornerShapeKeyword::Squircle(t) => Self::Squircle(t),
			})
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CornerShapeValue>(), 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CornerShapeValue, "squircle", CornerShapeValue::Squircle(_));
		assert_parse!(CornerShapeValue, "superellipse(-infinity)");
		assert_parse!(CornerShapeValue, "superellipse(1000)");
	}
}
