use crate::{Diagnostic, Parse, Parser, Peek, Result, SemanticEq, T, ToCursors};

/// This enum represents a set of comparison operators, used in Ranged Media Features (see
/// [RangedFeature][crate::RangedFeature]), and could be used in other parts of a CSS-alike language. This isn't a
/// strictly standard part of CSS, but is provided for convenience.
///
/// [Comparison] is defined as:
///
/// ```md
/// <comparison>
///  │├──╮─ "="  ─╭──┤│
///      ├─ "<"  ─┤
///      ├─ "<=" ─┤
///      ├─ ">"  ─┤
///      ╰─ ">=" ─╯
/// ```
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Comparison {
	LessThan(T![<]),
	GreaterThan(T![>]),
	GreaterThanEqual(T![>=]),
	LessThanEqual(T![<=]),
	Equal(T![=]),
}

impl<'a> Parse<'a> for Comparison {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Comparison>
	where
		I: Iterator<Item = crate::Cursor> + Clone,
	{
		let c = p.peek_n(1);
		match c.token().char() {
			Some('=') => p.parse::<T![=]>().map(Comparison::Equal),
			Some('>') => {
				if <T![>=]>::peek(p, c) {
					p.parse::<T![>=]>().map(Comparison::GreaterThanEqual)
				} else {
					p.parse::<T![>]>().map(Comparison::GreaterThan)
				}
			}
			Some('<') => {
				if <T![<=]>::peek(p, c) {
					p.parse::<T![<=]>().map(Comparison::LessThanEqual)
				} else {
					p.parse::<T![<]>().map(Comparison::LessThan)
				}
			}
			Some(_) => Err(Diagnostic::new(p.next(), Diagnostic::unexpected_delim))?,
			_ => Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?,
		}
	}
}

impl ToCursors for Comparison {
	fn to_cursors(&self, s: &mut impl crate::CursorSink) {
		match self {
			Self::LessThan(c) => ToCursors::to_cursors(c, s),
			Self::GreaterThan(c) => ToCursors::to_cursors(c, s),
			Self::GreaterThanEqual(c) => ToCursors::to_cursors(c, s),
			Self::LessThanEqual(c) => ToCursors::to_cursors(c, s),
			Self::Equal(c) => ToCursors::to_cursors(c, s),
		}
	}
}

impl SemanticEq for Comparison {
	fn semantic_eq(&self, other: &Self) -> bool {
		match (self, other) {
			(Self::LessThan(a), Self::LessThan(b)) => a.semantic_eq(b),
			(Self::GreaterThan(a), Self::GreaterThan(b)) => a.semantic_eq(b),
			(Self::GreaterThanEqual(a), Self::GreaterThanEqual(b)) => a.semantic_eq(b),
			(Self::LessThanEqual(a), Self::LessThanEqual(b)) => a.semantic_eq(b),
			(Self::Equal(a), Self::Equal(b)) => a.semantic_eq(b),
			_ => false,
		}
	}
}
