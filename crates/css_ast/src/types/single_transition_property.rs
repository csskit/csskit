#![allow(warnings)]
use css_lexer::{Cursor, SourceOffset};
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};
use csskit_derives::{Peek, ToCursors};

// https://drafts.csswg.org/css-transitions-1/#single-transition-property
// <single-transition-property> = all | <custom-ident>
#[derive(ToCursors, Peek, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SingleTransitionProperty {
	All(T![Ident]),
	CustomIdent(T![Ident]),
}

impl<'a> Parse<'a> for SingleTransitionProperty {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let ident = p.parse::<T![Ident]>()?;
		if p.eq_ignore_ascii_case(ident.into(), "all") {
			return Ok(SingleTransitionProperty::All(ident));
		}

		Ok(SingleTransitionProperty::CustomIdent(ident))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SingleTransitionProperty>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SingleTransitionProperty, "all");
		assert_parse!(SingleTransitionProperty, "bar");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(SingleTransitionProperty, "123deg");
	}
}
