use super::prelude::*;
use crate::{CSSInt, CalcableValue, CustomIdent, NonZero, PositiveNonZeroInt};
use css_parse::parse_optionals;

/// <https://drafts.csswg.org/css-grid-2/#typedef-grid-row-start-grid-line>
///
/// ```text,ignore
/// <grid-line> = auto | <custom-ident> | [ [ <integer [-∞,-1]> | <integer [1,∞]> ] && <custom-ident>? ] | [ span && [ <integer [1,∞]> || <custom-ident> ] ]
/// ```
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum GridLine<'a> {
	Auto(T![Ident]),
	Span(T![Ident], Option<CalcableValue<'a, PositiveNonZeroInt>>, Option<T![Ident]>),
	Area(CustomIdent),
	Placement(CalcableValue<'a, NonZero<CSSInt>>, Option<T![Ident]>),
}

impl<'a> Parse<'a> for GridLine<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if <T![Ident]>::peek(p, c) {
			return match p.to_atom::<CssAtomSet>(c) {
				CssAtomSet::Auto => Ok(GridLine::Auto(p.parse::<T![Ident]>()?)),
				CssAtomSet::Span => {
					let keyword = p.parse::<T![Ident]>()?;
					let (num, ident) = parse_optionals!(p, num: CalcableValue<PositiveNonZeroInt>, ident: T![Ident]);
					Ok(Self::Span(keyword, num, ident))
				}
				_ => Ok(Self::Area(p.parse::<CustomIdent>()?)),
			};
		}
		let num = p.parse::<CalcableValue<NonZero<CSSInt>>>()?;
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
		assert_eq!(std::mem::size_of::<GridLine>(), 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GridLine, "auto", GridLine::Auto(_));
		assert_parse!(CssAtomSet::ATOMS, GridLine, "span 1 foo", GridLine::Span(_, Some(_), Some(_)));
		assert_parse!(CssAtomSet::ATOMS, GridLine, "span 1");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "span foo");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "span foo 1");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "baz");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "1 baz");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "-1 baz");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, GridLine, "var(--n)");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "calc(1 + 1) baz");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "span var(--n)");
		assert_parse!(CssAtomSet::ATOMS, GridLine, "span calc(1 + 1) foo");
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
