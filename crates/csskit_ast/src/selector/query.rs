use super::metadata::{QuerySelectorMetadata, SelectorRequirements, SelectorStructure};
use crate::{CsskitAtomSet, diagnostics::QueryDiagnostic};
use bumpalo::collections::Vec;
use css_ast::{AttributeOperator, Nth, PropertyGroup, PropertyKind, VendorPrefixes, visit::NodeId};
use css_parse::{
	AtomSet, CompoundSelector as CompoundSelectorTrait, Cursor, CursorSink, Diagnostic, NodeMetadata, NodeWithMetadata,
	Parse, Parser, Peek, Result, SelectorComponent as SelectorComponentTrait, State, T, ToCursors, pseudo_class,
	syntax::CommaSeparated,
};
use csskit_derives::*;
use smallvec::SmallVec;

/// A pre-split segment of a compound selector.
/// Segments are stored in forward order (leftmost first).
/// Each segment has the combinator that follows it (None for the rightmost segment).
#[derive(Debug, Clone, Copy)]
pub struct SelectorSegment {
	/// Start index into the parent's parts array.
	pub start: u16,
	/// End index (exclusive) into the parent's parts array.
	pub end: u16,
	/// The combinator that follows this segment (None for the rightmost segment).
	pub combinator: Option<QueryCombinator>,
}

impl SelectorSegment {
	/// Get the simple selector parts for this segment from the parent's parts array.
	#[inline]
	pub fn parts<'p, 'b>(&self, all_parts: &'p [QuerySelectorComponent<'b>]) -> &'p [QuerySelectorComponent<'b>] {
		&all_parts[self.start as usize..self.end as usize]
	}
}

#[derive(Peek, Parse, ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QuerySelectorList<'a>(pub CommaSeparated<'a, QueryCompoundSelector<'a>>);

impl<'a> QuerySelectorList<'a> {
	pub fn selectors(&self) -> impl Iterator<Item = &QueryCompoundSelector<'a>> {
		(&self.0).into_iter().map(|(item, _comma)| item)
	}
}

#[derive(Peek, Debug, Clone)]
pub struct QueryCompoundSelector<'a> {
	/// Simple selector parts (no combinators - those are in segments).
	parts: Vec<'a, QuerySelectorComponent<'a>>,
	/// Precomputed metadata about this selector.
	metadata: QuerySelectorMetadata,
	/// Pre-split segments in forward order (leftmost first).
	/// Each segment has the combinator that follows it.
	segments: SmallVec<[SelectorSegment; 4]>,
	/// Leading combinator for :has() inner selectors (e.g., `>` in `:has(> A)`).
	/// None for normal selectors or :has() without leading combinator.
	leading_combinator: Option<QueryCombinator>,
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

	/// Get pre-split segments in forward order (leftmost first).
	pub fn segments(&self) -> &[SelectorSegment] {
		&self.segments
	}

	/// Get the rightmost simple selector (last segment).
	#[inline]
	pub fn rightmost(&self) -> &[QuerySelectorComponent<'a>] {
		self.segments.last().map(|s| s.parts(&self.parts)).unwrap_or(&[])
	}

	/// Get the leading combinator (for :has() inner selectors).
	#[inline]
	pub fn leading_combinator(&self) -> Option<QueryCombinator> {
		self.leading_combinator
	}
}

