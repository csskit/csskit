use css_parse::{
	Cursor, CursorSink, Kind, KindSet, Parse, Parser, Peek, Result as ParserResult, Span, T, ToCursors, ToSpan,
	diagnostics, keyword_set,
};
use csskit_derives::Visitable;

use crate::units::CSSInt;

keyword_set!(pub enum NthKeyword {
	Odd: "odd",
	Even: "even",
});

#[derive(Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum Nth {
	Odd(T![Ident]),
	Even(T![Ident]),
	Integer(CSSInt),
	Anb(i32, i32, [Cursor; 4]),
}

impl<'a> Peek<'a> for Nth {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Number]>::peek(p, c) || NthKeyword::peek(p, c)
	}
}

impl<'a> Parse<'a> for Nth {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let mut c: Cursor;
		if p.peek::<T![Number]>() {
			let number = p.parse::<CSSInt>()?;
			return Ok(Self::Integer(number));
		} else if let Some(kw) = p.parse_if_peek::<NthKeyword>()? {
			match kw {
				NthKeyword::Even(ident) => return Ok(Self::Even(ident)),
				NthKeyword::Odd(ident) => return Ok(Self::Odd(ident)),
			}
		} else {
			c = p.parse::<T![Any]>()?.into();
		}

		let a;
		let mut b_sign = 0;
		let mut cursors = [c, Cursor::EMPTY, Cursor::EMPTY, Cursor::EMPTY];

		if c == '+' {
			let skip = p.set_skip(KindSet::NONE);
			let next = p.parse::<T![Any]>();
			p.set_skip(skip);
			c = next?.into();
			debug_assert!(cursors[1] == Cursor::EMPTY);
			cursors[1] = c;
		}
		if !matches!(c.token().kind(), Kind::Number | Kind::Dimension | Kind::Ident) {
			Err(diagnostics::Unexpected(c.into(), c.into()))?
		}
		if c.token().is_float() {
			Err(diagnostics::ExpectedInt(c.token().value(), c.into()))?
		}

		match p.parse_str_lower(c) {
			"n-" => {
				b_sign = -1;
				a = if c.token().is_int() { c.token().value() as i32 } else { 1 };
			}
			anb => {
				let mut chars = anb.chars();
				let mut char = chars.next();
				a = if c.token().is_int() {
					c.token().value() as i32
				} else if char == Some('-') {
					char = chars.next();
					-1
				} else {
					1
				};
				if !matches!(char, Some('n') | Some('N')) {
					Err(diagnostics::Unexpected(c.into(), c.into()))?
				}
				if let Ok(b) = chars.as_str().parse::<i32>() {
					return Ok(Self::Anb(a, b, cursors));
				} else if !chars.as_str().is_empty() {
					Err(diagnostics::Unexpected(c.into(), c.into()))?
				}
			}
		}

		if b_sign == 0 {
			if p.peek::<T![+]>() {
				b_sign = 1;
				c = p.parse::<T![+]>()?.into();
				debug_assert!(cursors[2] == Cursor::EMPTY);
				cursors[2] = c;
			} else if p.peek::<T![-]>() {
				b_sign = -1;
				c = p.parse::<T![-]>()?.into();
				debug_assert!(cursors[2] == Cursor::EMPTY);
				cursors[2] = c;
			}
		}

		let b = if p.peek::<T![Number]>() {
			c = p.parse::<T![Number]>()?.into();
			debug_assert!(cursors[3] == Cursor::EMPTY);
			cursors[3] = c;
			if c.token().is_float() {
				Err(diagnostics::ExpectedInt(c.token().value(), c.into()))?
			}
			if c.token().has_sign() && b_sign != 0 {
				Err(diagnostics::ExpectedUnsigned(c.token().value(), c.into()))?
			}
			if b_sign == 0 {
				b_sign = 1;
			}
			let i = c.token().value();
			(i.abs() as i32) * b_sign
		} else {
			0
		};
		Ok(Self::Anb(a, b, cursors))
	}
}

impl ToCursors for Nth {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Odd(c) => ToCursors::to_cursors(c, s),
			Self::Even(c) => ToCursors::to_cursors(c, s),
			Self::Integer(c) => ToCursors::to_cursors(c, s),
			Self::Anb(_, _, cursors) => {
				for c in cursors {
					if *c != Cursor::EMPTY {
						s.append(*c);
					}
				}
			}
		}
	}
}

