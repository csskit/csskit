use crate::units::Angle;
use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};
use csskit_derives::{Parse, ToCursors};

#[derive(Parse, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum RotateKind {
	Angle(Angle),
	Zero(T![Number]),
}

// https://drafts.csswg.org/css-transforms-1/#funcdef-transform-rotate
// rotate() = rotate( [ <angle> | <zero> ] )
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Rotate {
	rotate: T![Function],
	pub kind: RotateKind,
	close: Option<T![')']>,
}

impl<'a> Peek<'a> for Rotate {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "rotate")
	}
}

impl<'a> Parse<'a> for Rotate {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let rotate = p.parse::<T![Function]>()?;
		let c: Cursor = rotate.into();
		if !p.eq_ignore_ascii_case(c, "rotate") {
			Err(diagnostics::UnexpectedFunction(p.parse_str(c).into(), c.into()))?
		}
		let kind = p.parse::<RotateKind>()?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { rotate, kind, close })
	}
}

impl<'a> ToCursors for Rotate {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.rotate, s);
		ToCursors::to_cursors(&self.kind, s);
		if let Some(ref close) = self.close {
			ToCursors::to_cursors(close, s)
		};
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Rotate>(), 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Rotate, "rotate(45deg)");
		assert_parse!(Rotate, "rotate(0)");
		assert_parse!(Rotate, "rotate(2turn)");
		assert_parse!(Rotate, "rotate(20rad");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Rotate, "rotate(45px)");
		assert_parse_error!(Rotate, "rotate(all the way around)");
	}
}