impl ToCursors for QueryCompoundSelector<'_> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		// Emit leading combinator if present
		if let Some(lc) = &self.leading_combinator {
			lc.to_cursors(s);
		}
		// Emit segments in forward order (they're already forward)
		for seg in &self.segments {
			for part in seg.parts(&self.parts) {
				part.to_cursors(s);
			}
			// Emit the combinator that follows this segment
			if let Some(comb) = &seg.combinator {
				comb.to_cursors(s);
			}
		}
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

		// Build segments in forward order. Each segment stores the combinator that follows it.
		let mut segments: SmallVec<[SelectorSegment; 4]> = SmallVec::new();
		let mut segment_start = 0u16;

		// Track rightmost segment's type for rightmost_type_id computation
		let mut rightmost_type: Option<NodeId> = None;
		let mut rightmost_has_wildcard = false;

		// Track leading combinator (for :has() inner selectors)
		let mut leading_combinator: Option<QueryCombinator> = None;

		// Trim leading whitespace
		p.consume_trivia();

		// Parse components incrementally, building metadata and segments as we go
		while let Some(component) = Self::parse_compound_selector_part(p)? {
			// Handle combinators separately - don't add to parts
			if let Some(combinator) = component.as_combinator() {
				let segment_end = parts.len() as u16;
				if segment_start < segment_end {
					// Emit the segment that just ended, with this combinator following it
					segments.push(SelectorSegment {
						start: segment_start,
						end: segment_end,
						combinator: Some(combinator),
					});
					segment_start = segment_end;
					rightmost_type = None;
					rightmost_has_wildcard = false;
				} else if segments.is_empty() && leading_combinator.is_none() {
					leading_combinator = Some(combinator);
				}
				metadata = metadata.merge(component.self_metadata());
				continue;
			}

			// Track type/wildcard for rightmost_type_id computation
			match &component {
				QuerySelectorComponent::Type(t) => {
					let c: Cursor = t.0.into();
					let node_id = NodeId::from_tag_name(p.to_source_cursor(c).source());
					if rightmost_type.is_none() {
						rightmost_type = node_id;
					}
					// Accumulate at-rule filter for all type selectors in the compound selector
					if let Some(at_rule_id) = node_id.and_then(|id| id.to_at_rule_id()) {
						metadata.at_rule_filter |= at_rule_id;
					}
				}
				QuerySelectorComponent::Wildcard(_) => {
					rightmost_has_wildcard = true;
				}
				_ => {}
			}

			// Build metadata incrementally (single pass)
			metadata = metadata.merge(component.self_metadata());
			parts.push(component);
		}

		// Emit final segment (rightmost, no following combinator)
		let final_end = parts.len() as u16;
		if segment_start < final_end {
			segments.push(SelectorSegment { start: segment_start, end: final_end, combinator: None });
		}

		// rightmost_type_id: from the final (rightmost) segment, if no wildcard
		metadata.rightmost_type_id = if rightmost_has_wildcard { None } else { rightmost_type };

		// Compute is_type_only: single type, no combinators, no other parts
		metadata.is_type_only =
			segments.len() == 1 && parts.len() == 1 && matches!(parts.first(), Some(QuerySelectorComponent::Type(_)));

		Ok(Self { parts, metadata, segments, leading_combinator })
	}
}

/// Selector components (type, wildcard, attribute, pseudo-class, combinator).
/// Note: Combinators are parsed but stored separately in segments, not in QueryCompoundSelector::parts.
#[derive(Peek, ToCursors, Debug, Clone, PartialEq, Eq)]
pub enum QuerySelectorComponent<'a> {
	Type(QueryType),
	Wildcard(QueryWildcard),
	Attribute(QueryAttribute),
	/// Combinators are used during parsing but NOT stored in parts - they go to segments.
	Combinator(QueryCombinator),
	PseudoClass(QueryPseudoClass),
	FunctionalPseudoClass(QueryFunctionalPseudoClass<'a>),
}

impl<'a> QuerySelectorComponent<'a> {
	/// Extract combinator if this is a Combinator variant.
	pub fn as_combinator(&self) -> Option<QueryCombinator> {
		match self {
			Self::Combinator(c) => Some(*c),
			_ => None,
		}
	}
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
#[derive(Peek, Parse, ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryType(pub T![Ident]);

impl QueryType {
	/// Returns the NodeId for this type selector, computed from the source.
	pub fn node_id(&self, source: &str) -> Option<NodeId> {
		let c: Cursor = self.0.into();
		NodeId::from_tag_name(c.str_slice(source))
	}
}

impl NodeWithMetadata<QuerySelectorMetadata> for QueryType {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		// Note: rightmost_type_id is computed during parsing, not here.
		// Setting it here would be wasteful since the parsing logic overrides it.
		QuerySelectorMetadata::default()
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
	}
}

/// Universal selector (`*`).
#[derive(Peek, Parse, ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryWildcard(pub T![*]);

