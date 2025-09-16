use crate::Syntax;
use css_parse::{
	ComponentValues, Cursor, Diagnostic, Function, Parse, Parser, Peek, Result as ParserResult, T, function_set,
	keyword_set,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

function_set!(pub struct AttrFunctionName "attr");

/// <https://drafts.csswg.org/css-values-5/#attr-notation>
///
/// ```text,ignore
/// attr() = attr( <attr-name> <attr-type>? , <declaration-value>?)
/// <attr-type> = type( <syntax> ) | raw-string | <attr-unit>
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct AttrFunction<'a>(Function<AttrFunctionName, AttrFunctionParams<'a>>);

#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AttrFunctionParams<'a>(AttrName, Option<AttrType>, Option<T![,]>, Option<ComponentValues<'a>>);

// <attr-name> = [ <ident-token>? '|' ]? <ident-token>
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AttrName(pub Option<T![Ident]>, pub Option<T![|]>, pub Option<T![Ident]>);

impl<'a> Peek<'a> for AttrName {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c)
	}
}

impl<'a> Parse<'a> for AttrName {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let a = p.parse_if_peek::<T![Ident]>()?;
		let b = p.parse_if_peek::<T![|]>()?;

		if a.is_some() && b.is_none() {
			return Ok(Self(None, None, a));
		}

		if a.is_none() && b.is_some() {
			return Ok(Self(None, b, Some(p.parse::<T![Ident]>()?)));
		}

		if a.is_none() && b.is_none() {
			Err(Diagnostic::new(p.next(), Diagnostic::expected_ident))?
		}

		debug_assert!(a.is_some() && b.is_some());

		Ok(Self(a, b, Some(p.parse::<T![Ident]>()?)))
	}
}

keyword_set!(pub struct AttrTypeKeywords "raw-string");

function_set!(pub struct AttrTypeFunctionName "type");

// <attr-type> = type( <syntax> ) | raw-string | <attr-unit>
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AttrType {
	Type(Function<AttrTypeFunctionName, Syntax>),
	RawString(T![Ident]),
	Unit(T![DimensionIdent]),
}

impl<'a> Peek<'a> for AttrType {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		AttrTypeKeywords::peek(p, c) || <T![DimensionIdent]>::peek(p, c) || AttrTypeFunctionName::peek(p, c)
	}
}

impl<'a> Parse<'a> for AttrType {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(raw) = p.parse_if_peek::<AttrTypeKeywords>()? {
			return Ok(Self::RawString(raw.into()));
		}
		if let Some(unit) = p.parse_if_peek::<T![DimensionIdent]>()? {
			return Ok(Self::Unit(unit));
		}
		p.parse::<Function<AttrTypeFunctionName, Syntax>>().map(Self::Type)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AttrFunction>(), 160);
	}

	#[test]
	fn test_writes() {
		assert_parse!(AttrFunction, "attr(foo)");
		assert_parse!(AttrFunction, "attr(foo)");
		assert_parse!(AttrFunction, "attr(bar px)");
		assert_parse!(AttrFunction, "attr(foo|bar px)");
		assert_parse!(AttrFunction, "attr(foo|bar)");
		assert_parse!(AttrFunction, "attr(|bar)");
		assert_parse!(AttrFunction, "attr(|bar px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(AttrName, "a|b|c");
		assert_parse_error!(AttrFunction, "attrr(foo)");
		assert_parse_error!(AttrFunction, "attr()");
		assert_parse_error!(AttrFunction, "attr(|)");
	}
}
