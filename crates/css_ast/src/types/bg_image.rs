use css_lexer::{Cursor, ToSpan};
use css_parse::{Parse, Parser, Peek, Result as ParserResult, T, diagnostics};
use csskit_derives::ToCursors;

use super::Image;

// https://drafts.csswg.org/css-backgrounds/#typedef-bg-image
// <bg-image> = <image> | none
#[derive(ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub enum BgImage<'a> {
	None(T![Ident]),
	Image(Image<'a>),
}

impl<'a> Peek<'a> for BgImage<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<Image>::peek(p, c) || (<T![Ident]>::peek(p, c) && p.eq_ignore_ascii_case(c, "none"))
	}
}

impl<'a> Parse<'a> for BgImage<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<Image>() {
			let image = p.parse::<Image>()?;
			Ok(Self::Image(image))
		} else {
			let ident = p.parse::<T![Ident]>()?;
			let c: Cursor = ident.into();
			if !p.eq_ignore_ascii_case(c, "none") {
				Err(diagnostics::UnexpectedIdent(p.parse_str(c).into(), ident.to_span()))?;
			}
			Ok(Self::None(ident))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BgImage>(), 216);
	}

	#[test]
	fn test_writes() {
		assert_parse!(BgImage, "none");
		assert_parse!(BgImage, "url(foo)");
	}
}