impl NodeWithMetadata<QuerySelectorMetadata> for QueryWildcard {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		QuerySelectorMetadata::default()
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
	}
}

/// Combinator (`>`, `+`, `~`, or descendant).
#[derive(Peek, Parse, ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
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
#[derive(Peek, Parse, ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryAttribute {
	pub open: T!['['],
	pub attr_name: T![Ident],
	pub matcher: Option<QueryAttributeMatcher>,
	pub close: Option<T![']']>,
}

#[derive(Peek, Parse, ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryAttributeMatcher {
	pub operator: AttributeOperator,
	pub value: QueryAttributeValue,
}

#[derive(Peek, Parse, ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
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
		QuerySelectorMetadata {
			structure: SelectorStructure::HasAttribute,
			attribute_filter,
			self_attribute_filter: attribute_filter,
			..Default::default()
		}
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
	}
}

// Non-functional pseudo-classes (`:important`, `:custom`, etc.).
pseudo_class!(
	#[derive(ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
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
		let structure = SelectorStructure::HasPseudo;
		match self {
			Self::Important(..) => QuerySelectorMetadata {
				requirements: SelectorRequirements::Important,
				self_requirements: SelectorRequirements::Important,
				structure,
				..Default::default()
			},
			Self::Custom(..) => QuerySelectorMetadata {
				requirements: SelectorRequirements::Custom,
				self_requirements: SelectorRequirements::Custom,
				structure,
				..Default::default()
			},
			Self::Computed(..) => QuerySelectorMetadata {
				requirements: SelectorRequirements::Computed,
				self_requirements: SelectorRequirements::Computed,
				structure,
				..Default::default()
			},
			Self::Shorthand(..) => QuerySelectorMetadata {
				requirements: SelectorRequirements::Shorthand,
				self_requirements: SelectorRequirements::Shorthand,
				structure,
				..Default::default()
			},
			Self::Longhand(..) => QuerySelectorMetadata {
				requirements: SelectorRequirements::Longhand,
				self_requirements: SelectorRequirements::Longhand,
				structure,
				..Default::default()
			},
			Self::Unknown(..) => QuerySelectorMetadata {
				requirements: SelectorRequirements::Unknown,
				self_requirements: SelectorRequirements::Unknown,
				structure,
				..Default::default()
			},
			Self::Prefixed(..) => QuerySelectorMetadata {
				requirements: SelectorRequirements::Prefixed,
				self_requirements: SelectorRequirements::Prefixed,
				structure,
				..Default::default()
			},
			Self::Rule(..) => QuerySelectorMetadata {
				requirements: SelectorRequirements::Rule,
				self_requirements: SelectorRequirements::Rule,
				structure,
				..Default::default()
			},
			Self::AtRule(..) => QuerySelectorMetadata {
				requirements: SelectorRequirements::AtRule,
				self_requirements: SelectorRequirements::AtRule,
				structure,
				..Default::default()
			},
			Self::OnlyChild(..) => QuerySelectorMetadata { deferred: true, structure, ..Default::default() },
			Self::LastChild(..) => QuerySelectorMetadata { deferred: true, structure, ..Default::default() },
			Self::Empty(..) => QuerySelectorMetadata {
				requirements: SelectorRequirements::Empty,
				self_requirements: SelectorRequirements::Empty,
				structure,
				..Default::default()
			},
			Self::FirstOfType(..) => {
				QuerySelectorMetadata { deferred: true, needs_type_tracking: true, structure, ..Default::default() }
			}
			Self::LastOfType(..) => {
				QuerySelectorMetadata { deferred: true, needs_type_tracking: true, structure, ..Default::default() }
			}
			Self::OnlyOfType(..) => {
				QuerySelectorMetadata { deferred: true, needs_type_tracking: true, structure, ..Default::default() }
			}
			_ => QuerySelectorMetadata { structure, ..Default::default() },
		}
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
	}
}

