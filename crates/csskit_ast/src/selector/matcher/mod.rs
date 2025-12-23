mod context;

use crate::{
	CsskitAtomSet, MatchOutput, QueryAttribute, QueryCombinator, QueryCompoundSelector, QueryFunctionalPseudoClass,
	QueryPseudoClass, QuerySelectorComponent, QuerySelectorList, SelectorRequirements, SelectorSegment,
	SelectorStructure,
};
use context::{MatchContext, ParentEntry, PropertyValues, SiblingInfo};
use css_ast::{
	PropertyGroup, VendorPrefixes,
	visit::{NodeId, QueryableNode, Visit, Visitable},
	*,
};
use css_lexer::Span;
use css_parse::{Declaration, DeclarationValue, NodeWithMetadata, ToSpan};
use smallvec::SmallVec;

/// Inline type counter for small numbers of unique types (common case).
/// Uses linear scan which is faster than HashMap for <~12 entries due to cache efficiency.
struct TypeCounts(SmallVec<[(NodeId, usize); 8]>);

impl TypeCounts {
	fn new() -> Self {
		Self(SmallVec::new())
	}

	/// Increment count for node_id and return the new count.
	#[inline]
	fn increment(&mut self, node_id: NodeId) -> usize {
		for (id, count) in &mut self.0 {
			if *id == node_id {
				*count += 1;
				return *count;
			}
		}
		self.0.push((node_id, 1));
		1
	}

	/// Get the count for node_id (returns 0 if not found).
	#[inline]
	fn get(&self, node_id: NodeId) -> usize {
		for (id, count) in &self.0 {
			if *id == node_id {
				return *count;
			}
		}
		0
	}
}

pub struct SelectorMatcher<'a, 'b> {
	selector_source: &'b str,
	/// Cached references to selectors that passed metadata filtering (O(1) access)
	active_selectors: Vec<&'a QueryCompoundSelector<'b>>,
	source: &'a str,
	matches: Vec<MatchOutput>,
	parent_stack: Vec<ParentEntry<'a>>,
}

impl<'a, 'b> SelectorMatcher<'a, 'b> {
	pub fn new(selectors: &'a QuerySelectorList<'b>, selector_source: &'b str, source: &'a str) -> Self {
		let active_selectors: Vec<_> = selectors.selectors().collect();
		Self { selector_source, active_selectors, source, matches: Vec::new(), parent_stack: Vec::new() }
	}

	/// Run matching with metadata-based early filtering.
	/// Selectors that require features not present in the metadata are skipped.
	pub fn run<T: Visitable + NodeWithMetadata<CssMetadata>>(mut self, root: &T) -> Vec<MatchOutput> {
		let css_meta = root.metadata();

		self.active_selectors.retain(|selector| {
			let selector_meta = selector.metadata();
			// TODO: Remove Prefixed bypass once CssMetadata properly detects unknown vendor-prefixed properties
			selector_meta.requirements.is_none()
				|| selector_meta.can_match(&css_meta)
				|| selector_meta.requirements.contains(SelectorRequirements::Prefixed)
		});

		if self.active_selectors.is_empty() {
			return Vec::new();
		}

		root.accept(&mut self);
		self.matches
	}

	fn check_match<T: QueryableNode>(&mut self, node: &T) {
		let node_id = T::NODE_ID;
		let span = node.to_span();
		let sibling_index = self.parent_stack.last().map(|p| p.visited_children.len() as i32 + 1).unwrap_or(1);
		let context = MatchContext {
			metadata: Some(node.self_metadata()),
			properties: PropertyValues::from_node(node),
			source: self.source,
			sibling_index,
			..Default::default()
		};
		self.check_match_with_context(node_id, span, &context);
		if let Some(parent) = self.parent_stack.last_mut() {
			parent.visited_children.push(SiblingInfo { node_id: Some(node_id), span, context: context.clone() });
		}
		self.parent_stack.push(ParentEntry { node_id: Some(node_id), span, context, visited_children: Vec::new() });
	}

	fn exit_node<T: QueryableNode>(&mut self, _node: &T) {
		if let Some(exiting) = self.parent_stack.pop() {
			self.check_deferred_matches(&exiting.visited_children);
		}
	}

