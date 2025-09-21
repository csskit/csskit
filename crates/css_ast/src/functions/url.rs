use super::prelude::*;

/// <https://drafts.csswg.org/css-values-4/#url-value>
///
/// ```text
/// <url> = <url()> | <src()>
/// <url()> = url( <string> <url-modifier>* ) | <url-token>
/// <src()> = src( <string> <url-modifier>* )
/// ```
#[derive(ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum Url {
	Url(T![Url]),
	UrlFunction(T![Function], T![String], T![')']),
	SrcFunction(T![Function], T![String], T![')']),
}

impl<'a> Peek<'a> for Url {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Url]>::peek(p, c)
			|| (<T![Function]>::peek(p, c) && matches!(p.to_atom(c), CssAtomSet::Url | CssAtomSet::Src))
	}
}

impl<'a> Parse<'a> for Url {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(url) = p.parse_if_peek::<T![Url]>()? {
			return Ok(Self::Url(url));
		}
		let function = p.parse::<T![Function]>()?;
		let string = p.parse::<T![String]>()?;
		let close = p.parse::<T![')']>()?;

		match p.to_atom::<CssAtomSet>(function.into()) {
			CssAtomSet::Url => Ok(Self::UrlFunction(function, string, close)),
			CssAtomSet::Src => Ok(Self::SrcFunction(function, string, close)),
			_ => Err(Diagnostic::new(function.into(), Diagnostic::unexpected_ident))?,
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Url>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Url, "url('foo')");
		assert_parse!(CssAtomSet::ATOMS, Url, "url(\"foo\")");
		assert_parse!(CssAtomSet::ATOMS, Url, "url(foo)");
	}
}
