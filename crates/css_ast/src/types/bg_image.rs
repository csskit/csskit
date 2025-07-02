use css_lexer::Cursor;
use css_parse::{CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics};

use super::Image;

// https://drafts.csswg.org/css-backgrounds/#typedef-bg-image
// <bg-image> = <image> | none
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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
				Err(diagnostics::UnexpectedIdent(p.parse_str(c).into(), ident.into()))?;
			}
			Ok(Self::None(ident))
		}
	}
}

impl ToCursors for BgImage<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::None(ident) => s.append(ident.into()),
			Self::Image(image) => ToCursors::to_cursors(image, s),
		}
	}
}
