use crate::units::Angle;
use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};
use csskit_derives::{Parse, Peek, ToCursors};

#[derive(Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum SkewKind {
	Angle(Angle),
	Zero(T![Number]),
}

// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-skew
// skew() = skew( [ <angle> | <zero> ] , [ <angle> | <zero> ]? )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Skew {
	skew: T![Function],
	pub x: SkewKind,
	x_: Option<T![,]>,
	pub y: Option<SkewKind>,
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for Skew {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "skew")
	}
}

impl<'a> Parse<'a> for Skew {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let skew = p.parse::<T![Function]>()?;
		let c: Cursor = skew.into();
		if !p.eq_ignore_ascii_case(c, "skew") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let x = p.parse::<SkewKind>()?;
		let x_ = p.parse_if_peek::<T![,]>()?;
		let y = p.parse_if_peek::<SkewKind>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { skew, x, x_, y, close })
	}
}

impl<'a> ToCursors for Skew {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.skew, s);
		ToCursors::to_cursors(&self.x, s);
		if let Some(ref x_) = self.x_ {
			ToCursors::to_cursors(x_, s);
		}
		if let Some(ref y) = self.y {
			ToCursors::to_cursors(y, s);
		}
		if let Some(ref close) = self.close {
			ToCursors::to_cursors(close, s);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Skew>(), 76);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Skew, "skew(1deg,2deg)");
		assert_parse!(Skew, "skew(0,0)");
		assert_parse!(Skew, "skew(1deg)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Skew, "skew()");
		assert_parse_error!(Skew, "skew(foo)");
	}
}
