use super::prelude::*;
use crate::PositiveNonZeroInt;
use css_parse::parse_optionals;

#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum GridLineKeywords {
	#[atom(CssAtomSet::Auto)]
	Auto(T![Ident]),
	#[atom(CssAtomSet::Span)]
	Span(T![Ident]),
}

// https://drafts.csswg.org/css-grid-2/#typedef-grid-row-start-grid-line
// <grid-line> = auto | <custom-ident> | [ [ <integer [-∞,-1]> | <integer [1,∞]> ] && <custom-ident>? ] | [ span && [ <integer [1,∞]> || <custom-ident> ] ]
#[derive(Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum GridLine {
	Auto(GridLineKeywords),
	Span(GridLineKeywords, Option<PositiveNonZeroInt>, Option<T![Ident]>),
	Area(T![Ident]),
	Placement(T![Number], Option<T![Ident]>),
}

impl<'a> Parse<'a> for GridLine {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(keyword) = p.parse_if_peek::<GridLineKeywords>()? {
			return match keyword {
				GridLineKeywords::Auto(_) => Ok(GridLine::Auto(keyword)),
				GridLineKeywords::Span(_) => {
					let (num, ident) = parse_optionals!(p, num: PositiveNonZeroInt, ident: T![Ident]);
					Ok(Self::Span(keyword, num, ident))
				}
			};
		}

		if let Some(ident) = p.parse_if_peek::<T![Ident]>()? {
			return Ok(Self::Area(ident));
		}

		let num = p.parse::<T![Number]>()?;
		{
			let c: Cursor = num.into();
			if !num.is_int() {
				Err(Diagnostic::new(c, Diagnostic::expected_int))?
			}
			if num.value() == 0.0 {
				Err(Diagnostic::new(c, Diagnostic::unexpected_zero))?
			}
		}

		Ok(Self::Placement(num, p.parse_if_peek::<T![Ident]>()?))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<GridLine>(), 48);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GridLine, "auto", GridLine::Auto(_));
		assert_parse!(CssAtomSet::ATOMS, GridLine, "span 1 foo", GridLine::Span(_, Some(_), Some(_)));
		assert_parse!(CssAtomSet::ATOMS, GridLine, "span 1");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "span foo");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "span foo 1", "span 1 foo");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "baz");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "1 baz");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "-1 baz");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, GridLine, "span 0 foo");
		assert_parse_error!(CssAtomSet::ATOMS, GridLine, "span 1.2 foo");
		assert_parse_error!(CssAtomSet::ATOMS, GridLine, "span -2 foo");
		assert_parse_error!(CssAtomSet::ATOMS, GridLine, "0 baz");
		assert_parse_error!(CssAtomSet::ATOMS, GridLine, "span 0");
		assert_parse_error!(CssAtomSet::ATOMS, GridLine, "span -0 baz");
	}
}