	fn check_deferred_matches(&mut self, children: &[SiblingInfo]) {
		let total = children.len();

		// Compute type indices once if any active selector needs type tracking
		let type_info = if self.active_selectors.iter().any(|s| {
			let meta = s.metadata();
			meta.deferred && meta.needs_type_tracking
		}) {
			self.compute_type_indices(children)
		} else {
			Vec::new()
		};

		for selector in &self.active_selectors {
			let meta = selector.metadata();
			if !meta.deferred {
				continue;
			}
			// :empty is handled separately when total == 0
			if total == 0 && meta.has_empty {
				continue;
			}

			for (index, child) in children.iter().enumerate() {
				let Some(node_id) = child.node_id else {
					continue;
				};

				let index_from_end = (total - index) as i32;
				let (type_index, type_index_from_end, type_count) =
					if meta.needs_type_tracking && !type_info.is_empty() { type_info[index] } else { (1, 1, 1) };
				let deferred_match = self.child_matches_deferred(
					selector,
					index_from_end,
					total,
					type_index,
					type_index_from_end,
					type_count,
				);
				if deferred_match && self.matches_deferred_selector(selector, node_id, &child.context) {
					self.matches.push(MatchOutput { node_id, span: child.span });
				}
			}
		}

		if total == 0 {
			self.check_empty_match();
		}
	}

	fn compute_type_indices(&self, children: &[SiblingInfo]) -> Vec<(i32, i32, usize)> {
		let mut type_counts = TypeCounts::new();
		let mut type_indices: Vec<i32> = Vec::with_capacity(children.len());

		for child in children {
			if let Some(node_id) = child.node_id {
				let count = type_counts.increment(node_id);
				type_indices.push(count as i32);
			} else {
				type_indices.push(0);
			}
		}

		let mut type_counts_reverse = TypeCounts::new();
		let mut result: Vec<(i32, i32, usize)> = vec![(0, 0, 0); children.len()];

		for (i, child) in children.iter().enumerate().rev() {
			if let Some(node_id) = child.node_id {
				let count = type_counts_reverse.increment(node_id);
				let total = type_counts.get(node_id);
				result[i] = (type_indices[i], count as i32, total);
			}
		}

		result
	}

	fn check_empty_match(&mut self) {
		let Some(exiting) = self.parent_stack.last() else {
			return;
		};
		let Some(node_id) = exiting.node_id else {
			return;
		};
		let span = exiting.span;
		let context = &exiting.context;

		for selector in &self.active_selectors {
			if !selector.metadata().has_empty {
				continue;
			}
			if self.matches_deferred_selector(selector, node_id, context) {
				self.matches.push(MatchOutput { node_id, span });
			}
		}
	}

	fn child_matches_deferred(
		&self,
		selector: &QueryCompoundSelector,
		index_from_end: i32,
		total: usize,
		type_index: i32,
		type_index_from_end: i32,
		type_count: usize,
	) -> bool {
		// Check all deferred conditions by iterating through selector parts
		for part in selector.parts() {
			match part {
				QuerySelectorComponent::PseudoClass(pseudo) => match pseudo {
					QueryPseudoClass::OnlyChild(..) if total != 1 => return false,
					QueryPseudoClass::LastChild(..) if index_from_end != 1 => return false,
					QueryPseudoClass::FirstOfType(..) if type_index != 1 => return false,
					QueryPseudoClass::LastOfType(..) if type_index_from_end != 1 => return false,
					QueryPseudoClass::OnlyOfType(..) if type_count != 1 => return false,
					_ => {}
				},
				QuerySelectorComponent::FunctionalPseudoClass(pseudo) => match pseudo {
					QueryFunctionalPseudoClass::NthLastChild(p) if !p.value.matches(index_from_end) => return false,
					QueryFunctionalPseudoClass::NthOfType(p) if !p.value.matches(type_index) => return false,
					QueryFunctionalPseudoClass::NthLastOfType(p) if !p.value.matches(type_index_from_end) => {
						return false;
					}
					_ => {}
				},
				_ => {}
			}
		}

		true
	}

