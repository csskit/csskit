use crate::{ComponentValues, CursorSink, Parse, Parser, Result as ParserResult, Span, T, ToCursors, ToSpan};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FunctionBlock<'a> {
	name: T![Function],
	params: ComponentValues<'a>,
	close: T![')'],
}

// https://drafts.csswg.org/css-syntax-3/#consume-function
impl<'a> Parse<'a> for FunctionBlock<'a> {
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> ParserResult<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let name = p.parse::<T![Function]>()?;
		let params = p.parse::<ComponentValues>()?;
		let close = p.parse::<T![')']>()?;
		Ok(Self { name, params, close })
	}
}

impl<'a> ToCursors for FunctionBlock<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.name, s);
		ToCursors::to_cursors(&self.params, s);
		ToCursors::to_cursors(&self.close, s);
	}
}

impl<'a> ToSpan for FunctionBlock<'a> {
	fn to_span(&self) -> Span {
		self.name.to_span() + self.close.to_span()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::EmptyAtomSet;
	use crate::test_helpers::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FunctionBlock>(), 56);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, FunctionBlock, "foo(bar)");
		assert_parse!(EmptyAtomSet::ATOMS, FunctionBlock, "foo(bar{})");
	}
}
