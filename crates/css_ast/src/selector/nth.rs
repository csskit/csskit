use crate::{CSSInt, CssAtomSet, CssDiagnostic};
use css_parse::{
	Cursor, CursorSink, Diagnostic, Kind, KindSet, Parse, Parser, Peek, Result as ParserResult, SemanticEq, Span, T,
	ToCursors, ToSpan,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Nth {
	Odd(T![Ident]),
	Even(T![Ident]),
	Integer(CSSInt),
	Anb(i32, i32, [Cursor; 4]),
}

impl<'a> Peek<'a> for Nth {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Number]>::peek(p, c) || <T![Ident]>::peek(p, c)
	}
}

impl<'a> Parse<'a> for Nth {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<CSSInt>() {
			return Ok(Self::Integer(p.parse::<CSSInt>()?.preserve_sign()));
		} else if p.peek::<T![Ident]>() {
			let peek_cursor = p.peek_n(1);
			let atom = p.to_atom::<CssAtomSet>(peek_cursor);
			if atom == CssAtomSet::Odd {
				let ident = p.parse::<T![Ident]>()?;
				return Ok(Self::Odd(ident));
			} else if atom == CssAtomSet::Even {
				let ident = p.parse::<T![Ident]>()?;
				return Ok(Self::Even(ident));
			}
		}

		let mut c = p.next();

		let a;
		let mut b_sign = 0;
		let mut cursors = [c, Cursor::EMPTY, Cursor::EMPTY, Cursor::EMPTY];

		if c == '+' {
			let skip = p.set_skip(KindSet::NONE);
			c = p.next();
			p.set_skip(skip);
			debug_assert!(cursors[1] == Cursor::EMPTY);
			cursors[1] = c;
		}
		if !matches!(c.token().kind(), Kind::Number | Kind::Dimension | Kind::Ident) {
			Err(Diagnostic::new(c, Diagnostic::unexpected))?
		}
		if c.token().is_float() {
			Err(Diagnostic::new(c, Diagnostic::expected_int))?
		}

