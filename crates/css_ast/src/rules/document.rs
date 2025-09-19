use super::prelude::*;

atkeyword_set!(pub struct AtDocumentKeyword "document");

// https://www.w3.org/TR/2012/WD-css3-conditional-20120911/#at-document
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct DocumentRule<'a>(pub AtRule<AtDocumentKeyword, DocumentMatcherList<'a>, DocumentRuleBlock<'a>>);

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DocumentMatcherList<'a>(pub CommaSeparated<'a, DocumentMatcher>);

function_set!(
	pub enum DocumentMatcherFunctionKeyword {
		Url: "url",
		UrlPrefix: "url-prefix",
		Domain: "domain",
		MediaDocument: "media-document",
		Regexp: "regexp",
	}
);

#[derive(Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum DocumentMatcher {
	Url(T![Url]),
	UrlFunction(T![Function], T![String], T![')']),
	UrlPrefix(T![Function], T![String], T![')']),
	Domain(T![Function], T![String], T![')']),
	MediaDocument(T![Function], T![String], T![')']),
	Regexp(T![Function], T![String], T![')']),
}

impl<'a> Parse<'a> for DocumentMatcher {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<T![Url]>() {
			Ok(Self::Url(p.parse::<T![Url]>()?))
		} else {
			match p.parse::<DocumentMatcherFunctionKeyword>()? {
				DocumentMatcherFunctionKeyword::Url(function) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlFunction(function, string, close))
				}
				DocumentMatcherFunctionKeyword::UrlPrefix(function) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
				DocumentMatcherFunctionKeyword::Domain(function) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
				DocumentMatcherFunctionKeyword::MediaDocument(function) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
				DocumentMatcherFunctionKeyword::Regexp(function) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
			}
		}
	}
}

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DocumentRuleBlock<'a>(RuleList<'a, Rule<'a>>);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DocumentRule>(), 128);
		assert_eq!(std::mem::size_of::<DocumentMatcher>(), 40);
		assert_eq!(std::mem::size_of::<DocumentRuleBlock>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(DocumentRule, r#"@document url("http://www.w3.org"){}"#);
		assert_parse!(DocumentRule, r#"@document domain("mozilla.org"){}"#);
		assert_parse!(DocumentRule, r#"@document url-prefix("http://www.w3.org/Style/"){}"#);
		assert_parse!(DocumentRule, r#"@document media-document("video"){}"#);
		assert_parse!(DocumentRule, r#"@document regexp("https:.*"){}"#);
		assert_parse!(
			DocumentRule,
			r#"@document url(http://www.w3.org),url-prefix("http://www.w3.org/Style/"),domain("mozilla.org"){}"#
		);
		assert_parse!(
			DocumentRule,
			r#"@document url(http://www.w3.org),url-prefix("http://www.w3.org/Style/"),domain("mozilla.org"){body{color:black}}"#
		);
	}
}
