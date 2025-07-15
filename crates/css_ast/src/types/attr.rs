use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{Parse, Parser, Peek, Result as ParserResult, T, diagnostics, keyword_set};
use csskit_derives::{ToCursors, ToSpan};

use crate::types::Syntax;

// TODO: Implement DeclarationValue
type DeclarationValue<'a> = Vec<'a, crate::Todo>;

// https://drafts.csswg.org/css-values-5/#attr-notation
// attr() = attr( <attr-name> <attr-type>? , <declaration-value>?)
//<attr-type> = type( <syntax> ) | raw-string | <attr-unit>
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Attr<'a>(
	pub T![Function],
	pub AttrName,
	pub Option<AttrType>,
	pub Option<T![,]>,
	pub Option<DeclarationValue<'a>>,
	pub Option<T![')']>,
);

impl<'a> Peek<'a> for Attr<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "attr")
	}
}

impl<'a> Parse<'a> for Attr<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let func = p.parse::<T![Function]>()?;
		if !p.eq_ignore_ascii_case(func.into(), "attr") {
			let c: Cursor = func.into();
			Err(diagnostics::ExpectedFunctionOf("attr".into(), p.parse_str_lower(c).into(), c.into()))?
		}

		let name = p.parse::<AttrName>()?;
		let attr_type = p.parse_if_peek::<AttrType>()?;
		let comma = p.parse_if_peek::<T![,]>()?;
		let value = if comma.is_some() { p.parse_if_peek::<DeclarationValue<'a>>()? } else { None };
		let close = p.parse_if_peek::<T![')']>()?;

		Ok(Self(func, name, attr_type, comma, value, close))
	}
}

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
			let any = p.parse::<T![Any]>()?;
			let c: Cursor = any.into();
			Err(diagnostics::ExpectedIdent(c.into(), c.into()))?
		}

		debug_assert!(a.is_some() && b.is_some());

		Ok(Self(a, b, Some(p.parse::<T![Ident]>()?)))
	}
}

keyword_set!(pub struct AttrTypeRawString "raw-string");

// <attr-type> = type( <syntax> ) | raw-string | <attr-unit>
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AttrType {
	Type(T![Function], Syntax),
	RawString(AttrTypeRawString),
	Unit(T![DimensionIdent]),
}

impl<'a> Peek<'a> for AttrType {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		AttrTypeRawString::peek(p, c)
			|| <T![DimensionIdent]>::peek(p, c)
			|| (<T![Function]>::peek(p, c) && p.eq_ignore_ascii_case(c, "type"))
	}
}

impl<'a> Parse<'a> for AttrType {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if let Some(func) = p.parse_if_peek::<T![Function]>()? {
			if !p.eq_ignore_ascii_case(func.into(), "type") {
				let c: Cursor = func.into();
				Err(diagnostics::ExpectedFunctionOf("type".into(), p.parse_str_lower(c).into(), c.into()))?
			}
			return Ok(Self::Type(func, p.parse::<Syntax>()?));
		}

		if let Some(raw) = p.parse_if_peek::<AttrTypeRawString>()? {
			return Ok(Self::RawString(raw));
		}

		Ok(Self::Unit(p.parse::<T![DimensionIdent]>()?))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Attr>(), 144);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Attr, "attr(foo)");
		assert_parse!(Attr, "attr(foo)");
		assert_parse!(Attr, "attr(bar px)");
		assert_parse!(Attr, "attr(foo|bar px)");
		assert_parse!(Attr, "attr(foo|bar)");
		assert_parse!(Attr, "attr(|bar)");
		assert_parse!(Attr, "attr(|bar px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Attr, "attr(a|b|c)");
		assert_parse_error!(Attr, "attrr(foo)");
		assert_parse_error!(Attr, "attr()");
		assert_parse_error!(Attr, "attr(foo bar)");
		assert_parse_error!(Attr, "attr(|)");
	}
}
