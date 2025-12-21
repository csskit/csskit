use crate::{CsskitAtomSet, diagnostics::QueryDiagnostic};
use bumpalo::collections::Vec;
use css_ast::{AttributeOperator, Nth, PropertyGroup, PropertyKind, visit::NodeId};
use css_parse::{
	AtomSet, CompoundSelector as CompoundSelectorTrait, Cursor, CursorSink, Diagnostic, NodeMetadata, NodeWithMetadata,
	Parse, Parser, Peek, Result, SelectorComponent as SelectorComponentTrait, T, ToCursors, pseudo_class,
	syntax::CommaSeparated,
};

use super::metadata::{QuerySelectorMetadata, SelectorRequirements, SelectorStructure};

#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QuerySelectorList<'a>(pub CommaSeparated<'a, QueryCompoundSelector<'a>>);

impl<'a> QuerySelectorList<'a> {
	pub fn selectors(&self) -> impl Iterator<Item = &QueryCompoundSelector<'a>> {
		(&self.0).into_iter().map(|(item, _comma)| item)
	}
}

#[derive(csskit_derives::Peek, Debug, Clone, PartialEq, Eq)]
pub struct QueryCompoundSelector<'a> {
	parts: Vec<'a, QuerySelectorComponent<'a>>,
	/// Precomputed metadata about this selector.
	metadata: QuerySelectorMetadata,
}

impl<'a> QueryCompoundSelector<'a> {
	pub fn parts(&self) -> &[QuerySelectorComponent<'a>] {
		&self.parts
	}

	/// Get the precomputed metadata for this selector.
	pub fn metadata(&self) -> QuerySelectorMetadata {
		self.metadata
	}
}

impl ToCursors for QueryCompoundSelector<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		self.parts.to_cursors(s);
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
		let parts = Self::parse_compound_selector(p)?;

		let mut metadata = QuerySelectorMetadata::default();
		for part in &parts {
			metadata = metadata.merge(part.self_metadata());
		}

		// Compute rightmost_type_id: scan backwards from end to first combinator
		// Only set if there's a type and no wildcard in the rightmost simple selector
		let mut rightmost_type_id = None;
		for part in parts.iter().rev() {
			match part {
				QuerySelectorComponent::Combinator(_) => break,
				QuerySelectorComponent::Type(t) => {
					if rightmost_type_id.is_none() {
						rightmost_type_id = Some(t.node_id);
					}
				}
				QuerySelectorComponent::Wildcard(_) => {
					rightmost_type_id = None;
					break;
				}
				_ => {}
			}
		}
		metadata.rightmost_type_id = rightmost_type_id;

		Ok(Self { parts, metadata })
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

impl<'a> NodeWithMetadata<QuerySelectorMetadata> for QuerySelectorComponent<'a> {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		match self {
			Self::Type(t) => t.self_metadata(),
			Self::Wildcard(w) => w.self_metadata(),
			Self::Attribute(a) => a.self_metadata(),
			Self::Combinator(c) => c.self_metadata(),
			Self::PseudoClass(p) => p.self_metadata(),
			Self::FunctionalPseudoClass(f) => f.self_metadata(),
		}
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
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
/// The NodeId is pre-computed at parse time for efficient matching.
#[derive(csskit_derives::Peek, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryType {
	pub ident: T![Ident],
	/// Pre-computed NodeId for fast matching.
	pub node_id: NodeId,
}

impl ToCursors for QueryType {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.ident.into());
	}
}

impl<'a> Parse<'a> for QueryType {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		let name = p.to_source_cursor(c).source();
		let Some(node_id) = NodeId::from_tag_name(name) else {
			Err(Diagnostic::new(c, Diagnostic::unknown_node_type))?
		};
		Ok(Self { ident: p.parse::<T![Ident]>()?, node_id })
	}
}

impl NodeWithMetadata<QuerySelectorMetadata> for QueryType {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		QuerySelectorMetadata {
			structure: SelectorStructure::HasType,
			rightmost_type_id: Some(self.node_id),
			..Default::default()
		}
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
	}
}

/// Universal selector (`*`).
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryWildcard(pub T![*]);

impl NodeWithMetadata<QuerySelectorMetadata> for QueryWildcard {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		QuerySelectorMetadata { structure: SelectorStructure::HasWildcard, ..Default::default() }
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
	}
}

/// Combinator (`>`, `+`, `~`, or descendant).
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryCombinator {
	Child(T![>]),
	NextSibling(T![+]),
	SubsequentSibling(T![~]),
	Descendant(T![' ']),
}

impl NodeWithMetadata<QuerySelectorMetadata> for QueryCombinator {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		QuerySelectorMetadata { structure: SelectorStructure::HasCombinator, ..Default::default() }
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
	}
}

/// Attribute selector (`[name]` or `[name=value]`).
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryAttribute {
	pub open: T!['['],
	pub attr_name: T![Ident],
	pub matcher: Option<QueryAttributeMatcher>,
	pub close: Option<T![']']>,
}

#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryAttributeMatcher {
	pub operator: AttributeOperator,
	pub value: QueryAttributeValue,
}

