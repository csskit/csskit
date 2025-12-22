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
use std::collections::HashMap;

pub struct SelectorMatcher<'a, 'b> {
	selectors: &'a QuerySelectorList<'b>,
	selector_source: &'b str,
	/// Indices of selectors that passed metadata filtering (empty = no filtering applied)
	active_selector_indices: Vec<usize>,
	source: &'a str,
	matches: Vec<MatchOutput>,
	parent_stack: Vec<ParentEntry<'a>>,
}

impl<'a, 'b> SelectorMatcher<'a, 'b> {
	pub fn new(selectors: &'a QuerySelectorList<'b>, selector_source: &'b str, source: &'a str) -> Self {
		Self {
			selectors,
			selector_source,
			active_selector_indices: Vec::new(),
			source,
			matches: Vec::new(),
			parent_stack: Vec::new(),
		}
	}

	/// Run matching with metadata-based early filtering.
	/// Selectors that require features not present in the metadata are skipped.
	pub fn run<T: Visitable + NodeWithMetadata<CssMetadata>>(mut self, root: &T) -> Vec<MatchOutput> {
		let css_meta = root.metadata();

		for (i, selector) in self.selectors.selectors().enumerate() {
			let selector_meta = selector.metadata();
			// TODO: Remove Prefixed bypass once CssMetadata properly detects unknown vendor-prefixed properties
			if selector_meta.requirements.is_none()
				|| selector_meta.can_match(&css_meta)
				|| selector_meta.requirements.contains(SelectorRequirements::Prefixed)
			{
				self.active_selector_indices.push(i);
			}
		}

		if self.active_selector_indices.is_empty() {
			return Vec::new();
		}

		root.accept(&mut self);
		self.matches
	}

	fn check_match<T: QueryableNode + ToSpan + NodeWithMetadata<CssMetadata>>(&mut self, node: &T) {
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
			parent.visited_children.push(SiblingInfo { node_id: Some(node_id), span });
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
		let type_info = if self.active_selector_indices.iter().any(|&idx| {
			self.selectors.selectors().nth(idx).is_some_and(|s| {
				let meta = s.metadata();
				meta.deferred && meta.needs_type_tracking
			})
		}) {
			self.compute_type_indices(children)
		} else {
			Vec::new()
		};

		for &selector_idx in &self.active_selector_indices {
			let Some(selector) = self.selectors.selectors().nth(selector_idx) else { continue };
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
				if deferred_match && self.matches_deferred_selector(selector, node_id) {
					self.matches.push(MatchOutput { node_id, span: child.span });
				}
			}
		}