/// Functional pseudo-classes (`:not()`, `:nth-child()`, etc.).
#[derive(Peek, ToCursors, Debug, Clone, PartialEq, Eq)]
pub enum QueryFunctionalPseudoClass<'a> {
	Not(QueryNotPseudo<'a>),
	Has(QueryHasPseudo<'a>),
	NthChild(QueryNthPseudo),
	NthLastChild(QueryNthPseudo),
	NthOfType(QueryNthPseudo),
	NthLastOfType(QueryNthPseudo),
	PropertyType(QueryPropertyTypePseudo),
	Prefixed(QueryPrefixedPseudo),
	Size(QuerySizePseudo),
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
			CsskitAtomSet::Has => p.parse::<QueryHasPseudo<'a>>().map(Self::Has),
			CsskitAtomSet::NthChild => p.parse::<QueryNthPseudo>().map(Self::NthChild),
			CsskitAtomSet::NthLastChild => p.parse::<QueryNthPseudo>().map(Self::NthLastChild),
			CsskitAtomSet::NthOfType => p.parse::<QueryNthPseudo>().map(Self::NthOfType),
			CsskitAtomSet::NthLastOfType => p.parse::<QueryNthPseudo>().map(Self::NthLastOfType),
			CsskitAtomSet::PropertyType => p.parse::<QueryPropertyTypePseudo>().map(Self::PropertyType),
			CsskitAtomSet::Prefixed => p.parse::<QueryPrefixedPseudo>().map(Self::Prefixed),
			CsskitAtomSet::Size => p.parse::<QuerySizePseudo>().map(Self::Size),
			_ => Err(Diagnostic::new(c, Diagnostic::unknown_functional_pseudo_class))?,
		}
	}
}

impl<'a> NodeWithMetadata<QuerySelectorMetadata> for QueryFunctionalPseudoClass<'a> {
	fn self_metadata(&self) -> QuerySelectorMetadata {
		let structure = SelectorStructure::HasFunctionalPseudo;
		match self {
			Self::Not(p) => {
				let inner_meta = p.selector.metadata();
				let not_type = if inner_meta.is_type_only { inner_meta.rightmost_type_id } else { None };
				QuerySelectorMetadata {
					not_type,
					deferred: inner_meta.deferred,
					needs_type_tracking: inner_meta.needs_type_tracking,
					structure,
					..Default::default()
				}
			}
			Self::Has(p) => {
				let inner_meta = p.selector.metadata();
				QuerySelectorMetadata {
					requirements: inner_meta.requirements,
					at_rule_filter: inner_meta.at_rule_filter,
					property_groups: inner_meta.property_groups,
					vendor_filter: inner_meta.vendor_filter,
					attribute_filter: inner_meta.attribute_filter,
					// self_* fields intentionally left as none() - descendants satisfy these
					structure,
					..Default::default()
				}
			}
			Self::NthLastChild(_) => QuerySelectorMetadata { deferred: true, structure, ..Default::default() },
			Self::NthOfType(_) => {
				QuerySelectorMetadata { deferred: true, needs_type_tracking: true, structure, ..Default::default() }
			}
			Self::NthLastOfType(_) => {
				QuerySelectorMetadata { deferred: true, needs_type_tracking: true, structure, ..Default::default() }
			}
			Self::PropertyType(p) => {
				let groups = p.property_group().unwrap_or(PropertyGroup::none());
				QuerySelectorMetadata {
					property_groups: groups,
					self_property_groups: groups,
					structure,
					..Default::default()
				}
			}
			Self::Prefixed(p) => {
				let vendor = p.vendor_prefix().unwrap_or(VendorPrefixes::none());
				QuerySelectorMetadata {
					requirements: SelectorRequirements::Prefixed,
					self_requirements: SelectorRequirements::Prefixed,
					vendor_filter: vendor,
					self_vendor_filter: vendor,
					structure,
					..Default::default()
				}
			}
			Self::Size(_) => QuerySelectorMetadata { structure, ..Default::default() },
			Self::NthChild(_) => QuerySelectorMetadata { structure, ..Default::default() },
		}
	}

	fn metadata(&self) -> QuerySelectorMetadata {
		self.self_metadata()
	}
}

/// `:not(<selector>)` pseudo-class.
#[derive(Peek, Parse, ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryNotPseudo<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub selector: QueryCompoundSelector<'a>,
	pub close: Option<T![')']>,
}

