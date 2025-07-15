use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	AtRule, Build, Parse, Parser, Result as ParserResult, RuleList, T, diagnostics, function_set,
	syntax::CommaSeparated,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

use crate::stylesheet::Rule;

// https://www.w3.org/TR/2012/WD-css3-conditional-20120911/#at-document
#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct DocumentRule<'a> {
	#[visit(skip)]
	pub at_keyword: T![AtKeyword],
	pub matchers: DocumentMatcherList<'a>,
	pub block: DocumentRuleBlock<'a>,
}
// https://drafts.csswg.org/css-page-3/#syntax-page-selector
impl<'a> Parse<'a> for DocumentRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (at_keyword, matchers, block) = Self::parse_at_rule(p)?;
		if let Some(matchers) = matchers {
			Ok(Self { at_keyword, matchers, block })
		} else {
			let c: Cursor = at_keyword.into();
			Err(diagnostics::MissingAtRulePrelude(c.into()))?
		}
	}
}

impl<'a> AtRule<'a> for DocumentRule<'a> {
	const NAME: Option<&'static str> = Some("document");
	type Prelude = DocumentMatcherList<'a>;
	type Block = DocumentRuleBlock<'a>;
}

#[derive(Peek, Parse, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DocumentMatcherList<'a>(pub CommaSeparated<'a, DocumentMatcher>);

function_set!(DocumentMatcherFunctionKeyword {
	Url: "url",
	UrlPrefix: "url-prefix",
	Domain: "domain",
	MediaDocument: "media-document",
	Regexp: "regexp",
});

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
			let keyword = p.parse::<DocumentMatcherFunctionKeyword>()?;
			let c = keyword.into();
			let function = <T![Function]>::build(p, c);
			match keyword {
				DocumentMatcherFunctionKeyword::Url(_) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlFunction(function, string, close))
				}
				DocumentMatcherFunctionKeyword::UrlPrefix(_) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
				DocumentMatcherFunctionKeyword::Domain(_) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
				DocumentMatcherFunctionKeyword::MediaDocument(_) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
				DocumentMatcherFunctionKeyword::Regexp(_) => {
					let string = p.parse::<T![String]>()?;
					let close = p.parse::<T![')']>()?;
					Ok(Self::UrlPrefix(function, string, close))
				}
			}
		}
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct DocumentRuleBlock<'a> {
	#[visit(skip)]
	pub open: T!['{'],
	#[cfg_attr(feature = "serde", serde(borrow))]
	pub rules: Vec<'a, Rule<'a>>,
	#[visit(skip)]
	pub close: Option<T!['}']>,
}

impl<'a> RuleList<'a> for DocumentRuleBlock<'a> {
	type Rule = Rule<'a>;
}

impl<'a> Parse<'a> for DocumentRuleBlock<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let (open, rules, close) = Self::parse_rule_list(p)?;
		Ok(Self { open, rules, close })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DocumentRule>(), 112);
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
