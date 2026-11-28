use super::prelude::*;
use crate::{Result, syntax::ComponentValue};

#[node]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BadDeclaration<'a>(pub(crate) Vec<'a, ComponentValue<'a>>);

impl<'a> BadDeclaration<'a> {
	/// Returns `true` when no tokens were consumed.
	pub fn is_empty(&self) -> bool {
		self.0.is_empty()
	}

	pub(crate) fn stray<I>(p: &Parser<'a, I>, value: ComponentValue<'a>) -> Self
	where
		I: Iterator<Item = crate::Cursor> + Clone,
	{
		let mut values = Vec::with_capacity_in(1, p.alloc());
		values.push(value);
		Self(values)
	}

	/// Consumes one component value, keeping tokens a component value cannot hold (stray closers, bad strings and
	/// urls) as error variants.
	pub(crate) fn parse_value<Iter>(p: &mut Parser<'a, Iter>) -> Result<ComponentValue<'a>>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		if let Some(closer) = p.parse_if_peek::<T![PairWiseEnd]>()? {
			return Ok(ComponentValue::StrayCloser(closer));
		}
		if let Some(bad) = p.parse_if_peek::<T![BadString]>()? {
			return Ok(ComponentValue::BadString(bad));
		}
		if let Some(bad) = p.parse_if_peek::<T![BadUrl]>()? {
			return Ok(ComponentValue::BadUrl(bad));
		}
		p.parse::<ComponentValue>()
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-the-remnants-of-a-bad-declaration
impl<'a> Parse<'a> for BadDeclaration<'a> {
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let mut values = Vec::new_in(p.alloc());
		// To consume the remnants of a bad declaration from a token stream input, given a bool nested:
		//
		// Process input:
		loop {
			// <eof-token>
			// <semicolon-token>
			//
			//     Discard a token from input, and return nothing.
			if p.at_end() {
				return Ok(Self(values));
			}
			let c = p.peek_n(1);
			if <T![;]>::peek(p, c) {
				values.push(p.parse::<ComponentValue>()?);
				return Ok(Self(values));
			}

			// <}-token>
			//
			//     If nested is true, return nothing. Otherwise, discard a token.
			if <T!['}']>::peek(p, c) && p.is(State::Nested) {
				return Ok(Self(values));
			}
			values.push(Self::parse_value(p)?);
		}
	}
}

impl<'a> ToSpan for BadDeclaration<'a> {
	fn to_span(&self) -> Span {
		self.0.to_span()
	}
}

impl<'a> ToCursors for BadDeclaration<'a> {
	fn to_cursors(&self, s: &mut (impl CursorSink + ?Sized)) {
		for value in &self.0 {
			ToCursors::to_cursors(value, s);
		}
	}
}

impl<'a> SemanticEq for BadDeclaration<'a> {
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		self.0.semantic_eq(&other.0, source_text)
	}
}

impl<'a, M: crate::NodeMetadata> crate::NodeWithMetadata<M> for BadDeclaration<'a> {
	fn metadata(&self) -> M {
		M::default()
	}
}
