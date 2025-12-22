use crate::{CsskitAtomSet, diagnostics::QueryDiagnostic};
use bumpalo::collections::Vec;
use css_ast::{AttributeOperator, Nth, PropertyGroup, PropertyKind, VendorPrefixes, visit::NodeId};
use css_parse::{
	AtomSet, CompoundSelector as CompoundSelectorTrait, Cursor, CursorSink, Diagnostic, NodeMetadata, NodeWithMetadata,
	Parse, Parser, Peek, Result, SelectorComponent as SelectorComponentTrait, T, ToCursors, pseudo_class,
	syntax::CommaSeparated,
};
use smallvec::SmallVec;

use super::metadata::{QuerySelectorMetadata, SelectorRequirements, SelectorStructure};

/// A pre-split segment of a compound selector: a combinator followed by simple selector parts.
/// The first segment has `combinator = None`, subsequent segments have the combinator that
/// precedes them (stored in the segment to the right).
#[derive(Debug, Clone, Copy)]
pub struct SelectorSegment {
	/// The combinator before this segment (None for the rightmost segment).
	pub combinator: Option<QueryCombinator>,
	/// Start index into the parent's parts array.
	pub start: u16,
	/// End index (exclusive) into the parent's parts array.
	pub end: u16,
	/// Precomputed NodeId for the type selector in this segment (if any, and no wildcard).
	pub type_id: Option<NodeId>,
}

impl SelectorSegment {
	/// Get the simple selector parts for this segment from the parent's parts array.
	#[inline]
	pub fn parts<'p, 'b>(&self, all_parts: &'p [QuerySelectorComponent<'b>]) -> &'p [QuerySelectorComponent<'b>] {
		&all_parts[self.start as usize..self.end as usize]
	}
}

#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QuerySelectorList<'a>(pub CommaSeparated<'a, QueryCompoundSelector<'a>>);

impl<'a> QuerySelectorList<'a> {
	pub fn selectors(&self) -> impl Iterator<Item = &QueryCompoundSelector<'a>> {
		(&self.0).into_iter().map(|(item, _comma)| item)
	}
}

#[derive(csskit_derives::Peek, Debug, Clone)]
pub struct QueryCompoundSelector<'a> {
	parts: Vec<'a, QuerySelectorComponent<'a>>,
	/// Precomputed metadata about this selector.
	metadata: QuerySelectorMetadata,
	/// Pre-split segments for efficient matching. Stored in reverse order (rightmost first).
	/// Most selectors have 1-3 segments, so SmallVec avoids allocation.
	segments: SmallVec<[SelectorSegment; 4]>,
}

impl<'a> PartialEq for QueryCompoundSelector<'a> {
	fn eq(&self, other: &Self) -> bool {
		self.parts == other.parts
	}
}

impl<'a> Eq for QueryCompoundSelector<'a> {}

impl<'a> QueryCompoundSelector<'a> {
	pub fn parts(&self) -> &[QuerySelectorComponent<'a>] {
		&self.parts
	}

	/// Get the precomputed metadata for this selector.
	pub fn metadata(&self) -> QuerySelectorMetadata {
		self.metadata
	}

	/// Get pre-split segments in reverse order (rightmost first).
	/// Each segment is a simple selector with its leading combinator.
	pub fn segments(&self) -> &[SelectorSegment] {
		&self.segments
	}

	/// Get the rightmost simple selector (first segment, no combinator).
	#[inline]
	pub fn rightmost(&self) -> &[QuerySelectorComponent<'a>] {
		self.segments.first().map(|s| s.parts(&self.parts)).unwrap_or(&[])
	}

	/// Get ancestor segments (all segments except the rightmost).
	#[inline]
	pub fn ancestor_segments(&self) -> &[SelectorSegment] {
		if self.segments.len() > 1 { &self.segments[1..] } else { &[] }
	}

