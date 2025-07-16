use css_ast::CSSInt;
use css_parse::{Parse, Parser, Result as ParserResult, T};
use csskit_derives::{ToCursors, ToSpan, Visitable};

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit(self)]
pub enum FunctionalPseudoElement {
	Specificity(T![::], T![Function], CSSInt, CSSInt, CSSInt, T![')']),
}

impl<'a> Parse<'a> for FunctionalPseudoElement {
	fn parse(_: &mut Parser<'a>) -> ParserResult<Self> {
		todo!();
	}
}
