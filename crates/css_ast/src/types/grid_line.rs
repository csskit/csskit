use css_lexer::Cursor;
use css_parse::{Parse, Parser, Result as ParserResult, T, diagnostics, keyword_set};
use csskit_derives::{IntoSpan, Peek, ToCursors};

use crate::Unit;

keyword_set!(GridLineKeywords { Auto: "auto", Span: "span" });

// https://drafts.csswg.org/css-grid-2/#typedef-grid-row-start-grid-line
// <grid-line> = auto | <custom-ident> | [ [ <integer [-∞,-1]> | <integer [1,∞]> ] && <custom-ident>? ] | [ span && [ <integer [1,∞]> || <custom-ident> ] ]
#[derive(IntoSpan, Peek, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum GridLine {
	Auto(GridLineKeywords),
	Span(GridLineKeywords, T![Number], T![Ident]),
	Area(T![Ident]),
	Placement(T![Number], Option<T![Ident]>),
}

impl<'a> Parse<'a> for GridLine {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(keyword) = p.parse_if_peek::<GridLineKeywords>()? {
			return match keyword {
				GridLineKeywords::Auto(_) => Ok(GridLine::Auto(keyword)),
				GridLineKeywords::Span(_) => {
					let mut num = None;
					let mut ident = None;

					while num.is_none() || ident.is_none() {
						if num.is_none() {
							num = p.parse_if_peek::<T![Number]>()?;
							if num.is_some() {
								continue;
							}
						}

						if ident.is_none() {
							ident = p.parse_if_peek::<T![Ident]>()?;
							if ident.is_some() {
								continue;
							}
						}

						break;
					}

					if num.is_none() && ident.is_none() {
						let c: Cursor = p.parse::<T![Any]>()?.into();
						Err(diagnostics::Unexpected(c.into(), c.into()))?
					}

					let num = num.unwrap();
					let ident = ident.unwrap();
					{
						let c: Cursor = num.into();
						if !num.is_int() {
							Err(diagnostics::ExpectedInt(num.into(), c.into()))?
						}
						if !(num.is_positive() && num.value() != 0.0) {
							Err(diagnostics::NumberTooSmall(num.into(), c.into()))?
						}
					}

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
				Err(diagnostics::ExpectedInt(num.into(), c.into()))?
			}
			if num.value() == 0.0 {
				Err(diagnostics::UnexpectedZero(c.into()))?
			}
		}

		Ok(Self::Placement(num, p.parse_if_peek::<T![Ident]>()?))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<GridLine>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(GridLine, "auto");
		assert_parse!(GridLine, "span 1 foo");
		assert_parse!(GridLine, "span foo 1", "span 1 foo");
		assert_parse!(GridLine, "baz");
		assert_parse!(GridLine, "1 baz");
		assert_parse!(GridLine, "-1 baz");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(GridLine, "span 0 foo");
		assert_parse_error!(GridLine, "span 1.2 foo");
		assert_parse_error!(GridLine, "span -2 foo");
		assert_parse_error!(GridLine, "0 baz");
	}
}