	/// Check if this selector is a simple type-only selector (e.g., "style-rule").
	/// Returns true only if: no combinators, and exactly one Type component.
	pub fn is_type_only(&self) -> bool {
		self.segments.len() == 1
			&& self.parts.len() == 1
			&& matches!(self.parts.first(), Some(QuerySelectorComponent::Type(_)))
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
		let mut parts = Vec::new_in(p.bump());
		let mut metadata = QuerySelectorMetadata::default();

		// Build segments incrementally (forward order, reversed at end)
		let mut segments: SmallVec<[SelectorSegment; 4]> = SmallVec::new();
		let mut segment_start = 0u16;
		let mut prev_combinator: Option<QueryCombinator> = None;

		// Track rightmost segment's type/wildcard incrementally (reset on each combinator)
		let mut current_segment_type: Option<NodeId> = None;
		let mut current_segment_has_wildcard = false;

		// Trim leading whitespace
		p.consume_trivia();

		// Parse components incrementally, building metadata and segments as we go
		while let Some(component) = Self::parse_compound_selector_part(p)? {
			// Track type/wildcard in current segment for rightmost_type_id
			match &component {
				QuerySelectorComponent::Type(t) => {
					if current_segment_type.is_none() {
						let c: Cursor = t.0.into();
						current_segment_type = NodeId::from_tag_name(p.to_source_cursor(c).source());
					}
				}
				QuerySelectorComponent::Wildcard(_) => {
					current_segment_has_wildcard = true;
				}
				QuerySelectorComponent::Combinator(c) => {
					// Emit segment that just ended
					let segment_end = parts.len() as u16;
					if segment_start < segment_end {
						let segment_type_id = if current_segment_has_wildcard { None } else { current_segment_type };
						segments.push(SelectorSegment {
							combinator: prev_combinator,
							start: segment_start,
							end: segment_end,
							type_id: segment_type_id,
						});
					}
					prev_combinator = Some(*c);
					segment_start = segment_end + 1; // Skip the combinator itself

					// Reset type/wildcard tracking for new segment
					current_segment_type = None;
					current_segment_has_wildcard = false;
				}
				_ => {}
			}

			// Build metadata incrementally (single pass)
			metadata = metadata.merge(component.self_metadata());

			parts.push(component);
		}

		// Emit final segment
		let final_end = parts.len() as u16;
		let final_type_id = if current_segment_has_wildcard { None } else { current_segment_type };
		if segment_start < final_end {
			segments.push(SelectorSegment {
				combinator: prev_combinator,
				start: segment_start,
				end: final_end,
				type_id: final_type_id,
			});
		} else if parts.is_empty() {
			// Empty selector - no segments
		} else if segments.is_empty() {
			// Single segment covering all parts (no combinators)
			segments.push(SelectorSegment { combinator: None, start: 0, end: final_end, type_id: final_type_id });
		}

		// Reverse segments to get rightmost-first order, then shift combinators
		// Forward: [A:None, B:Child, C:Desc] → Reversed: [C:Desc, B:Child, A:None]
		// After shift: [C:None, B:Desc, A:Child] (each gets combinator from next in forward order)
		segments.reverse();
		let mut shifted_combinator = None;
		for seg in &mut segments {
			std::mem::swap(&mut seg.combinator, &mut shifted_combinator);
		}

		// rightmost_type_id: type from rightmost segment, unless it has a wildcard
		metadata.rightmost_type_id = if current_segment_has_wildcard { None } else { current_segment_type };

		Ok(Self { parts, metadata, segments })
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

/// Type selector.
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryType(pub T![Ident]);

impl QueryType {
	/// Returns the NodeId for this type selector, computed lazily from the source.
	pub fn node_id(&self, source: &str) -> Option<NodeId> {
		let c: Cursor = self.0.into();
		NodeId::from_tag_name(c.str_slice(source))
	}
}

impl NodeWithMetadata<QuerySelectorMetadata> for QueryType {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		// Note: rightmost_type_id is computed during parsing, not here.
		// Setting it here would be wasteful since the parsing logic overrides it.
		QuerySelectorMetadata { structure: SelectorStructure::HasType, ..Default::default() }
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
	pub fn property_kind(&self) -> Option<PropertyKind> {
		let c: Cursor = self.attr_name.into();
		CsskitAtomSet::from_bits(c.atom_bits()).to_property_kind()
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
			Self::Not(p) => {
				// Only use not_type optimization for simple type-only selectors like :not(style-rule)
				// Selectors with combinators or additional filters require full matching
				let not_type = if p.selector.is_type_only() { p.selector.metadata().rightmost_type_id } else { None };
				QuerySelectorMetadata { not_type, ..Default::default() }
			}
			Self::NthLastChild(_) => QuerySelectorMetadata { deferred: true, ..Default::default() },
			Self::NthOfType(_) | Self::NthLastOfType(_) => {
				QuerySelectorMetadata { deferred: true, needs_type_tracking: true, ..Default::default() }
			}
			Self::PropertyType(p) => {
				QuerySelectorMetadata { property_groups: p.property_group(), ..Default::default() }
			}
			Self::Prefixed(p) => QuerySelectorMetadata {
				requirements: SelectorRequirements::Prefixed,
				vendor_filter: p.vendor_prefix(),
				..Default::default()
			},
			Self::NthChild(_) => QuerySelectorMetadata::default(),
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

impl QueryPropertyTypePseudo {
	/// Returns the PropertyGroup for this pseudo-class
	pub fn property_group(&self) -> PropertyGroup {
		let c: Cursor = self.group.into();
		let atom = CsskitAtomSet::from_bits(c.atom_bits());
		atom.to_property_group().unwrap_or(PropertyGroup::none())
	}
}

/// `:prefixed(<vendor>)` pseudo-class.
#[derive(csskit_derives::Peek, csskit_derives::Parse, csskit_derives::ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryPrefixedPseudo {
	pub colon: T![:],
	pub function: T![Function],
	pub vendor: T![Ident],
	pub close: Option<T![')']>,
}

impl QueryPrefixedPseudo {
	/// Returns the VendorPrefixes for this pseudo-class
	pub fn vendor_prefix(&self) -> VendorPrefixes {
		let c: Cursor = self.vendor.into();
		let atom = CsskitAtomSet::from_bits(c.atom_bits());
		atom.to_vendor_prefix().unwrap_or(VendorPrefixes::none())
	}
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