	fn matches_deferred_selector(
		&self,
		selector: &QueryCompoundSelector,
		node_id: NodeId,
		context: &MatchContext,
	) -> bool {
		let meta = selector.metadata();
		// Fast type checks from metadata
		if meta.rightmost_type_id.is_some_and(|expected| expected != node_id) {
			return false;
		}
		if meta.not_type.is_some_and(|excluded| excluded == node_id) {
			return false;
		}
		// Only match if this is a simple selector (no combinators leading to ancestors)
		if meta.structure.contains(SelectorStructure::HasCombinator) {
			return false;
		}

		// Check non-deferred parts (deferred ones already checked by child_matches_deferred)
		for part in selector.parts() {
			match part {
				QuerySelectorComponent::Attribute(attr) => {
					if !self.matches_attribute(attr, context) {
						return false;
					}
				}
				QuerySelectorComponent::PseudoClass(p) => {
					// Skip deferred pseudos - they're already checked
					if meta.deferred
						&& matches!(
							p,
							QueryPseudoClass::OnlyChild(..)
								| QueryPseudoClass::LastChild(..)
								| QueryPseudoClass::FirstOfType(..)
								| QueryPseudoClass::LastOfType(..)
								| QueryPseudoClass::OnlyOfType(..)
								| QueryPseudoClass::Empty(..)
						) {
						continue;
					}
					if !self.matches_pseudo(p, Some(node_id), context) {
						return false;
					}
				}
				QuerySelectorComponent::FunctionalPseudoClass(p) => {
					// Skip deferred functional pseudos - they're already checked
					if meta.deferred
						&& matches!(
							p,
							QueryFunctionalPseudoClass::NthLastChild(_)
								| QueryFunctionalPseudoClass::NthOfType(_)
								| QueryFunctionalPseudoClass::NthLastOfType(_)
						) {
						continue;
					}
					if !self.matches_functional_pseudo(p, Some(node_id), context) {
						return false;
					}
				}
				_ => {}
			}
		}
		true
	}

	fn check_match_with_context(&mut self, node_id: NodeId, span: Span, context: &MatchContext) {
		for selector in &self.active_selectors {
			if self.matches_selector_with_context(selector, node_id, context) {
				self.matches.push(MatchOutput { node_id, span });
			}
		}
	}

	fn check_declaration_match(&mut self, span: Span, context: &MatchContext) {
		for selector in &self.active_selectors {
			let meta = selector.metadata();
			// Declaration selectors should not have combinators or type selectors
			if meta.structure.contains(SelectorStructure::HasCombinator)
				|| meta.structure.contains(SelectorStructure::HasType)
			{
				continue;
			}
			if self.matches_declaration_parts(selector.parts(), context) {
				self.matches.push(MatchOutput { node_id: NodeId::StyleRule, span });
			}
		}
	}

	fn matches_attribute(&self, attr: &QueryAttribute, context: &MatchContext) -> bool {
		let Some(property_kind) = attr.property_kind() else {
			return false;
		};
		let Some(cursor) = context.properties.get(property_kind) else {
			return false;
		};
		// Presence-only selector [name] - just check if property exists
		let Some(expected_value) = attr.attr_value(self.selector_source) else {
			return true;
		};
		let actual_value = cursor.str_slice(context.source);
		let Some(operator) = attr.operator() else {
			return true;
		};
		let actual = actual_value.as_bytes();
		let expected = expected_value.as_bytes();
		match operator {
			AttributeOperator::Exact(_) => {
				let expected_atom = CssAtomSet::from_str(expected_value);
				if expected_atom != CssAtomSet::_None {
					return CssAtomSet::from_bits(cursor.atom_bits()) == expected_atom;
				}
				actual.eq_ignore_ascii_case(expected)
			}
			AttributeOperator::SpaceList(_) => {
				!expected.is_empty()
					&& actual_value.split_ascii_whitespace().any(|word| word.as_bytes().eq_ignore_ascii_case(expected))
			}
			AttributeOperator::LangPrefix(_) => {
				expected.is_empty()
					|| actual.eq_ignore_ascii_case(expected)
					|| (actual.len() > expected.len()
						&& actual.get(expected.len()) == Some(&b'-')
						&& actual.get(..expected.len()).is_some_and(|prefix| prefix.eq_ignore_ascii_case(expected)))
			}
			AttributeOperator::Prefix(_) => {
				expected.is_empty()
					|| actual.get(..expected.len()).is_some_and(|prefix| prefix.eq_ignore_ascii_case(expected))
			}
			AttributeOperator::Suffix(_) => {
				expected.is_empty()
					|| actual
						.len()
						.checked_sub(expected.len())
						.and_then(|start| actual.get(start..))
						.is_some_and(|suffix| suffix.eq_ignore_ascii_case(expected))
			}
			AttributeOperator::Contains(_) => {
				expected.is_empty()
					|| actual.windows(expected.len()).any(|window| window.eq_ignore_ascii_case(expected))
			}
		}
	}

