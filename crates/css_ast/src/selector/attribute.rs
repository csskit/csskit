use css_parse::{Cursor, Diagnostic, KindSet, Parse, Parser, Peek, Result as ParserResult, T};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, ToSpan, Visitable};

use super::NamespacePrefix;

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct Attribute {
	#[visit(skip)]
	pub open: T!['['],
	#[visit(skip)]
	pub namespace_prefix: Option<NamespacePrefix>,
	#[visit(skip)]
	pub attribute: T![Ident],
	pub operator: Option<AttributeOperator>,
	pub value: Option<AttributeValue>,
	pub modifier: Option<AttributeModifier>,
	#[visit(skip)]
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

#[derive(ToSpan, Peek, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
#[visit(self)]
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

#[derive(Peek, Parse, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", content = "value"))]
#[visit(self)]
pub enum AttributeValue {
	String(T![String]),
	Ident(T![Ident]),
}

#[derive(ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(
	feature = "css_feature_data",
	derive(::csskit_derives::ToCSSFeature),
	css_feature("css.selectors.attribute")
)]
#[visit(self)]
pub enum AttributeModifier {
	#[cfg_attr(feature = "css_feature_data", css_feature("css.selectors.attribute.case_sensitive_modifier"))]
	Sensitive(T![Ident]),
	#[cfg_attr(feature = "css_feature_data", css_feature("css.selectors.attribute.case_insensitive_modifier"))]
	Insensitive(T![Ident]),
}

impl<'a> Peek<'a> for AttributeModifier {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c) && matches!(p.parse_str(c), "i" | "s" | "I" | "S")
	}
}

impl<'a> Parse<'a> for AttributeModifier {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<Self>() {
			let c = p.peek_n(1);
			if matches!(p.parse_str(c), "s" | "S") {
				Ok(Self::Sensitive(p.parse::<T![Ident]>()?))
			} else {
				Ok(Self::Insensitive(p.parse::<T![Ident]>()?))
			}
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
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

	#[cfg(feature = "css_feature_data")]
	#[test]
	fn test_feature_data() {
		use crate::assert_feature_id;
		assert_feature_id!("i", AttributeModifier, "css.selectors.attribute.case_insensitive_modifier");
		assert_feature_id!("s", AttributeModifier, "css.selectors.attribute.case_sensitive_modifier");
	}
}