#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryAttributeValue {
	String(T![String]),
	Ident(T![Ident]),
}

impl QueryAttribute {
	/// Returns the attribute name atom.
	pub fn attr_name_atom(&self) -> CsskitAtomSet {
		let c: Cursor = self.attr_name.into();
		CsskitAtomSet::from_bits(c.atom_bits())
	}

	/// Returns the attribute operator, or None for presence-only selectors like `[name]`.
	pub fn operator(&self) -> Option<&AttributeOperator> {
		self.matcher.as_ref().map(|m| &m.operator)
	}

	/// Returns the attribute value, or None for presence-only selectors like `[name]`.
	pub fn attr_value<'a>(&self, source: &'a str) -> Option<&'a str> {
		self.matcher.as_ref().map(|m| match m.value {
			QueryAttributeValue::String(t) => {
				let c: Cursor = t.into();
				let raw = c.str_slice(source);
				&raw[1..raw.len() - 1]
			}
			QueryAttributeValue::Ident(t) => {
				let c: Cursor = t.into();
				c.str_slice(source)
			}
		})
	}
}

impl NodeWithMetadata<QuerySelectorMetadata> for QueryAttribute {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		let cursor: Cursor = self.attr_name.into();
		let atom = CsskitAtomSet::from_bits(cursor.atom_bits());
		let attribute_filter = atom.to_property_kind().unwrap_or(PropertyKind::none());
		QuerySelectorMetadata { structure: SelectorStructure::HasAttribute, attribute_filter, ..Default::default() }
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
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

impl NodeWithMetadata<QuerySelectorMetadata> for QueryPseudoClass {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		match self {
			Self::Important(..) => {
				QuerySelectorMetadata { requirements: SelectorRequirements::Important, ..Default::default() }
			}
			Self::Custom(..) => {
				QuerySelectorMetadata { requirements: SelectorRequirements::Custom, ..Default::default() }
			}
			Self::Computed(..) => {
				QuerySelectorMetadata { requirements: SelectorRequirements::Computed, ..Default::default() }
			}
			Self::Shorthand(..) => {
				QuerySelectorMetadata { requirements: SelectorRequirements::Shorthand, ..Default::default() }
			}
			Self::Longhand(..) => {
				QuerySelectorMetadata { requirements: SelectorRequirements::Longhand, ..Default::default() }
			}
			Self::Unknown(..) => {
				QuerySelectorMetadata { requirements: SelectorRequirements::Unknown, ..Default::default() }
			}
			Self::Prefixed(..) => {
				QuerySelectorMetadata { requirements: SelectorRequirements::Prefixed, ..Default::default() }
			}
			Self::Rule(..) => QuerySelectorMetadata { requirements: SelectorRequirements::Rule, ..Default::default() },
			Self::AtRule(..) => {
				QuerySelectorMetadata { requirements: SelectorRequirements::AtRule, ..Default::default() }
			}
			Self::OnlyChild(..) | Self::LastChild(..) => QuerySelectorMetadata { deferred: true, ..Default::default() },
			Self::Empty(..) => QuerySelectorMetadata { deferred: true, has_empty: true, ..Default::default() },
			Self::FirstOfType(..) | Self::LastOfType(..) | Self::OnlyOfType(..) => {
				QuerySelectorMetadata { deferred: true, needs_type_tracking: true, ..Default::default() }
			}
			_ => QuerySelectorMetadata::default(),
		}
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
	}
}

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

impl<'a> NodeWithMetadata<QuerySelectorMetadata> for QueryFunctionalPseudoClass<'a> {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		match self {
			Self::NthLastChild(_) => QuerySelectorMetadata { deferred: true, ..Default::default() },
			Self::NthOfType(_) | Self::NthLastOfType(_) => {
				QuerySelectorMetadata { deferred: true, needs_type_tracking: true, ..Default::default() }
			}
			Self::PropertyType(p) => {
				let cursor: Cursor = p.group.into();
				let atom = CsskitAtomSet::from_bits(cursor.atom_bits());
				let property_groups = atom.to_property_group().unwrap_or(PropertyGroup::none());
				QuerySelectorMetadata { property_groups, ..Default::default() }
			}
			Self::Prefixed(_) => {
				QuerySelectorMetadata { requirements: SelectorRequirements::Prefixed, ..Default::default() }
			}
			_ => QuerySelectorMetadata::default(),
		}
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
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
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryPropertyTypePseudo {
	pub colon: T![:],
	pub function: T![Function],
	pub group: T![Ident],
	pub close: Option<T![')']>,
}

/// `:prefixed(<vendor>)` pseudo-class.
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
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

	#[test]
	fn test_parse_attribute_prefix_operator() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "[name^=background]");
	}

	#[test]
	fn test_parse_attribute_suffix_operator() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "[name$=color]");
	}

	#[test]
	fn test_parse_attribute_contains_operator() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "[name*=margin]");
	}

	#[test]
	fn test_parse_attribute_spacelist_operator() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "[name~=value]");
	}

	#[test]
	fn test_parse_attribute_langprefix_operator() {
		assert_parse!(CsskitAtomSet::ATOMS, QueryCompoundSelector, "[name|=en]");
	}
}
