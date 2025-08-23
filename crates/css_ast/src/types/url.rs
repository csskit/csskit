use css_lexer::Cursor;
use css_parse::{Build, Parse, Parser, Peek, Result as ParserResult, T, function_set};
use csskit_derives::{ToCursors, ToSpan, Visitable};

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

function_set!(
	pub enum UrlFunctionKeywords {
		Url: "url",
		Src: "src"
	}
);

impl<'a> Peek<'a> for Url {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Url]>::peek(p, c) || <UrlFunctionKeywords>::peek(p, c)
	}
}

impl<'a> Parse<'a> for Url {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(url) = p.parse_if_peek::<T![Url]>()? {
			return Ok(Self::Url(url));
		}
		let keyword = p.parse::<UrlFunctionKeywords>()?;
		let c: Cursor = keyword.into();
		let function = <T![Function]>::build(p, c);
		match keyword {
			UrlFunctionKeywords::Url(_) => {
				let string = p.parse::<T![String]>()?;
				let close = p.parse::<T![')']>()?;
				Ok(Self::SrcFunction(function, string, close))
			}
			UrlFunctionKeywords::Src(_) => {
				let string = p.parse::<T![String]>()?;
				let close = p.parse::<T![')']>()?;
				Ok(Self::SrcFunction(function, string, close))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Url>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Url, "url('foo')");
		assert_parse!(Url, "url(\"foo\")");
		assert_parse!(Url, "url(foo)");
	}
}