		if p.equals_atom(c, &CssAtomSet::_NDash) {
			b_sign = -1;
			a = if c.token().is_int() { c.token().value() as i32 } else { 1 };
		} else {
			let source_cursor = p.to_source_cursor(c);
			let anb = source_cursor.parse(p.bump());
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
				Err(Diagnostic::new(c, Diagnostic::unexpected))?
			}
			if let Ok(b) = chars.as_str().parse::<i32>() {
				return Ok(Self::Anb(a, b, cursors));
			} else if !chars.as_str().is_empty() {
				Err(Diagnostic::new(c, Diagnostic::unexpected))?
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
			if c.token().is_float() {
				Err(Diagnostic::new(c, Diagnostic::expected_int))?
			}
			if c.token().has_sign() && b_sign != 0 {
				Err(Diagnostic::new(c, Diagnostic::expected_unsigned))?
			}
			// If the number has a sign (like +1 or -1), mark it as required for minification
			if c.token().has_sign() {
				c = c.with_sign_required();
			}
			cursors[3] = c;
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

impl Nth {
	/// Check if the given 1-based index matches this Nth pattern.
	///
	/// For example:
	/// - `odd` matches indices 1, 3, 5, ...
	/// - `even` matches indices 2, 4, 6, ...
	/// - `3` matches only index 3
	/// - `2n+1` matches indices 1, 3, 5, ... (same as odd)
	/// - `3n` matches indices 3, 6, 9, ...
	pub fn matches(&self, index: i32) -> bool {
		match self {
			Self::Odd(_) => index % 2 == 1,
			Self::Even(_) => index % 2 == 0,
			Self::Integer(n) => index == i32::from(*n),
			Self::Anb(a, b, _) => {
				if *a == 0 {
					// 0n+b just matches index b
					index == *b
				} else {
					// Check if (index - b) / a is a non-negative integer
					let diff = index - b;
					diff % a == 0 && diff / a >= 0
				}
			}
		}
	}
}

impl SemanticEq for Nth {
	fn semantic_eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::Odd(a), Self::Odd(b)) => a.semantic_eq(b),
			(Self::Even(a), Self::Even(b)) => a.semantic_eq(b),
			(Self::Integer(a), Self::Integer(b)) => a.semantic_eq(b),
			(Self::Anb(a1, b1, _), Self::Anb(a2, b2, _)) => a1 == a2 && b1 == b2,
			_ => false,
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
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Nth>(), 60);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Nth, "odd");
		assert_parse!(CssAtomSet::ATOMS, Nth, "ODD");
		assert_parse!(CssAtomSet::ATOMS, Nth, "eVeN");
		assert_parse!(CssAtomSet::ATOMS, Nth, "5");
		assert_parse!(CssAtomSet::ATOMS, Nth, "n");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+n");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+N");
		assert_parse!(CssAtomSet::ATOMS, Nth, "-n");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+5");
		assert_parse!(CssAtomSet::ATOMS, Nth, "5n");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+5n");
		assert_parse!(CssAtomSet::ATOMS, Nth, "-5n");
		assert_parse!(CssAtomSet::ATOMS, Nth, "n-4");
		assert_parse!(CssAtomSet::ATOMS, Nth, "-n-4");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+n-4");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+n+4");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+n-123456789");
		assert_parse!(CssAtomSet::ATOMS, Nth, "2n");
		assert_parse!(CssAtomSet::ATOMS, Nth, "2n+1");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+2n+1");
		assert_parse!(CssAtomSet::ATOMS, Nth, "-2n+1");
		assert_parse!(CssAtomSet::ATOMS, Nth, "-2n-1");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+2n-1");
		assert_parse!(CssAtomSet::ATOMS, Nth, "3n+4");
		assert_parse!(CssAtomSet::ATOMS, Nth, "3n+1");
		assert_parse!(CssAtomSet::ATOMS, Nth, "n+ 3");
		assert_parse!(CssAtomSet::ATOMS, Nth, "-n+3");

		// Ported from https://github.com/web-platform-tests/wpt/blob/c1247636413abebe66ca11a2ca3476de771c99cb/css/selectors/parsing/parse-anplusb.html
		assert_parse!(CssAtomSet::ATOMS, Nth, "1n+0");
		assert_parse!(CssAtomSet::ATOMS, Nth, "n+0");
		assert_parse!(CssAtomSet::ATOMS, Nth, "n");
		assert_parse!(CssAtomSet::ATOMS, Nth, "-n+0");
		assert_parse!(CssAtomSet::ATOMS, Nth, "-n");
		assert_parse!(CssAtomSet::ATOMS, Nth, "N");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+n+3");
		assert_parse!(CssAtomSet::ATOMS, Nth, "+n + 7 ");
		assert_parse!(CssAtomSet::ATOMS, Nth, "N- 123");
		assert_parse!(CssAtomSet::ATOMS, Nth, "n- 10");
		assert_parse!(CssAtomSet::ATOMS, Nth, "-n\n- 1");
		assert_parse!(CssAtomSet::ATOMS, Nth, " 23n\n\n+\n\n123 ");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "3n + -6");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "3 n");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "+ 2n");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "+ 2");

		// Ported from https://github.com/web-platform-tests/wpt/blob/c1247636413abebe66ca11a2ca3476de771c99cb/css/selectors/parsing/parse-anplusb.html
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "n- 1 2");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "n-b1");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "n-+1");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "n-1n");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "-n -b1");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "-1n- b1");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "-n-13b1");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "-n-+1");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "-n+n");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "+ 1n");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "  n +12 3");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "  12 n ");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "+12n-0+1");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "+12N -- 1");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "+12 N ");
		assert_parse_error!(CssAtomSet::ATOMS, Nth, "+ n + 7");
	}

	// #[cfg(feature = "serde")]
	// #[test]
	// fn test_serializes() {
	// 	assert_json!(Nth, "odd", { "node": [2, 1], "start": 0, "end": 3 });
	// 	assert_json!(Nth, "3n+1", { "node": [3, 1], "start": 0, "end": 4 });
	// }
}
