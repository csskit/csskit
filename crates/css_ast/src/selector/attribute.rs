use crate::CssAtomSet;
use css_parse::{KindSet, Parse, Parser, Result as ParserResult, T};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, ToSpan};

use super::NamespacePrefix;

#[derive(Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct Attribute {
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub open: T!['['],
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub namespace_prefix: Option<NamespacePrefix>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub attribute: T![Ident],
	pub operator: Option<AttributeOperator>,
	pub value: Option<AttributeValue>,
	pub modifier: Option<AttributeModifier>,
	#[cfg_attr(feature = "visitable", visit(skip))]
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

#[derive(Parse, ToSpan, Peek, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum AttributeOperator {
	Exact(T![=]),
	SpaceList(T![~=]),
	LangPrefix(T![|=]),
	Prefix(T![^=]),
	Suffix(T!["$="]),
	Contains(T![*=]),
}

#[derive(Peek, Parse, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum AttributeValue {
	String(T![String]),
	Ident(T![Ident]),
}

#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[cfg_attr(
	feature = "css_feature_data",
	derive(::csskit_derives::ToCSSFeature),
	css_feature("css.selectors.attribute")
)]
pub enum AttributeModifier {
	#[cfg_attr(feature = "css_feature_data", css_feature("css.selectors.attribute.case_sensitive_modifier"))]
	#[atom(CssAtomSet::S)]
	Sensitive(T![Ident]),
	#[cfg_attr(feature = "css_feature_data", css_feature("css.selectors.attribute.case_insensitive_modifier"))]
	#[atom(CssAtomSet::I)]
	Insensitive(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
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
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[foo]");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[foo='bar']");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[foo=\"bar\"]");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[foo='bar']");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[attr*='foo']");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[attr='foo']");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[*|attr='foo']");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[x|attr='foo']");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[attr|='foo']");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[attr|=foo i]");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[attr|=foo s]");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[attr|='foo'i]");
		assert_parse!(CssAtomSet::ATOMS, Attribute, "[attr|='foo's]");
	}

	#[cfg(feature = "css_feature_data")]
	#[test]
	fn test_feature_data() {
		use crate::assert_feature_id;
		assert_feature_id!("i", AttributeModifier, "css.selectors.attribute.case_insensitive_modifier");
		assert_feature_id!("s", AttributeModifier, "css.selectors.attribute.case_sensitive_modifier");
	}
}
