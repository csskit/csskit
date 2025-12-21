use crate::{CsskitAtomSet, diagnostics::QueryDiagnostic};
use bumpalo::collections::Vec;
use css_ast::{Nth, visit::NodeId};
use css_parse::{
	CompoundSelector as CompoundSelectorTrait, Cursor, CursorSink, Diagnostic, Parse, Parser, Peek, Result,
	SelectorComponent as SelectorComponentTrait, T, ToCursors, pseudo_class, syntax::CommaSeparated,
};

#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QuerySelectorList<'a>(pub CommaSeparated<'a, QueryCompoundSelector<'a>>);

impl<'a> QuerySelectorList<'a> {
	pub fn selectors(&self) -> impl Iterator<Item = &QueryCompoundSelector<'a>> {
		(&self.0).into_iter().map(|(item, _comma)| item)
	}
}

#[derive(csskit_derives::Peek, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryCompoundSelector<'a>(pub Vec<'a, QuerySelectorComponent<'a>>);

impl<'a> QueryCompoundSelector<'a> {
	pub fn parts(&self) -> &[QuerySelectorComponent<'a>] {
		&self.0
	}
}

impl<'a> CompoundSelectorTrait<'a> for QueryCompoundSelector<'a> {
	type SelectorComponent = QuerySelectorComponent<'a>;
}

impl<'a> Parse<'a> for QueryCompoundSelector<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self(Self::parse_compound_selector(p)?))
	}
}

/// Selector components (type, wildcard, attribute, combinator, pseudo-class).
#[derive(csskit_derives::Peek, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub enum QuerySelectorComponent<'a> {
	Type(QueryType),
	Wildcard(QueryWildcard),
	Attribute(QueryAttribute),
	Combinator(QueryCombinator),
	PseudoClass(QueryPseudoClass),
	FunctionalPseudoClass(QueryFunctionalPseudoClass<'a>),
}

impl<'a> Parse<'a> for QuerySelectorComponent<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_selector_component(p)
	}
}

/// Placeholder for unsupported CSS selector features.
pub struct NeverMatch;

impl<'a> Peek<'a> for NeverMatch {
	fn peek<I>(_p: &Parser<'a, I>, _c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		false
	}
}

impl<'a> Parse<'a> for NeverMatch {
	fn parse<I>(_p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		unreachable!("NeverMatch should never be parsed")
	}
}

impl ToCursors for NeverMatch {
	fn to_cursors(&self, _s: &mut impl CursorSink) {
		unreachable!("NeverMatch should never be serialized")
	}
}

impl<'a> SelectorComponentTrait<'a> for QuerySelectorComponent<'a> {
	type Wildcard = QueryWildcard;
	type Id = NeverMatch;
	type Type = QueryType;
	type PseudoClass = QueryPseudoClass;
	type PseudoElement = NeverMatch;
	type LegacyPseudoElement = NeverMatch;
	type Class = NeverMatch;
	type NsType = NeverMatch;
	type Combinator = QueryCombinator;
	type Attribute = QueryAttribute;
	type FunctionalPseudoClass = QueryFunctionalPseudoClass<'a>;
	type FunctionalPseudoElement = NeverMatch;

	fn build_wildcard(node: QueryWildcard) -> Self {
		Self::Wildcard(node)
	}

	fn build_id(_node: NeverMatch) -> Self {
		unreachable!()
	}

	fn build_class(_node: NeverMatch) -> Self {
		unreachable!()
	}

	fn build_type(node: QueryType) -> Self {
		Self::Type(node)
	}

	fn build_pseudo_class(node: QueryPseudoClass) -> Self {
		Self::PseudoClass(node)
	}

	fn build_pseudo_element(_node: NeverMatch) -> Self {
		unreachable!()
	}

	fn build_legacy_pseudo_element(_node: NeverMatch) -> Self {
		unreachable!()
	}

	fn build_ns_type(_node: NeverMatch) -> Self {
		unreachable!()
	}

	fn build_combinator(node: QueryCombinator) -> Self {
		Self::Combinator(node)
	}

	fn build_attribute(node: QueryAttribute) -> Self {
		Self::Attribute(node)
	}

	fn build_functional_pseudo_class(node: QueryFunctionalPseudoClass<'a>) -> Self {
		Self::FunctionalPseudoClass(node)
	}

	fn build_functional_pseudo_element(_node: NeverMatch) -> Self {
		unreachable!()
	}
}

/// Type selector validated against [`NodeId`].
#[derive(csskit_derives::Peek, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryType(pub T![Ident]);

impl QueryType {
	pub fn node_id(&self, source: &str) -> NodeId {
		let c: Cursor = self.0.into();
		// Safe: we validated during parsing
		NodeId::from_tag_name(c.str_slice(source)).unwrap()
	}
}

impl<'a> Parse<'a> for QueryType {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		let name = p.to_source_cursor(c).source();
		if NodeId::from_tag_name(name).is_none() {
			Err(Diagnostic::new(c, Diagnostic::unknown_node_type))?;
		}
		Ok(Self(p.parse::<T![Ident]>()?))
	}
}

/// Universal selector (`*`).
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryWildcard(pub T![*]);

/// Combinator (`>`, `+`, `~`, or descendant).
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryCombinator {
	Child(T![>]),
	NextSibling(T![+]),
	SubsequentSibling(T![~]),
	Descendant(T![' ']),
}

/// Attribute selector (`[name=value]`).
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryAttribute {
	pub open: T!['['],
	pub attr_name: T![Ident],
	pub eq: T![=],
	pub value: QueryAttributeValue,
	pub close: Option<T![']']>,
}

#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryAttributeValue {
	String(T![String]),
	Ident(T![Ident]),
}