	fn matches_selector_with_context(
		&self,
		selector: &QueryCompoundSelector<'b>,
		node_id: NodeId,
		context: &MatchContext,
	) -> bool {
		let parts = selector.parts();
		if parts.is_empty() {
			return false;
		}

		let meta = selector.metadata();

		// Type check using pre-computed rightmost type
		if meta.rightmost_type_id.is_some_and(|expected| expected != node_id) {
			return false;
		}

		// :not(type) check using pre-computed excluded type
		if meta.not_type.is_some_and(|excluded| excluded == node_id) {
			return false;
		}

		// Simple type-only selectors (e.g., "style-rule")
		if parts.len() == 1 && meta.rightmost_type_id.is_some() {
			return true;
		}

		// Check property_groups containment
		if !meta.property_groups.is_none() {
			let node_groups = context.metadata.map(|m| m.property_groups).unwrap_or(PropertyGroup::none());
			if !node_groups.contains(meta.property_groups) {
				return false;
			}
		}

		// Check vendor_filter containment
		if !meta.vendor_filter.is_none() {
			let node_vendors = context.metadata.map(|m| m.vendor_prefixes).unwrap_or(VendorPrefixes::none());
			if !node_vendors.contains(meta.vendor_filter) {
				return false;
			}
		}

		// Check rightmost simple selector against current node
		if !self.matches_simple_parts(selector.rightmost(), node_id, context, meta.rightmost_type_id) {
			return false;
		}

		// Check ancestor segments
		self.matches_ancestor_segments(selector.ancestor_segments(), parts)
	}

	/// Match a selector's parts against a node or declaration.
	/// Used for inner selectors in :not() - no fast-path optimizations.
	fn matches_inner_selector(
		&self,
		selector: &QueryCompoundSelector<'b>,
		node_id: Option<NodeId>,
		context: &MatchContext,
	) -> bool {
		match node_id {
			Some(id) => {
				let meta = selector.metadata();
				self.matches_simple_parts(selector.rightmost(), id, context, meta.rightmost_type_id)
					&& self.matches_ancestor_segments(selector.ancestor_segments(), selector.parts())
			}
			None => {
				selector.ancestor_segments().is_empty() && self.matches_declaration_parts(selector.rightmost(), context)
			}
		}
	}

