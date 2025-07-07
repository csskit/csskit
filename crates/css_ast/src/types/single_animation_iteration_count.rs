use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};

use crate::units::CSSFloat;

// https://drafts.csswg.org/css-animations/#typedef-single-animation-iteration-count
// <single-animation-iteration-count> = infinite | <number [0,∞]>
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
pub enum SingleAnimationIterationCount {
	Infinite(T![Ident]),
	Number(CSSFloat),
}

impl<'a> Peek<'a> for SingleAnimationIterationCount {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<CSSFloat>::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "infinite"))
	}
}

impl<'a> Parse<'a> for SingleAnimationIterationCount {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Ident]>() && p.eq_ignore_ascii_case(p.peek_n(1), "infinite") {
			return Ok(Self::Infinite(p.parse::<T![Ident]>()?));
		}
		let int = p.parse::<CSSFloat>()?;
		let f: f32 = int.into();
		if f < 0.0 {
			let c: Cursor = int.into();
			Err(diagnostics::NumberTooSmall(f, c.into()))?
		}
		Ok(Self::Number(int))
	}
}

impl From<SingleAnimationIterationCount> for Cursor {
	fn from(value: SingleAnimationIterationCount) -> Self {
		match value {
			SingleAnimationIterationCount::Infinite(c) => c.into(),
			SingleAnimationIterationCount::Number(c) => c.into(),
		}
	}
}

impl ToCursors for SingleAnimationIterationCount {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append((*self).into());
	}
}