/// `:has(<relative-selector>)` pseudo-class.
#[derive(Peek, ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryHasPseudo<'a> {
	pub colon: T![:],
	pub function: T![Function],
	pub selector: QueryCompoundSelector<'a>,
	pub close: Option<T![')']>,
}

impl<'a> Parse<'a> for QueryHasPseudo<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Check for nested :has() which is disallowed by CSS spec
		if p.is(State::DisallowRelativeSelector) {
			return Err(Diagnostic::new(p.peek_n(2), Diagnostic::nested_has_not_allowed))?;
		}
		let colon = p.parse::<T![:]>()?;
		let function = p.parse::<T![Function]>()?;
		// Parse the inner selector with DisallowRelativeSelector flag set
		let old_state = p.set_state(State::DisallowRelativeSelector);
		let selector = p.parse::<QueryCompoundSelector<'a>>();
		p.set_state(old_state);
		let selector = selector?;
		let close = p.parse_if_peek::<T![')']>()?;
		Ok(Self { colon, function, selector, close })
	}
}

/// `:nth-child()`, `:nth-last-child()`, `:nth-of-type()`, `:nth-last-of-type()` pseudo-classes.
#[derive(Peek, Parse, ToCursors, Debug, Clone, PartialEq, Eq)]
pub struct QueryNthPseudo {
	pub colon: T![:],
	pub function: T![Function],
	pub value: Nth,
	pub close: Option<T![')']>,
}

/// `:property-type(<group>)` pseudo-class.
#[derive(Peek, Parse, ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryPropertyTypePseudo {
	pub colon: T![:],
	pub function: T![Function],
	pub group: T![Ident],
	pub close: Option<T![')']>,
}

impl QueryPropertyTypePseudo {
	/// Returns the PropertyGroup for this pseudo-class, or None if unknown.
	pub fn property_group(&self) -> Option<PropertyGroup> {
		let c: Cursor = self.group.into();
		let atom = CsskitAtomSet::from_bits(c.atom_bits());
		atom.to_property_group()
	}
}

/// `:prefixed(<vendor>)` pseudo-class.
#[derive(Peek, Parse, ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QueryPrefixedPseudo {
	pub colon: T![:],
	pub function: T![Function],
	pub vendor: T![Ident],
	pub close: Option<T![')']>,
}

impl QueryPrefixedPseudo {
	/// Returns the VendorPrefixes for this pseudo-class, or None if unknown.
	pub fn vendor_prefix(&self) -> Option<VendorPrefixes> {
		let c: Cursor = self.vendor.into();
		let atom = CsskitAtomSet::from_bits(c.atom_bits());
		atom.to_vendor_prefix()
	}
}

/// `:size(<comparison>)` pseudo-class.
#[derive(Peek, Parse, ToCursors, Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuerySizePseudo {
	pub colon: T![:],
	pub function: T![Function],
	pub comparison: SizeComparison,
	pub close: Option<T![')']>,
}

#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SizeComparison {
	pub operator: Option<SizeOperator>,
	pub value: T![Number],
}

#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum SizeOperator {
	/// `!=` operator (not equal)
	NotEqual(T![!=]),
	/// `>` operator (greater than)
	GreaterThan(T![>]),
	/// `<` operator (less than)
	LessThan(T![<]),
	/// `>=` operator (greater than or equal)
	GreaterThanOrEqual(T![>=]),
	/// `<=` operator (less than or equal)
	LessThanOrEqual(T![<=]),
}

impl QuerySizePseudo {
	/// Returns true if the given size matches the comparison.
	pub fn matches(&self, size: u16) -> bool {
		let target = {
			let c: Cursor = self.comparison.value.into();
			c.token().value() as u16
		};
		match &self.comparison.operator {
			None => size == target,
			Some(SizeOperator::NotEqual(_)) => size != target,
			Some(SizeOperator::GreaterThan(_)) => size > target,
			Some(SizeOperator::LessThan(_)) => size < target,
			Some(SizeOperator::GreaterThanOrEqual(_)) => size >= target,
			Some(SizeOperator::LessThanOrEqual(_)) => size <= target,
		}
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
