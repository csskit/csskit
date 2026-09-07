use super::prelude::*;

/// The operator of an attribute selector, e.g. the `^=` of `[href^="https"]`.
///
/// <https://drafts.csswg.org/selectors/#attribute-selectors>
#[node]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AttributeOperator {
	Exact(T![=]),
	SpaceList(T![~=]),
	LangPrefix(T![|=]),
	Prefix(T![^=]),
	Suffix(T!["$="]),
	Contains(T![*=]),
}

impl<M: NodeMetadata> NodeWithMetadata<M> for AttributeOperator {
	fn metadata(&self) -> M {
		M::default()
	}
}

impl<'a> Peek<'a> for AttributeOperator {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Delim]);
}

impl<'a> Parse<'a> for AttributeOperator {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<T![~=]>() {
			p.parse::<T![~=]>().map(Self::SpaceList)
		} else if p.peek::<T![|=]>() {
			p.parse::<T![|=]>().map(Self::LangPrefix)
		} else if p.peek::<T![^=]>() {
			p.parse::<T![^=]>().map(Self::Prefix)
		} else if p.peek::<T!["$="]>() {
			p.parse::<T!["$="]>().map(Self::Suffix)
		} else if p.peek::<T![*=]>() {
			p.parse::<T![*=]>().map(Self::Contains)
		} else {
			p.parse::<T![=]>().map(Self::Exact)
		}
	}
}

impl ToCursors for AttributeOperator {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::Exact(t) => t.to_cursors(s),
			Self::SpaceList(t) => t.to_cursors(s),
			Self::LangPrefix(t) => t.to_cursors(s),
			Self::Prefix(t) => t.to_cursors(s),
			Self::Suffix(t) => t.to_cursors(s),
			Self::Contains(t) => t.to_cursors(s),
		}
	}
}

impl ToSpan for AttributeOperator {
	fn to_span(&self) -> Span {
		match self {
			Self::Exact(t) => t.to_span(),
			Self::SpaceList(t) => t.to_span(),
			Self::LangPrefix(t) => t.to_span(),
			Self::Prefix(t) => t.to_span(),
			Self::Suffix(t) => t.to_span(),
			Self::Contains(t) => t.to_span(),
		}
	}
}

impl SemanticEq for AttributeOperator {
	fn semantic_eq(&self, other: &Self, _source_text: &str) -> bool {
		std::mem::discriminant(self) == std::mem::discriminant(other)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{EmptyAtomSet, assert_parse};

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, AttributeOperator, "=");
		assert_parse!(EmptyAtomSet::ATOMS, AttributeOperator, "~=");
		assert_parse!(EmptyAtomSet::ATOMS, AttributeOperator, "|=");
		assert_parse!(EmptyAtomSet::ATOMS, AttributeOperator, "^=");
		assert_parse!(EmptyAtomSet::ATOMS, AttributeOperator, "$=");
		assert_parse!(EmptyAtomSet::ATOMS, AttributeOperator, "*=");
	}
}
