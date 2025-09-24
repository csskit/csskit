use crate::{
	CursorSink, KindSet, Parse, Parser, Peek, Result as ParserResult, Span, T, ToCursors, ToSpan,
	syntax::ComponentValues,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SimpleBlock<'a> {
	pub open: T![PairWiseStart],
	pub values: ComponentValues<'a>,
	pub close: Option<T![PairWiseEnd]>,
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-simple-block
impl<'a> Parse<'a> for SimpleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let open = p.parse::<T![PairWiseStart]>()?;
		let stop = p.set_stop(KindSet::new(&[open.end()]));
		let values = p.parse::<ComponentValues>();
		p.set_stop(stop);
		let values = values?;
		if <T![PairWiseEnd]>::peek(p, p.peek_n(1)) {
			return Ok(Self { open, values, close: p.parse::<T![PairWiseEnd]>().ok() });
		}
		Ok(Self { open, values, close: None })
	}
}

impl<'a> ToCursors for SimpleBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.open, s);
		ToCursors::to_cursors(&self.values, s);
		ToCursors::to_cursors(&self.close, s);
	}
}

impl<'a> ToSpan for SimpleBlock<'a> {
	fn to_span(&self) -> Span {
		self.open.to_span() + if let Some(close) = self.close { close.to_span() } else { self.values.to_span() }
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::EmptyAtomSet;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SimpleBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "[foo]");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "(one two three)");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "{}");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "{foo}");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "{foo:bar}");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "{one(two)}");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "(one(two))");
		// Incomplete but recoverable
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "[foo");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "{foo:bar");
		assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "(one(two)");
		// assert_parse!(EmptyAtomSet::ATOMS, SimpleBlock, "(one(two");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(EmptyAtomSet::ATOMS, SimpleBlock, "foo");
	}
}
