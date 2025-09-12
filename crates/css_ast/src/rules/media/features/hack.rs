use crate::diagnostics;
use css_parse::{Cursor, Parse, Parser, Result as ParserResult, T};
use csskit_derives::{ToCursors, ToSpan};

#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum HackMediaFeature {
	IEBackslashZero(T!['('], T![Ident], T![:], T![Dimension], T![')']),
}

impl<'a> Parse<'a> for HackMediaFeature {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let open = p.parse::<T!['(']>()?;
		let keyword = p.parse::<T![Ident]>()?;
		if !p.eq_ignore_ascii_case(keyword.into(), "min-width") {
			let source_cursor = p.to_source_cursor(keyword.into());
			Err(diagnostics::UnexpectedIdent(source_cursor.to_string(), keyword.into()))?
		}
		let colon = p.parse::<T![:]>()?;
		let dimension = p.parse::<T![Dimension]>()?;
		let c: Cursor = dimension.into();
		let str = p.parse_raw_str(c);
		if str != "0\\0" {
			Err(diagnostics::Unexpected(c))?
		}
		let close = p.parse::<T![')']>()?;
		Ok(Self::IEBackslashZero(open, keyword, colon, dimension, close))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<HackMediaFeature>(), 60);
	}

	#[test]
	fn test_writes() {
		assert_parse!(HackMediaFeature, "(min-width:0\\0)");
	}
}
