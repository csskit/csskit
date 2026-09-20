use super::prelude::*;

use crate::CSSInt;
use css_parse::{CursorSink, Nth as NthTrait, SemanticEq, Span, ToCursors, ToSpan};

#[node]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Nth {
	Odd(T![Ident]),
	Even(T![Ident]),
	Integer(CSSInt),
	Anb(#[metadata(skip)] i32, #[metadata(skip)] i32, #[metadata(skip)] [Cursor; 4]),
}

impl<'a> Peek<'a> for Nth {
	const PEEK_KINDSET: KindSet = <Self as NthTrait<'a>>::NTH_KINDSET;
}

impl<'a> NthTrait<'a> for Nth {
	fn peek_odd<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.to_atom::<CssAtomSet>(c) == CssAtomSet::Odd
	}

	fn peek_even<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.to_atom::<CssAtomSet>(c) == CssAtomSet::Even
	}

	fn build_odd(node: T![Ident]) -> Self {
		Self::Odd(node)
	}

	fn build_even(node: T![Ident]) -> Self {
		Self::Even(node)
	}

	fn build_integer(node: T![Number]) -> Self {
		Self::Integer(CSSInt(node))
	}

	fn build_anb(a: i32, b: i32, cursors: [Cursor; 4]) -> Self {
		Self::Anb(a, b, cursors)
	}
}

impl<'a> Parse<'a> for Nth {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_nth(p)
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
			Self::Anb(a, b, _) => Self::anb_matches(*a, *b, index),
		}
	}
}

impl SemanticEq for Nth {
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		match (self, other) {
			(Self::Odd(a), Self::Odd(b)) => a.semantic_eq(b, source_text),
			(Self::Even(a), Self::Even(b)) => a.semantic_eq(b, source_text),
			(Self::Integer(a), Self::Integer(b)) => a.semantic_eq(b, source_text),
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
				let mut span = Span::DUMMY;
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
	use css_parse::{assert_parse, assert_parse_error, assert_parse_span};

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
		// A `-` closing the unit makes b negative, whether or not a is signed or spelled out.
		assert_parse!(CssAtomSet::ATOMS, Nth, "-n- 4");
		assert_parse!(CssAtomSet::ATOMS, Nth, "5n- 4");
		assert_parse!(CssAtomSet::ATOMS, Nth, "-5n- 4");
	}

	#[test]
	fn test_spans() {
		assert_parse_span!(
			CssAtomSet::ATOMS,
			Nth,
			r#"
			odd
			^^^
		"#
		);
		assert_parse_span!(
			CssAtomSet::ATOMS,
			Nth,
			r#"
			5
			^
		"#
		);
		assert_parse_span!(
			CssAtomSet::ATOMS,
			Nth,
			r#"
			2n
			^^
		"#
		);
		assert_parse_span!(
			CssAtomSet::ATOMS,
			Nth,
			r#"
			-2n+1 foo
			^^^^^
		"#
		);
		assert_parse_span!(
			CssAtomSet::ATOMS,
			Nth,
			r#"
			n- 10
			^^^^^
		"#
		);
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