	/// Match ancestor segments against parent stack. Segments are in reverse order (rightmost first).
	fn matches_ancestor_segments(&self, segments: &[SelectorSegment], parts: &[QuerySelectorComponent<'b>]) -> bool {
		if segments.is_empty() {
			return true;
		}

		let mut parent_idx = self.parent_stack.len();

		for segment in segments {
			let simple_parts = segment.parts(parts);
			let combinator = segment.combinator.as_ref();
			let type_id = segment.type_id;

			match combinator {
				Some(QueryCombinator::Child(_)) => {
					// Must match direct parent
					if parent_idx == 0 {
						return false;
					}
					parent_idx -= 1;
					let p = &self.parent_stack[parent_idx];
					if !self.matches_entry_parts(simple_parts, p.node_id, &p.context, type_id) {
						return false;
					}
				}
				Some(QueryCombinator::NextSibling(_)) => {
					// Must match immediately preceding sibling
					if parent_idx == 0 {
						return false;
					}
					let siblings = &self.parent_stack[parent_idx - 1].visited_children;
					if siblings
						.last()
						.is_none_or(|s| !self.matches_entry_parts(simple_parts, s.node_id, &s.context, type_id))
					{
						return false;
					}
				}
				Some(QueryCombinator::SubsequentSibling(_)) => {
					// Must match any preceding sibling
					if parent_idx == 0 {
						return false;
					}
					let siblings = &self.parent_stack[parent_idx - 1].visited_children;
					if !siblings.iter().any(|s| self.matches_entry_parts(simple_parts, s.node_id, &s.context, type_id))
					{
						return false;
					}
				}
				Some(QueryCombinator::Descendant(_)) | None => {
					// Find any matching ancestor
					let mut found = false;
					while parent_idx > 0 {
						parent_idx -= 1;
						let p = &self.parent_stack[parent_idx];
						if self.matches_entry_parts(simple_parts, p.node_id, &p.context, type_id) {
							found = true;
							break;
						}
					}
					if !found {
						return false;
					}
				}
			}
		}

		true
	}

	fn matches_entry_parts(
		&self,
		parts: &[QuerySelectorComponent<'b>],
		node_id: Option<NodeId>,
		context: &MatchContext,
		type_id: Option<NodeId>,
	) -> bool {
		match node_id {
			Some(id) => self.matches_simple_parts(parts, id, context, type_id),
			None => self.matches_declaration_parts(parts, context),
		}
	}

	fn matches_declaration_parts(&self, parts: &[QuerySelectorComponent<'b>], context: &MatchContext) -> bool {
		let mut has_meaningful = false;
		for part in parts {
			match part {
				QuerySelectorComponent::Type(_) => return false,
				QuerySelectorComponent::Wildcard(_) | QuerySelectorComponent::Combinator(_) => {}
				QuerySelectorComponent::Attribute(attr) => {
					has_meaningful = true;
					if !self.matches_attribute(attr, context) {
						return false;
					}
				}
				QuerySelectorComponent::PseudoClass(p) => {
					// matches_pseudo handles declaration-applicable pseudos
					if !self.matches_pseudo(p, None, context) {
						return false;
					}
					has_meaningful = true;
				}
				QuerySelectorComponent::FunctionalPseudoClass(p) => {
					if !self.matches_functional_pseudo(p, None, context) {
						return false;
					}
					has_meaningful = true;
				}
			}
		}
		has_meaningful
	}

	fn matches_simple_parts(
		&self,
		parts: &[QuerySelectorComponent<'b>],
		node_id: NodeId,
		context: &MatchContext,
		segment_type_id: Option<NodeId>,
	) -> bool {
		for part in parts {
			match part {
				QuerySelectorComponent::Type(_) => {
					if segment_type_id.is_some_and(|expected| expected != node_id) {
						return false;
					}
				}
				QuerySelectorComponent::Wildcard(_) => {}
				QuerySelectorComponent::Attribute(attr) => {
					if !self.matches_attribute(attr, context) {
						return false;
					}
				}
				QuerySelectorComponent::PseudoClass(p) => {
					if !self.matches_pseudo(p, Some(node_id), context) {
						return false;
					}
				}
				QuerySelectorComponent::FunctionalPseudoClass(p) => {
					if !self.matches_functional_pseudo(p, Some(node_id), context) {
						return false;
					}
				}
				QuerySelectorComponent::Combinator(_) => {
					// Combinators shouldn't appear in simple selector parts
					return false;
				}
			}
		}
		true
	}

	fn matches_pseudo(&self, pseudo: &QueryPseudoClass, node_id: Option<NodeId>, context: &MatchContext) -> bool {
		let meta = context.metadata.as_ref();
		match pseudo {
			QueryPseudoClass::Important(_, _) => context.is_important,
			QueryPseudoClass::Custom(_, _) => context.is_custom_property,
			QueryPseudoClass::Computed(_, _) => meta.is_some_and(|m| m.has_computed()),
			QueryPseudoClass::Shorthand(_, _) => meta.is_some_and(|m| m.has_shorthands()),
			QueryPseudoClass::Longhand(_, _) => meta.is_some_and(|m| m.has_longhands()),
			QueryPseudoClass::Unknown(_, _) => {
				meta.is_some_and(|m| m.has_unknown()) || node_id.is_some_and(|id| id.tag_name().contains("unknown"))
			}
			QueryPseudoClass::Prefixed(_, _) => self.is_prefixed_ctx(meta, context, None),
			QueryPseudoClass::AtRule(_, _) => meta.is_some_and(|m| m.node_kinds.contains(NodeKinds::AtRule)),
			QueryPseudoClass::Rule(_, _) => {
				meta.is_some_and(|m| m.node_kinds.intersects(NodeKinds::StyleRule | NodeKinds::AtRule))
			}
			QueryPseudoClass::Function(_, _) => {
				meta.is_some_and(|m| m.has_functions())
					|| node_id.is_some_and(|id| id.tag_name().ends_with("-function"))
			}
			QueryPseudoClass::FirstChild(_, _) => context.sibling_index == 1,
			QueryPseudoClass::Nested(_, _) => self.parent_stack.iter().any(|p| p.node_id == Some(NodeId::StyleRule)),
			QueryPseudoClass::Root(_, _) => self.parent_stack.is_empty(),
			// Deferred pseudos - return false during normal matching, checked in child_matches_deferred
			QueryPseudoClass::OnlyChild(_, _)
			| QueryPseudoClass::LastChild(_, _)
			| QueryPseudoClass::FirstOfType(_, _)
			| QueryPseudoClass::LastOfType(_, _)
			| QueryPseudoClass::OnlyOfType(_, _)
			| QueryPseudoClass::Empty(_, _) => false,
		}
	}

	fn matches_functional_pseudo(
		&self,
		pseudo: &QueryFunctionalPseudoClass,
		node_id: Option<NodeId>,
		context: &MatchContext,
	) -> bool {
		let meta = context.metadata.as_ref();
		match pseudo {
			QueryFunctionalPseudoClass::Not(p) => !self.matches_inner_selector(&p.selector, node_id, context),
			QueryFunctionalPseudoClass::NthChild(p) => p.value.matches(context.sibling_index),
			// Deferred pseudos - return false during normal matching, checked in child_matches_deferred
			QueryFunctionalPseudoClass::NthLastChild(_)
			| QueryFunctionalPseudoClass::NthOfType(_)
			| QueryFunctionalPseudoClass::NthLastOfType(_) => false,
			QueryFunctionalPseudoClass::PropertyType(p) => {
				meta.is_some_and(|m| m.property_groups.contains(p.property_group()))
			}
			QueryFunctionalPseudoClass::Prefixed(p) => {
				// Check metadata first, then fallback to property name string check
				if meta.is_some_and(|m| m.vendor_prefixes.contains(p.vendor_prefix())) {
					return true;
				}
				// Fallback for unknown properties: check property name string
				let Some(cursor) = context.properties.get(PropertyKind::Name) else { return false };
				let name: &str = cursor.str_slice(context.source);
				if !name.starts_with('-') {
					return false;
				}
				let Some(end) = name[1..].find('-') else { return false };
				if end == 0 {
					return false; // Excludes custom properties (--foo)
				}
				let prefix_str = &name[1..1 + end];
				CsskitAtomSet::from_str(prefix_str).to_vendor_prefix() == Some(p.vendor_prefix())
			}
		}
	}

	/// Check if node/declaration is vendor-prefixed, with optional filter.
	fn is_prefixed_ctx(&self, meta: Option<&CssMetadata>, context: &MatchContext, filter: Option<&str>) -> bool {
		// First check metadata
		if let Some(prefix) = meta.and_then(|m| m.single_vendor_prefix()).and_then(CsskitAtomSet::from_vendor_prefix) {
			return filter.is_none_or(|f| prefix == CsskitAtomSet::from_str(f));
		}
		// Fallback: check property name string for unknown properties
		let Some(cursor) = context.properties.get(PropertyKind::Name) else { return false };
		let name: &str = cursor.str_slice(context.source);
		if !name.starts_with('-') {
			return false;
		}
		let Some(end) = name[1..].find('-') else { return false };
		if end == 0 {
			return false; // Excludes custom properties (--foo)
		}
		filter.is_none_or(|f| name[1..1 + end].eq_ignore_ascii_case(f))
	}
}

impl Visit for SelectorMatcher<'_, '_> {
	fn visit_queryable_node<T: QueryableNode>(&mut self, node: &T) {
		self.check_match(node);
	}

	fn exit_queryable_node<T: QueryableNode>(&mut self, node: &T) {
		self.exit_node(node);
	}

	// Special handling for Declaration to support :important, :custom, and [name=value]
	fn visit_declaration<'c, T: DeclarationValue<'c, CssMetadata>>(&mut self, node: &Declaration<'c, T, CssMetadata>) {
		let span = node.to_span();

		// Calculate sibling index for declarations
		let sibling_index = self.parent_stack.last().map(|p| p.visited_children.len() as i32 + 1).unwrap_or(1);

		// Build context - metadata already contains computed/shorthand/longhand/unknown/vendor info
		let context = MatchContext {
			metadata: Some(node.metadata()),
			is_important: node.important.is_some(),
			is_custom_property: node.name.is_dashed_ident(),
			properties: PropertyValues::from_declaration_name(node.name.into()),
			source: self.source,
			sibling_index,
		};

		// Check if any selector targets "declaration" type with context-dependent pseudo-classes
		self.check_declaration_match(span, &context);

		// Record this declaration as a visited child in the parent's entry (for sibling combinators)
		if let Some(parent) = self.parent_stack.last_mut() {
			parent.visited_children.push(SiblingInfo { node_id: None, span, context: context.clone() });
		}

		// Push declaration onto parent stack so child nodes can see it as ancestor
		self.parent_stack.push(ParentEntry {
			node_id: None, // Declaration has no NodeId
			span,
			context,
			visited_children: Vec::new(),
		});
	}

	// Pop declaration from parent stack
	fn exit_declaration<'c, T: DeclarationValue<'c, CssMetadata>>(&mut self, _node: &Declaration<'c, T, CssMetadata>) {
		self.parent_stack.pop();
	}
}

#[cfg(test)]
mod tests;
