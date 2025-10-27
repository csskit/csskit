use crate::{
	CursorSink, Parse, Parser, Peek, Result as ParserResult, Span, State, T, ToCursors, ToSpan, syntax::ComponentValue,
};
use bumpalo::collections::Vec;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BadDeclaration<'a>(Vec<'a, ComponentValue<'a>>);

// https://drafts.csswg.org/css-syntax-3/#consume-the-remnants-of-a-bad-declaration
impl<'a> Parse<'a> for BadDeclaration<'a> {
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> ParserResult<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let mut values = Vec::new_in(p.bump());
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
			if <T!['}']>::peek(p, c) {
				if p.is(State::Nested) {
					return Ok(Self(values));
				} else {
					p.parse::<T!['}']>()?;
				}
			}

			// anything else
			//
			//     Consume a component value from input, and do nothing.
			//
			values.push(p.parse::<ComponentValue>()?);
		}
	}
}

impl<'a> ToSpan for BadDeclaration<'a> {
	fn to_span(&self) -> Span {
		self.0.to_span()
	}
}

impl<'a> ToCursors for BadDeclaration<'a> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		for value in &self.0 {
			ToCursors::to_cursors(value, s);
		}
	}
}

impl<'a, M: crate::NodeMetadata> crate::NodeWithMetadata<M> for BadDeclaration<'a> {
	fn metadata(&self) -> M {
		M::default()
	}
}
