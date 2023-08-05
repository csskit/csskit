pub mod page;

use crate::{Kind, Parse, Parser, Result, Spanned};

pub struct NoPreludeAllowed;
impl<'a> Parse<'a> for NoPreludeAllowed {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.cur().span;
		parser.expect_without_advance(Kind::LeftCurly)?;
		Ok(Self {}.spanned(span.up_to(&parser.cur().span)))
	}
}
