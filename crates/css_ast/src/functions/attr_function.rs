use super::prelude::*;
use crate::Syntax;
use css_parse::ComponentValues;

/// <https://drafts.csswg.org/css-values-5/#attr-notation>
///
/// ```text,ignore
/// attr() = attr( <attr-name> <attr-type>? , <declaration-value>?)
/// <attr-type> = type( <syntax> ) | raw-string | <attr-unit>
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct AttrFunction<'a> {
	#[atom(CssAtomSet::Attr)]
	pub name: T![Function],
	pub params: AttrFunctionParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AttrFunctionParams<'a>(AttrName, Option<AttrType>, Option<T![,]>, Option<ComponentValues<'a>>);

// <attr-name> = [ <ident-token>? '|' ]? <ident-token>
#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct AttrName(pub Option<T![Ident]>, pub Option<T![|]>, pub Option<T![Ident]>);

impl<'a> Peek<'a> for AttrName {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Ident]>::peek(p, c)
	}
}

impl<'a> Parse<'a> for AttrName {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
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

// <attr-type> = type( <syntax> ) | raw-string | <attr-unit>
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AttrType {
	#[atom(CssAtomSet::Type)]
	Type(T![Function], Syntax, T![')']),
	#[atom(CssAtomSet::RawString)]
	RawString(T![Ident]),
	Unit(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AttrFunction>(), 152);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(foo)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(foo)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(bar px)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(foo|bar px)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(foo|bar)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(|bar)");
		assert_parse!(CssAtomSet::ATOMS, AttrFunction, "attr(|bar px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AttrName, "a|b|c");
		assert_parse_error!(CssAtomSet::ATOMS, AttrFunction, "attrr(foo)");
		assert_parse_error!(CssAtomSet::ATOMS, AttrFunction, "attr()");
		assert_parse_error!(CssAtomSet::ATOMS, AttrFunction, "attr(|)");
	}
}