impl QueryAttribute {
	/// Get the attribute name
	pub fn attr_name<'a>(&self, source: &'a str) -> &'a str {
		let c: Cursor = self.attr_name.into();
		c.str_slice(source)
	}

	/// Get the attribute value (without quotes for strings)
	pub fn attr_value<'a>(&self, source: &'a str) -> &'a str {
		match self.value {
			QueryAttributeValue::String(t) => {
				let c: Cursor = t.into();
				let raw = c.str_slice(source);
				// Strip quotes
				&raw[1..raw.len() - 1]
			}
			QueryAttributeValue::Ident(t) => {
				let c: Cursor = t.into();
				c.str_slice(source)
			}
		}
	}
}

// Non-functional pseudo-classes (`:important`, `:custom`, etc.).
pseudo_class!(
	#[derive(csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
	pub enum QueryPseudoClass {
		AtRule: CsskitAtomSet::AtRule,
		Computed: CsskitAtomSet::Computed,
		Custom: CsskitAtomSet::Custom,
		Empty: CsskitAtomSet::Empty,
		FirstChild: CsskitAtomSet::FirstChild,
		FirstOfType: CsskitAtomSet::FirstOfType,
		Function: CsskitAtomSet::Function,
		Important: CsskitAtomSet::Important,
		LastChild: CsskitAtomSet::LastChild,
		LastOfType: CsskitAtomSet::LastOfType,
		Longhand: CsskitAtomSet::Longhand,
		Nested: CsskitAtomSet::Nested,
		OnlyChild: CsskitAtomSet::OnlyChild,
		OnlyOfType: CsskitAtomSet::OnlyOfType,
		Prefixed: CsskitAtomSet::Prefixed,
		Root: CsskitAtomSet::Root,
		Rule: CsskitAtomSet::Rule,
		Shorthand: CsskitAtomSet::Shorthand,
		Unknown: CsskitAtomSet::Unknown,
	}
);

/// Functional pseudo-classes (`:not()`, `:nth-child()`, etc.).
#[derive(csskit_derives::Peek, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub enum QueryFunctionalPseudoClass<'a> {
	Not(QueryNotPseudo<'a>),
	NthChild(QueryNthPseudo),
	NthLastChild(QueryNthPseudo),
	NthOfType(QueryNthPseudo),
	NthLastOfType(QueryNthPseudo),
	PropertyType(QueryPropertyTypePseudo),
	Prefixed(QueryPrefixedPseudo),
}

impl<'a> Parse<'a> for QueryFunctionalPseudoClass<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(2);
		let atom = p.to_atom::<CsskitAtomSet>(c);

		match atom {
			CsskitAtomSet::Not => p.parse::<QueryNotPseudo<'a>>().map(Self::Not),
			CsskitAtomSet::NthChild => p.parse::<QueryNthPseudo>().map(Self::NthChild),
			CsskitAtomSet::NthLastChild => p.parse::<QueryNthPseudo>().map(Self::NthLastChild),
			CsskitAtomSet::NthOfType => p.parse::<QueryNthPseudo>().map(Self::NthOfType),
			CsskitAtomSet::NthLastOfType => p.parse::<QueryNthPseudo>().map(Self::NthLastOfType),
			CsskitAtomSet::PropertyType => p.parse::<QueryPropertyTypePseudo>().map(Self::PropertyType),
			CsskitAtomSet::Prefixed => p.parse::<QueryPrefixedPseudo>().map(Self::Prefixed),
			_ => Err(Diagnostic::new(c, Diagnostic::unknown_functional_pseudo_class))?,
		}
	}
}

/// `:not(<selector>)` pseudo-class.
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryNotPseudo<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub selector: QueryCompoundSelector<'a>,
	pub close: Option<T![')']>,
}

/// `:nth-child()`, `:nth-last-child()`, `:nth-of-type()`, `:nth-last-of-type()` pseudo-classes.
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryNthPseudo {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth,
	pub close: Option<T![')']>,
}

/// `:property-type(<group>)` pseudo-class.
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryPropertyTypePseudo {
	pub colon: T![:],
	pub function: T![Function],
	pub group: T![Ident],
	pub close: Option<T![')']>,
}

/// `:prefixed(<vendor>)` pseudo-class.
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryPrefixedPseudo {
	pub colon: T![:],
	pub function: T![Function],
	pub vendor: T![Ident],
	pub close: Option<T![')']>,
}

#[cfg(test)]
mod tests {
	use super::{QueryCompoundSelector, QuerySelectorList};
	use crate::CsskitAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_parse_simple_type() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "style-rule");
	}

	#[test]
	fn test_parse_universal() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "*");
	}

	#[test]
	fn test_parse_pseudo_class() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "*:important");
	}

	#[test]
	fn test_parse_descendant() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "style-rule *:important");
	}

	#[test]
	fn test_parse_child() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "style-rule > *:important");
	}

	#[test]
	fn test_parse_list() {
		assert_parse!(CsskitAtomSet::ATOMS, QuerySelectorList, "style-rule,media-rule");
	}

	#[test]
	fn test_parse_not() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "*:not(:important)");
	}

	#[test]
	fn test_parse_attribute_selector() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "[name=color]");
	}

	#[test]
	fn test_parse_attribute_selector_quoted() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "[name='background-color']");
	}

	#[test]
	fn test_parse_attribute_selector_double_quoted() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "[name=\"margin-top\"]");
	}

	#[test]
	fn test_parse_attribute_any_attr_name() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "[foo=bar]");
	}

	#[test]
	fn test_parse_universal_with_attribute() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "*[name=color]");
	}

	#[test]
	fn test_parse_just_pseudo_class() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, ":important");
	}

	#[test]
	fn test_parse_attribute_with_pseudo() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "[name=color]:important");
	}
}
