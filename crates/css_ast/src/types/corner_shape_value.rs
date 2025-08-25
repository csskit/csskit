use css_parse::{Cursor, Parse, Peek, Result as ParserResult, T, keyword_set};
use csskit_derives::{ToCursors, ToSpan, Visitable};

use crate::SuperellipseFunction;

keyword_set!(pub enum CornerShapeKeyword {
	Round: "round",
	Scoop: "scoop",
	Bevel: "bevel",
	Notch: "notch",
	Square: "square",
	Squircle: "squircle",
});

/// <https://drafts.csswg.org/css-borders-4/#typedef-corner-shape-value>
///
/// ```text,ignore
/// <corner-shape-value> = round | scoop | bevel | notch | square | squircle | <superellipse()>
/// ```
#[derive(ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum CornerShapeValue {
	#[visit(skip)]
	Round(T![Ident]),
	#[visit(skip)]
	Scoop(T![Ident]),
	#[visit(skip)]
	Bevel(T![Ident]),
	#[visit(skip)]
	Notch(T![Ident]),
	#[visit(skip)]
	Square(T![Ident]),
	#[visit(skip)]
	Squircle(T![Ident]),
	Superellipse(SuperellipseFunction),
}

impl<'a> Peek<'a> for CornerShapeValue {
	fn peek(p: &css_parse::Parser<'a>, c: Cursor) -> bool {
		CornerShapeKeyword::peek(p, c) || SuperellipseFunction::peek(p, c)
	}
}

impl<'a> Parse<'a> for CornerShapeValue {
	fn parse(p: &mut css_parse::Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Ident]>() {
			Ok(match p.parse::<CornerShapeKeyword>()? {
				CornerShapeKeyword::Round(t) => Self::Round(t),
				CornerShapeKeyword::Scoop(t) => Self::Scoop(t),
				CornerShapeKeyword::Bevel(t) => Self::Bevel(t),
				CornerShapeKeyword::Notch(t) => Self::Notch(t),
				CornerShapeKeyword::Square(t) => Self::Square(t),
				CornerShapeKeyword::Squircle(t) => Self::Squircle(t),
			})
		} else {
			p.parse::<SuperellipseFunction>().map(Self::Superellipse)
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
