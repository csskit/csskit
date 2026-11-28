use super::prelude::*;
use crate::{Result, syntax::ComponentValues};

#[node]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SimpleBlock<'a> {
	pub open: T![PairWiseStart],
	pub values: ComponentValues<'a>,
	pub close: T![PairWiseEnd],
}

impl<'a> Peek<'a> for SimpleBlock<'a> {
	const PEEK_KINDSET: KindSet = KindSet::PAIRWISE_START;
}

impl<'a, M: NodeMetadata> NodeWithMetadata<M> for SimpleBlock<'a> {
	fn metadata(&self) -> M {
		M::default()
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-simple-block
impl<'a> Parse<'a> for SimpleBlock<'a> {
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let open = p.parse::<T![PairWiseStart]>()?;
		let stop = p.set_stop(KindSet::new(&[open.end()]));
		let values = p.parse_infallible::<ComponentValues>();
		p.set_stop(stop);
		let close = if p.peek_n(1) == open.end() {
			p.parse::<T![PairWiseEnd]>()?
		} else {
			<T![PairWiseEnd]>::missing(p, open.end())
		};
		Ok(Self { open, values, close })
	}
}

impl<'a> ToCursors for SimpleBlock<'a> {
	fn to_cursors(&self, s: &mut (impl CursorSink + ?Sized)) {
		ToCursors::to_cursors(&self.open, s);
		ToCursors::to_cursors(&self.values, s);
		ToCursors::to_cursors(&self.close, s);
	}
}

impl<'a> ToSpan for SimpleBlock<'a> {
	fn to_span(&self) -> Span {
		self.open.to_span() + self.close.to_span()
	}
}

impl<'a> SemanticEq for SimpleBlock<'a> {
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		self.open.semantic_eq(&other.open, source_text)
			&& self.values.semantic_eq(&other.values, source_text)
			&& self.close.semantic_eq(&other.close, source_text)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::EmptyAtomSet;
	use crate::test_helpers::*;

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "[foo]");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "(one two three)");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "{}");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "{foo}");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "{foo:bar}");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "{one(two)}");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "(one(two))");
	}

	#[test]
	fn test_missing_close_recovers() {
		assert_parse_recovers!(EmptyAtomSet::ATOMS, SimpleBlock, "[foo", "[foo]");
		assert_parse_recovers!(EmptyAtomSet::ATOMS, SimpleBlock, "{foo:bar", "{foo:bar}");
		assert_parse_recovers!(EmptyAtomSet::ATOMS, SimpleBlock, "(one(two)", "(one(two))");
		assert_parse_recovers!(EmptyAtomSet::ATOMS, SimpleBlock, "(one(two", "(one(two))");
		assert_parse_recovers!(EmptyAtomSet::ATOMS, SimpleBlock, "{a:[b", "{a:[b]}");
		assert_parse_recovers!(EmptyAtomSet::ATOMS, SimpleBlock, "({)", "({})");
		assert_parse_recovers!(EmptyAtomSet::ATOMS, SimpleBlock, "[(]", "[()]");
	}

	#[test]
	fn test_peek() {
		assert_peek_false!(EmptyAtomSet::ATOMS, SimpleBlock, "foo");
		assert_peek_false!(EmptyAtomSet::ATOMS, SimpleBlock, "");
	}
}
