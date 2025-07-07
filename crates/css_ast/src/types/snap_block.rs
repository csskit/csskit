use css_lexer::Cursor;
use css_parse::{Parse, Parser, Peek, Result as ParserResult, T, diagnostics, keyword_set};
use csskit_derives::ToCursors;

use crate::units::LengthPercentage;

// https://drafts.csswg.org/css-page-floats-3/#funcdef-float-snap-block
// snap-block() = snap-block( <length> , [ start | end | near ]? )
#[derive(ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct SnapBlock {
	pub function: T![Function],
	pub length: LengthPercentage,
	pub comma: Option<T![,]>,
	pub keyword: Option<SnapBlockKeyword>,
	pub close: Option<T![')']>,
}

impl<'a> Peek<'a> for SnapBlock {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "snap-block")
	}
}

impl<'a> Parse<'a> for SnapBlock {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let function = p.parse::<T![Function]>()?;
		let c: Cursor = function.into();
		if !p.eq_ignore_ascii_case(c, "snap-block") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let length = p.parse::<LengthPercentage>()?;
		let comma = p.parse_if_peek::<T![,]>()?;
		let keyword = p.parse_if_peek::<SnapBlockKeyword>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { function, length, comma, keyword, close })
	}
}

keyword_set!(SnapBlockKeyword { Start: "start", End: "end", Near: "near" });
