use css_lexer::{Cursor, KindSet};
use css_parse::{Build, Parse, Parser, Peek, Result as ParserResult, T};
use csskit_derives::{IntoCursor, IntoSpan, Peek, ToCursors};
use csskit_proc_macro::visit;

use crate::{Visit, Visitable};

use super::NamespacePrefix;

#[derive(IntoSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct Attribute {
	pub open: T!['['],
	pub namespace_prefix: Option<NamespacePrefix>,
	pub attribute: T![Ident],
	pub operator: Option<AttributeOperator>,
	pub value: Option<AttributeValue>,
	pub modifier: Option<AttributeModifier>,
	pub close: Option<T![']']>,
}

impl<'a> Parse<'a> for Attribute {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let open = p.parse::<T!['[']>()?;
		let mut namespace_prefix = if p.peek::<T![*|]>() { Some(p.parse::<NamespacePrefix>()?) } else { None };
		let mut attribute = p.parse::<T![Ident]>()?;
		let skip = p.set_skip(KindSet::NONE);
		// namespace_prefix might be `<Ident> '|' <Ident>`
		if namespace_prefix.is_none() && p.peek::<T![|]>() && !p.peek::<T![|=]>() {
			let pipe = p.parse::<T![|]>();
			let ident = p.parse::<T![Ident]>();
			p.set_skip(skip);
			namespace_prefix = Some(NamespacePrefix::Name(attribute, pipe?));
			attribute = ident?;
		}
		p.set_skip(skip);
		let operator = p.parse_if_peek::<AttributeOperator>()?;
		let value = if operator.is_some() { Some(p.parse::<AttributeValue>()?) } else { None };
		let modifier =
			if value.is_some() && p.peek::<AttributeModifier>() { Some(p.parse::<AttributeModifier>()?) } else { None };
		let close = p.parse_if_peek::<T![']']>()?;
		Ok(Self { open, namespace_prefix, attribute, operator, value, modifier, close })
	}
}

impl<'a> Visitable<'a> for Attribute {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_attribute(self);
	}
}

#[derive(IntoSpan, Peek, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum AttributeOperator {
	Exact(T![=]),
	SpaceList(T![~=]),
	LangPrefix(T![|=]),
	Prefix(T![^=]),
	Suffix(T!["$="]),
	Contains(T![*=]),
}

impl<'a> Parse<'a> for AttributeOperator {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let c = p.peek_n(1);
		if <T![=]>::peek(p, c) {
			p.parse::<T![=]>().map(AttributeOperator::Exact)
		} else if <T![~=]>::peek(p, c) {
			p.parse::<T![~=]>().map(AttributeOperator::SpaceList)
		} else if <T![|=]>::peek(p, c) {
			p.parse::<T![|=]>().map(AttributeOperator::LangPrefix)
		} else if <T![^=]>::peek(p, c) {
			p.parse::<T![^=]>().map(AttributeOperator::Prefix)
		} else if <T!["$="]>::peek(p, c) {
			p.parse::<T!["$="]>().map(AttributeOperator::Suffix)
		} else {
			p.parse::<T![*=]>().map(AttributeOperator::Contains)
		}
	}
}

#[derive(Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
pub enum AttributeValue {
	String(T![String]),
	Ident(T![Ident]),
}

impl<'a> Build<'a> for AttributeValue {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![Ident]>::peek(p, c) {
			Self::Ident(<T![Ident]>::build(p, c))
		} else {
			Self::String(<T![String]>::build(p, c))
		}
	}
}

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum AttributeModifier {
	Sensitive(T![Ident]),
	Insensitive(T![Ident]),
}

impl<'a> Peek<'a> for AttributeModifier {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c) && matches!(p.parse_str(c), "i" | "s" | "I" | "S")
	}
}

impl<'a> Build<'a> for AttributeModifier {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if matches!(p.parse_str(c), "s" | "S") {
			Self::Sensitive(<T![Ident]>::build(p, c))
		} else {
			Self::Insensitive(<T![Ident]>::build(p, c))
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Attribute>(), 128);
		assert_eq!(std::mem::size_of::<AttributeOperator>(), 28);
		assert_eq!(std::mem::size_of::<AttributeModifier>(), 16);
		assert_eq!(std::mem::size_of::<AttributeValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Attribute, "[foo]");
		assert_parse!(Attribute, "[foo='bar']");
		assert_parse!(Attribute, "[foo=\"bar\"]");
		assert_parse!(Attribute, "[foo='bar']");
		assert_parse!(Attribute, "[attr*='foo']");
		assert_parse!(Attribute, "[attr='foo']");
		assert_parse!(Attribute, "[*|attr='foo']");
		assert_parse!(Attribute, "[x|attr='foo']");
		assert_parse!(Attribute, "[attr|='foo']");
		assert_parse!(Attribute, "[attr|=foo i]");
		assert_parse!(Attribute, "[attr|=foo s]");
		assert_parse!(Attribute, "[attr|='foo'i]");
		assert_parse!(Attribute, "[attr|='foo's]");
	}
}
