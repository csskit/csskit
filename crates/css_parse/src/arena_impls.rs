use crate::{Cursor, CursorSink, Parse, Parser, Peek, SemanticEq, ToCursors};
use allocator_api2::alloc::Allocator;
use css_lexer::KindSet;
use csskit_arena::Box;

impl<'a, T: ToCursors, A: Allocator> ToCursors for Box<'a, T, A> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		(**self).to_cursors(s);
	}
}

impl<'a, T: SemanticEq, A: Allocator> SemanticEq for Box<'a, T, A> {
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		(**self).semantic_eq(other, source_text)
	}
}

impl<'a, M: crate::NodeMetadata, T: crate::NodeWithMetadata<M>, A: Allocator> crate::NodeWithMetadata<M>
	for Box<'a, T, A>
{
	fn self_metadata(&self) -> M {
		(**self).self_metadata()
	}

	fn metadata(&self) -> M {
		(**self).metadata()
	}
}

impl<'a, T: Peek<'a>, A: Allocator> Peek<'a> for Box<'a, T, A> {
	const PEEK_KINDSET: KindSet = T::PEEK_KINDSET;

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		T::peek(p, c)
	}
}

impl<'a, T: Parse<'a>> Parse<'a> for Box<'a, T> {
	fn parse<I>(p: &mut Parser<'a, I>) -> crate::Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let value = T::parse(p)?;
		Ok(Box::new_in(p.alloc(), value))
	}
}

#[cfg(test)]
mod test {
	use crate::{Arena, ComponentValues, EmptyAtomSet, Parser};
	use css_lexer::Lexer;
	use std::panic::{AssertUnwindSafe, catch_unwind};

	#[test]
	fn parsing_beyond_default_arena_capacity_does_not_panic() {
		let source = "a ".repeat(16_384);
		let result = catch_unwind(AssertUnwindSafe(|| {
			let arena = Arena::new();
			let lexer = Lexer::new(&EmptyAtomSet::ATOMS, &source);
			let mut parser = Parser::new(&arena, &source, lexer);
			let _ = parser.parse_entirely::<ComponentValues>();
		}));

		assert!(result.is_ok(), "arena exhaustion must not abort parsing");
	}
}