impl ToSpan for Nth {
	fn to_span(&self) -> Span {
		match self {
			Nth::Odd(c) => c.to_span(),
			Nth::Even(c) => c.to_span(),
			Nth::Integer(c) => c.to_span(),
			Nth::Anb(_, _, cursors) => {
				let mut span = Span::ZERO;
				for c in cursors {
					if *c != Cursor::EMPTY {
						span = span + (*c).into()
					}
				}
				span
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Nth>(), 60);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Nth, "odd");
		assert_parse!(Nth, "ODD");
		assert_parse!(Nth, "eVeN");
		assert_parse!(Nth, "5");
		assert_parse!(Nth, "n");
		assert_parse!(Nth, "+n");
		assert_parse!(Nth, "+N");
		assert_parse!(Nth, "-n");
		assert_parse!(Nth, "+5");
		assert_parse!(Nth, "5n");
		assert_parse!(Nth, "+5n");
		assert_parse!(Nth, "-5n");
		assert_parse!(Nth, "n-4");
		assert_parse!(Nth, "-n-4");
		assert_parse!(Nth, "+n-4");
		assert_parse!(Nth, "+n+4");
		assert_parse!(Nth, "+n-123456789");
		assert_parse!(Nth, "2n");
		assert_parse!(Nth, "2n+1");
		assert_parse!(Nth, "+2n+1");
		assert_parse!(Nth, "-2n+1");
		assert_parse!(Nth, "-2n-1");
		assert_parse!(Nth, "+2n-1");
		assert_parse!(Nth, "3n+4");
		assert_parse!(Nth, "3n+1");
		assert_parse!(Nth, "n+ 3");
		assert_parse!(Nth, "-n+3");

		// Ported from https://github.com/web-platform-tests/wpt/blob/c1247636413abebe66ca11a2ca3476de771c99cb/css/selectors/parsing/parse-anplusb.html
		assert_parse!(Nth, "1n+0");
		assert_parse!(Nth, "n+0");
		assert_parse!(Nth, "n");
		assert_parse!(Nth, "-n+0");
		assert_parse!(Nth, "-n");
		assert_parse!(Nth, "N");
		assert_parse!(Nth, "+n+3");
		assert_parse!(Nth, "+n + 7 ", "+n+ 7");
		assert_parse!(Nth, "N- 123");
		assert_parse!(Nth, "n- 10");
		assert_parse!(Nth, "-n\n- 1", "-n- 1");
		assert_parse!(Nth, " 23n\n\n+\n\n123 ", "23n+ 123");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Nth, "3n + -6");
		assert_parse_error!(Nth, "3 n");
		assert_parse_error!(Nth, "+ 2n");
		assert_parse_error!(Nth, "+ 2");

		// Ported from https://github.com/web-platform-tests/wpt/blob/c1247636413abebe66ca11a2ca3476de771c99cb/css/selectors/parsing/parse-anplusb.html
		assert_parse_error!(Nth, "n- 1 2");
		assert_parse_error!(Nth, "n-b1");
		assert_parse_error!(Nth, "n-+1");
		assert_parse_error!(Nth, "n-1n");
		assert_parse_error!(Nth, "-n -b1");
		assert_parse_error!(Nth, "-1n- b1");
		assert_parse_error!(Nth, "-n-13b1");
		assert_parse_error!(Nth, "-n-+1");
		assert_parse_error!(Nth, "-n+n");
		assert_parse_error!(Nth, "+ 1n");
		assert_parse_error!(Nth, "  n +12 3");
		assert_parse_error!(Nth, "  12 n ");
		assert_parse_error!(Nth, "+12n-0+1");
		assert_parse_error!(Nth, "+12N -- 1");
		assert_parse_error!(Nth, "+12 N ");
		assert_parse_error!(Nth, "+ n + 7");
	}

	// #[cfg(feature = "serde")]
	// #[test]
	// fn test_serializes() {
	// 	assert_json!(Nth, "odd", { "node": [2, 1], "start": 0, "end": 3 });
	// 	assert_json!(Nth, "3n+1", { "node": [3, 1], "start": 0, "end": 4 });
	// }
}
