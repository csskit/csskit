#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics, keyword_set};
use csskit_derives::{IntoSpan, Parse, Peek, ToCursors};

use crate::Unit;

keyword_set!(GridLineKeywords { Auto: "auto", Span: "span" });

// https://drafts.csswg.org/css-grid-2/#typedef-grid-row-start-grid-line
// <grid-line> = auto | <custom-ident> | [ [ <integer [-∞,-1]> | <integer [1,∞]> ] && <custom-ident>? ] | [ span && [ <integer [1,∞]> || <custom-ident> ] ]
#[derive(IntoSpan, ToCursors, Peek, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum GridLine {
	Auto(GridLineKeywords),
	Span(GridLineKeywords, T![Number], T![Ident]),
}

impl<'a> Parse<'a> for GridLine {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(keyword) = p.parse_if_peek::<GridLineKeywords>()? {
			match keyword {
				GridLineKeywords::Auto(_) => return Ok(GridLine::Auto(keyword)),
				GridLineKeywords::Span(c) => {
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

					if !num.is_int() {
						let c: Cursor = num.into();
						Err(diagnostics::ExpectedInt(num.into(), c.into()))?
					}

					if !(num.is_positive() && num.value() != 0.0) {
						let c: Cursor = num.into();
						Err(diagnostics::NumberTooSmall(num.into(), c.into()))?
					}

					return Ok(Self::Span(keyword, num, ident));
				}
			}
		}

		todo!()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		// assert_eq!(std::mem::size_of::<GridLine>(), 1);
	}

	#[test]
	fn test_writes() {
		assert_parse!(GridLine, "auto");
		assert_parse!(GridLine, "span 1 foo");
		assert_parse!(GridLine, "span foo 1", "span 1 foo");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(GridLine, "span 0 foo");
		assert_parse_error!(GridLine, "span 1.2 foo");
		assert_parse_error!(GridLine, "span -2 foo");
	}
}
