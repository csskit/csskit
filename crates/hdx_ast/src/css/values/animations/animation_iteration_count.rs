use hdx_atom::atom;
use hdx_lexer::Token;
use hdx_parser::{unexpected, unexpected_ident, Parse, Parser, Result as ParserResult};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;

use crate::{css::values::units::CSSFloat, Value, Writable};
use smallvec::{smallvec, SmallVec};

// https://drafts.csswg.org/css-animations-2/#animation-fill-mode
#[derive(Value, Default, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct AnimationIterationCount(pub SmallVec<[SingleAnimationIterationCount; 1]>);

#[derive(Writable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub enum SingleAnimationIterationCount {
	Infinite, // atom!("infinite")
	Number(CSSFloat),
}

impl Default for SingleAnimationIterationCount {
	fn default() -> Self {
		Self::Number(1.0.into())
	}
}

impl<'a> Parse<'a> for AnimationIterationCount {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut values = smallvec![];
		loop {
			match parser.cur() {
				Token::Ident(ident) => match ident.to_ascii_lowercase() {
					atom!("infinite") => {
						parser.advance();
						values.push(SingleAnimationIterationCount::Infinite);
					}
					atom => unexpected_ident!(parser, atom),
				},
				Token::Number(val, ty) if ty.is_int() && !ty.is_signed() => {
					parser.advance();
					values.push(SingleAnimationIterationCount::Number(val.into()))
				}
				token => unexpected!(parser, token),
			}
			match parser.cur() {
				Token::Comma => {
					parser.advance();
				}
				_ => {
					break;
				}
			}
		}
		Ok(Self(values))
	}
}

impl<'a> WriteCss<'a> for AnimationIterationCount {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.0.iter().peekable();
		while let Some(fill) = iter.next() {
			fill.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_trivia_char(' ')?;
			}
		}
		Ok(())
	}
}

#[cfg(test)]
mod tests {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::test_write;

	#[test]
	fn size_test() {
		use std::mem::size_of;
		assert_eq!(size_of::<AnimationIterationCount>(), 24);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<AnimationIterationCount>(&allocator, "infinite", "infinite");
		test_write::<AnimationIterationCount>(&allocator, "1, infinite, 7, 800", "1,infinite,7,800");
	}
}