		if total == 0 {
			self.check_empty_match();
		}
	}

	fn compute_type_indices(&self, children: &[SiblingInfo]) -> Vec<(i32, i32, usize)> {
		let mut type_counts: HashMap<NodeId, usize> = HashMap::new();
		let mut type_indices: Vec<i32> = Vec::with_capacity(children.len());

		for child in children {
			if let Some(node_id) = child.node_id {
				let count = type_counts.entry(node_id).or_insert(0);
				*count += 1;
				type_indices.push(*count as i32);
			} else {
				type_indices.push(0);
			}
		}

		let mut type_counts_reverse: HashMap<NodeId, usize> = HashMap::new();
		let mut result: Vec<(i32, i32, usize)> = vec![(0, 0, 0); children.len()];

		for (i, child) in children.iter().enumerate().rev() {
			if let Some(node_id) = child.node_id {
				let count = type_counts_reverse.entry(node_id).or_insert(0);
				*count += 1;
				let total = type_counts.get(&node_id).copied().unwrap_or(1);
				result[i] = (type_indices[i], *count as i32, total);
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

		for &selector_idx in &self.active_selector_indices {
			let Some(selector) = self.selectors.selectors().nth(selector_idx) else { continue };
			if !selector.metadata().has_empty {
				continue;
			}
			if self.matches_deferred_selector(selector, node_id) {
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

	fn matches_deferred_selector(&self, selector: &QueryCompoundSelector, node_id: NodeId) -> bool {
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
		let context = MatchContext { source: self.source, sibling_index: 1, ..Default::default() };
		for part in selector.parts() {
			match part {
				QuerySelectorComponent::Type(t) if meta.rightmost_type_id.is_none() => {
					if t.node_id(self.selector_source) != Some(node_id) {
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
					if !self.matches_pseudo(p, Some(node_id), &context) {
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
					if !self.matches_functional_pseudo(p, Some(node_id), &context) {
						return false;
					}
				}
				_ => {}
			}
		}
		true
	}

	fn check_match_with_context(&mut self, node_id: NodeId, span: Span, context: &MatchContext) {
		for &selector_idx in &self.active_selector_indices {
			let Some(selector) = self.selectors.selectors().nth(selector_idx) else { continue };
			if self.matches_selector_with_context(selector, node_id, context) {
				self.matches.push(MatchOutput { node_id, span });
			}
		}
	}

	fn check_declaration_match(&mut self, span: Span, context: &MatchContext) {
		for &selector_idx in &self.active_selector_indices {
			let Some(selector) = self.selectors.selectors().nth(selector_idx) else { continue };
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
		let Some(property_kind) = attr.attr_name_atom().to_property_kind() else {
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
		match operator {
			AttributeOperator::Exact(_) => {
				let expected_atom = CssAtomSet::from_str(expected_value);
				if expected_atom != CssAtomSet::_None {
					return CssAtomSet::from_bits(cursor.atom_bits()) == expected_atom;
				}
				actual_value.eq_ignore_ascii_case(expected_value)
			}
			AttributeOperator::SpaceList(_) => {
				actual_value.split_ascii_whitespace().any(|word| word.eq_ignore_ascii_case(expected_value))
			}
			AttributeOperator::LangPrefix(_) => {
				actual_value.eq_ignore_ascii_case(expected_value)
					|| (actual_value.len() > expected_value.len()
						&& actual_value[expected_value.len()..].starts_with('-')
						&& actual_value[..expected_value.len()].eq_ignore_ascii_case(expected_value))
			}
			AttributeOperator::Prefix(_) => {
				actual_value.len() >= expected_value.len()
					&& actual_value[..expected_value.len()].eq_ignore_ascii_case(expected_value)
			}
			AttributeOperator::Suffix(_) => {
				actual_value.len() >= expected_value.len()
					&& actual_value[actual_value.len() - expected_value.len()..].eq_ignore_ascii_case(expected_value)
			}
			AttributeOperator::Contains(_) => {
				actual_value.to_ascii_lowercase().contains(&expected_value.to_ascii_lowercase())
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
		if !self.matches_simple_parts(selector.rightmost(), node_id, context, meta.rightmost_type_id.is_some()) {
			return false;
		}

		// Check ancestor segments
		self.matches_ancestor_segments(selector.ancestor_segments(), parts)
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

			match combinator {
				Some(QueryCombinator::Child(_)) => {
					// Must match direct parent
					if parent_idx == 0 {
						return false;
					}
					parent_idx -= 1;
					if !self.matches_parent_entry_parts(simple_parts, &self.parent_stack[parent_idx]) {
						return false;
					}
				}
				Some(QueryCombinator::NextSibling(_)) => {
					// Must match immediately preceding sibling
					if parent_idx == 0 {
						return false;
					}
					let siblings = &self.parent_stack[parent_idx - 1].visited_children;
					if siblings.last().is_none_or(|s| !self.matches_sibling_info_parts(simple_parts, s)) {
						return false;
					}
				}
				Some(QueryCombinator::SubsequentSibling(_)) => {
					// Must match any preceding sibling
					if parent_idx == 0 {
						return false;
					}
					let siblings = &self.parent_stack[parent_idx - 1].visited_children;
					if !siblings.iter().any(|s| self.matches_sibling_info_parts(simple_parts, s)) {
						return false;
					}
				}
				Some(QueryCombinator::Descendant(_)) | None => {
					// Find any matching ancestor
					let mut found = false;
					while parent_idx > 0 {
						parent_idx -= 1;
						if self.matches_parent_entry_parts(simple_parts, &self.parent_stack[parent_idx]) {
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

	fn matches_sibling_info_parts(&self, parts: &[QuerySelectorComponent<'b>], sibling: &SiblingInfo) -> bool {
		match sibling.node_id {
			Some(node_id) => {
				let expected_type = self.get_type_from_parts(parts);
				expected_type.is_none_or(|expected| expected == node_id)
			}
			None => self.get_type_from_parts(parts).is_none(),
		}
	}

	fn get_type_from_parts(&self, parts: &[QuerySelectorComponent<'b>]) -> Option<NodeId> {
		for part in parts {
			if let QuerySelectorComponent::Type(t) = part {
				return t.node_id(self.selector_source);
			}
		}
		None
	}

	fn matches_parent_entry_parts(&self, parts: &[QuerySelectorComponent<'b>], parent: &ParentEntry) -> bool {
		match parent.node_id {
			// Ancestor types are not pre-verified, must check during iteration
			Some(node_id) => self.matches_simple_parts(parts, node_id, &parent.context, false),
			None => self.matches_declaration_parts(parts, &parent.context),
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
		type_pre_verified: bool,
	) -> bool {
		for part in parts {
			match part {
				QuerySelectorComponent::Type(t) => {
					// Skip type check if already verified via rightmost_type_id
					if !type_pre_verified && t.node_id(self.selector_source) != Some(node_id) {
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
			QueryFunctionalPseudoClass::Not(p) => {
				// Use pre-computed rightmost_type_id from the inner selector's metadata
				let inner_type = p.selector.metadata().rightmost_type_id;
				inner_type.is_none_or(|expected| node_id.is_none_or(|id| expected != id))
			}
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

// Generate visit methods
macro_rules! gen_visit_methods {
	( $(
		$name:ident$(<$($gen:tt),+>)?($obj:ty),
	)+ ) => {
		$(
			fn $name$(<$($gen),+>)?(&mut self, node: &$obj) {
				self.check_match(node);
			}
		)+
	}
}

// Generate exit methods
macro_rules! gen_exit_methods {
	( $(
		$name:ident$(<$($gen:tt),+>)?($obj:ty),
	)+ ) => {
		$(
			fn $name$(<$($gen),+>)?(&mut self, node: &$obj) {
				self.exit_node(node);
			}
		)+
	}
}

impl Visit for SelectorMatcher<'_, '_> {
	css_ast::visit::apply_queryable_visit_methods!(gen_visit_methods);
	css_ast::visit::apply_queryable_exit_methods!(gen_exit_methods);

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
			parent.visited_children.push(SiblingInfo { node_id: None, span });
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
