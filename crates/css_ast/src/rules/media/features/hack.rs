use crate::CssAtomSet;
use css_parse::{Cursor, Diagnostic, Parse, Parser, Result as ParserResult, T};
use csskit_derives::{ToCursors, ToSpan};

#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum HackMediaFeature {
	IEBackslashZero(T!['('], T![Ident], T![:], T![Dimension], T![')']),
}

impl<'a> Parse<'a> for HackMediaFeature {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let open = p.parse::<T!['(']>()?;
		let keyword = p.parse::<T![Ident]>()?;
		if !p.equals_atom(keyword.into(), &CssAtomSet::MinWidth) {
			Err(Diagnostic::new(keyword.into(), Diagnostic::expected_ident))?
		}
		let colon = p.parse::<T![:]>()?;
		let dimension = p.parse::<T![Dimension]>()?;
		let c: Cursor = dimension.into();
		let str = p.to_source_cursor(c).source();
		if str != "0\\0" {
			Err(Diagnostic::new(c, Diagnostic::unexpected))?
		}
		let close = p.parse::<T![')']>()?;
		Ok(Self::IEBackslashZero(open, keyword, colon, dimension, close))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<HackMediaFeature>(), 60);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, HackMediaFeature, "(min-width:0\\0)");
	}
}
