use super::prelude::*;
use crate::ToNumberValue;

#[node]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Either<Left, Right> {
	Left(Left),
	Right(Right),
}

impl<'a, Left: Peek<'a>, Right: Peek<'a>> Peek<'a> for Either<Left, Right> {
	const PEEK_KINDSET: KindSet = Left::PEEK_KINDSET.combine(Right::PEEK_KINDSET);

	fn peek<I>(p: &crate::Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Left::peek(p, c) || Right::peek(p, c)
	}
}

impl<'a, Left: Parse<'a> + Peek<'a>, Right: Parse<'a>> Parse<'a> for Either<Left, Right> {
	fn parse<I>(p: &mut crate::Parser<'a, I>) -> crate::Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if Left::peek(p, c)
			&& let Ok(res) = Left::parse(p)
		{
			Ok(Either::Left(res))
		} else {
			Right::parse(p).map(Either::Right)
		}
	}
}

impl<Left: ToCursors, Right: ToCursors> ToCursors for Either<Left, Right> {
	fn to_cursors(&self, s: &mut impl crate::CursorSink) {
		match self {
			Self::Left(t) => t.to_cursors(s),
			Self::Right(t) => t.to_cursors(s),
		}
	}
}

impl<Left: ToSpan, Right: ToSpan> ToSpan for Either<Left, Right> {
	fn to_span(&self) -> css_lexer::Span {
		match self {
			Self::Left(t) => t.to_span(),
			Self::Right(t) => t.to_span(),
		}
	}
}

impl<Left, Right, M: NodeMetadata> NodeWithMetadata<M> for Either<Left, Right>
where
	Left: NodeWithMetadata<M>,
	Right: NodeWithMetadata<M>,
{
	fn metadata(&self) -> M {
		match self {
			Self::Left(t) => t.metadata(),
			Self::Right(t) => t.metadata(),
		}
	}
}

impl<Left, Right> ToNumberValue for Either<Left, Right>
where
	Left: ToNumberValue,
	Right: ToNumberValue,
{
	fn to_number_value(&self) -> Option<f32> {
		match self {
			Self::Left(t) => t.to_number_value(),
			Self::Right(t) => t.to_number_value(),
		}
	}
}

impl<Left: SemanticEq, Right: SemanticEq> SemanticEq for Either<Left, Right> {
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		match (self, other) {
			(Self::Left(a), Self::Left(b)) => a.semantic_eq(b, source_text),
			(Self::Right(a), Self::Right(b)) => a.semantic_eq(b, source_text),
			_ => false,
		}
	}
}

impl<Left: Copy, Right: Copy> Copy for Either<Left, Right> {}

impl<Left, Right> From<Either<Left, Right>> for Cursor
where
	Left: Copy,
	Right: Copy,
	Cursor: From<Left> + From<Right>,
{
	fn from(value: Either<Left, Right>) -> Self {
		match value {
			Either::Left(t) => t.into(),
			Either::Right(t) => t.into(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{T, assert_parse, assert_parse_error, assert_peek_false};
	use css_lexer::EmptyAtomSet;

	type IdentOrNumber = Either<T![Ident], T![Number]>;
	type NumberOrDimension = Either<T![Number], T![Dimension]>;

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, IdentOrNumber, "all", Either::Left(_));
		assert_parse!(EmptyAtomSet::ATOMS, IdentOrNumber, "1", Either::Right(_));
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(EmptyAtomSet::ATOMS, IdentOrNumber, "");
		assert_peek_false!(EmptyAtomSet::ATOMS, IdentOrNumber, "foo(");
		assert_parse_error!(EmptyAtomSet::ATOMS, IdentOrNumber, "auto auto");
		assert_parse_error!(EmptyAtomSet::ATOMS, IdentOrNumber, "1 1");
	}

	#[test]
	fn test_to_number_value() {
		assert_parse!(EmptyAtomSet::ATOMS, NumberOrDimension, "47", |node| {
			assert_eq!(node.to_number_value(), Some(47.0));
		});
		assert_parse!(EmptyAtomSet::ATOMS, NumberOrDimension, "47px", |node| {
			assert_eq!(node.to_number_value(), Some(47.0));
		});
	}
}
