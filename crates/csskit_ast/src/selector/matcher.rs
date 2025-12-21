use std::collections::HashMap;

use css_ast::{
	PropertyGroup, VendorPrefixes,
	visit::{NodeId, QueryableNode, Visit, Visitable},
	*,
};
use css_lexer::{AtomSet, Span};
use css_parse::{Cursor, Declaration, DeclarationValue, NodeWithMetadata, ToSpan};
use smallvec::SmallVec;

use super::metadata::SelectorRequirements;
use super::output::MatchOutput;
use super::query::{
	QueryAttribute, QueryCombinator, QueryCompoundSelector, QueryFunctionalPseudoClass, QueryPseudoClass,
	QuerySelectorComponent, QuerySelectorList,
};
use crate::CsskitAtomSet;

/// Stores queryable property values extracted from a node.
/// Uses a small inline array since most nodes have 0-1 queryable properties.
#[derive(Clone, Default)]
struct PropertyValues(SmallVec<[(PropertyKind, Cursor); 1]>);

impl PropertyValues {
	fn from_node<T: QueryableNode>(node: &T) -> Self {
		let mut values = SmallVec::new();
		for &kind in css_ast::PROPERTY_KIND_VARIANTS {
			if let Some(cursor) = node.get_property(kind) {
				values.push((kind, cursor));
			}
		}
		Self(values)
	}

	fn get(&self, kind: PropertyKind) -> Option<Cursor> {
		self.0.iter().find(|(k, _)| *k == kind).map(|(_, c)| *c)
	}
}

/// Context for matching declarations against selectors.
/// Stores metadata directly rather than copying individual fields.
#[derive(Clone, Default)]
struct MatchContext<'a> {
	metadata: Option<CssMetadata>,
	is_important: bool,
	is_custom_property: bool,
	properties: PropertyValues,
	source: &'a str,
	sibling_index: i32,
}

#[derive(Clone)]
struct SiblingInfo {
	node_id: Option<NodeId>,
	span: Span,
}

#[derive(Clone)]
struct ParentEntry<'a> {
	node_id: Option<NodeId>,
	span: Span,
	context: MatchContext<'a>,
	visited_children: Vec<SiblingInfo>,
}

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

				let child_index = (index + 1) as i32;
				let index_from_end = (total - index) as i32;
				let (type_index, type_index_from_end, type_count) =
					if meta.needs_type_tracking && !type_info.is_empty() { type_info[index] } else { (1, 1, 1) };
				let deferred_match = self.child_matches_deferred(
					selector,
					child_index,
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

	#[allow(clippy::too_many_arguments)]
	fn child_matches_deferred(
		&self,
		selector: &QueryCompoundSelector,
		_child_index: i32,
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
		let parts = selector.parts();
		if parts.is_empty() {
			return false;
		}

		let meta = selector.metadata();
		if let Some(expected) = meta.rightmost_type_id
			&& expected != node_id
		{
			return false;
		}

		let context = MatchContext { source: self.source, sibling_index: 1, ..Default::default() };

		for part in parts {
			match part {
				QuerySelectorComponent::PseudoClass(pseudo) => {
					if matches!(
						pseudo,
						QueryPseudoClass::OnlyChild(..)
							| QueryPseudoClass::LastChild(..)
							| QueryPseudoClass::FirstOfType(..)
							| QueryPseudoClass::LastOfType(..)
							| QueryPseudoClass::OnlyOfType(..)
							| QueryPseudoClass::Empty(..)
					) {
						continue;
					}
					if !self.matches_pseudo_with_context(pseudo, node_id, &context) {
						return false;
					}
				}
				QuerySelectorComponent::FunctionalPseudoClass(pseudo) => {
					if matches!(
						pseudo,
						QueryFunctionalPseudoClass::NthLastChild(_)
							| QueryFunctionalPseudoClass::NthOfType(_)
							| QueryFunctionalPseudoClass::NthLastOfType(_)
					) {
						continue;
					}
					if !self.matches_functional_pseudo_with_context(pseudo, node_id, &context) {
						return false;
					}
				}
				_ => {}
			}
		}
		// Only match if this is a simple selector (no combinators leading to ancestors)
		!selector.metadata().structure.contains(super::metadata::SelectorStructure::HasCombinator)
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
			if self.matches_declaration_selector(selector, context) {
				self.matches.push(MatchOutput { node_id: NodeId::StyleRule, span });
			}
		}
	}

	fn matches_declaration_selector(&self, selector: &QueryCompoundSelector, context: &MatchContext) -> bool {
		let parts = selector.parts();
		let meta = selector.metadata();

		// Declaration selectors should not have combinators or type selectors
		if meta.structure.contains(super::metadata::SelectorStructure::HasCombinator)
			|| meta.structure.contains(super::metadata::SelectorStructure::HasType)
		{
			return false;
		}

		self.matches_declaration_parts(parts, context)
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
		selector: &QueryCompoundSelector,
		node_id: NodeId,
		context: &MatchContext,
	) -> bool {
		let parts = selector.parts();
		if parts.is_empty() {
			return false;
		}

		let selector_meta = selector.metadata();

		// Fast type check using pre-computed rightmost type
		if let Some(expected) = selector_meta.rightmost_type_id
			&& expected != node_id
		{
			return false;
		}

		// Fast path for simple type-only selectors (e.g., "style-rule")
		// If we have exactly one part and it's a type that already matched, we're done
		if parts.len() == 1 && selector_meta.rightmost_type_id.is_some() {
			return true;
		}

		// Early exit: check property_groups containment (all selector groups must be in node)
		if !selector_meta.property_groups.is_none() {
			let node_groups = context.metadata.map(|m| m.property_groups).unwrap_or(PropertyGroup::none());
			if !node_groups.contains(selector_meta.property_groups) {
				return false;
			}
		}

		// Early exit: check vendor_filter containment (all selector vendors must be in node)
		if !selector_meta.vendor_filter.is_none() {
			let node_vendors = context.metadata.map(|m| m.vendor_prefixes).unwrap_or(VendorPrefixes::none());
			if !node_vendors.contains(selector_meta.vendor_filter) {
				return false;
			}
		}

		// Split at the last combinator to get the rightmost simple selector
		let (ancestor_parts, rightmost_parts) = self.split_at_last_combinator(parts);

		// Check if current node matches the rightmost simple selector
		// Pass type_pre_verified=true if we already checked rightmost_type_id above
		if !self.matches_simple_parts(rightmost_parts, node_id, context, selector_meta.rightmost_type_id.is_some()) {
			return false;
		}

		if ancestor_parts.is_empty() {
			return true;
		}

		self.matches_ancestors(ancestor_parts)
	}

	fn split_at_last_combinator<'c>(
		&self,
		parts: &'c [QuerySelectorComponent<'b>],
	) -> (&'c [QuerySelectorComponent<'b>], &'c [QuerySelectorComponent<'b>]) {
		for (i, part) in parts.iter().enumerate().rev() {
			if matches!(part, QuerySelectorComponent::Combinator(_)) {
				return (&parts[..i + 1], &parts[i + 1..]);
			}
		}
		(&[], parts)
	}

	fn matches_ancestors(&self, parts: &[QuerySelectorComponent<'b>]) -> bool {
		if parts.is_empty() {
			return true;
		}

		let mut part_idx = parts.len();
		let mut parent_idx = self.parent_stack.len();

		while part_idx > 0 {
			part_idx -= 1;
			let part = &parts[part_idx];

			match part {
				QuerySelectorComponent::Combinator(combinator) => match combinator {
					QueryCombinator::Descendant(_) => {}
					QueryCombinator::Child(_) => {
						if part_idx == 0 || parent_idx == 0 {
							return false;
						}
						part_idx -= 1;
						parent_idx -= 1;
						let parent_parts = self.get_simple_parts_ending_at(parts, part_idx);
						if !self.matches_parent_entry_parts(parent_parts, &self.parent_stack[parent_idx]) {
							return false;
						}
						// Skip remaining parts of this simple selector
						part_idx = self.skip_to_prev_combinator(parts, part_idx);
					}
					QueryCombinator::NextSibling(_) => {
						if part_idx == 0 || parent_idx == 0 {
							return false;
						}
						part_idx -= 1;
						let parent_parts = self.get_simple_parts_ending_at(parts, part_idx);
						let siblings = &self.parent_stack[parent_idx - 1].visited_children;
						if siblings.is_empty() {
							return false;
						}
						if !self.matches_sibling_info_parts(parent_parts, siblings.last().unwrap()) {
							return false;
						}
						part_idx = self.skip_to_prev_combinator(parts, part_idx);
					}
					QueryCombinator::SubsequentSibling(_) => {
						if part_idx == 0 || parent_idx == 0 {
							return false;
						}
						part_idx -= 1;
						let parent_parts = self.get_simple_parts_ending_at(parts, part_idx);
						let siblings = &self.parent_stack[parent_idx - 1].visited_children;
						if !siblings.iter().any(|s| self.matches_sibling_info_parts(parent_parts, s)) {
							return false;
						}
						part_idx = self.skip_to_prev_combinator(parts, part_idx);
					}
				},
				_ => {
					// Simple selector part - find matching ancestor
					let simple_parts = self.get_simple_parts_ending_at(parts, part_idx);
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
					part_idx = self.skip_to_prev_combinator(parts, part_idx);
				}
			}
		}
		true
	}

	fn get_simple_parts_ending_at<'c>(
		&self,
		parts: &'c [QuerySelectorComponent<'b>],
		end_idx: usize,
	) -> &'c [QuerySelectorComponent<'b>] {
		let start = self.find_prev_combinator_idx(parts, end_idx).map(|i| i + 1).unwrap_or(0);
		&parts[start..=end_idx]
	}

	fn find_prev_combinator_idx(&self, parts: &[QuerySelectorComponent<'b>], from: usize) -> Option<usize> {
		for i in (0..from).rev() {
			if matches!(parts[i], QuerySelectorComponent::Combinator(_)) {
				return Some(i);
			}
		}
		None
	}

	fn skip_to_prev_combinator(&self, parts: &[QuerySelectorComponent<'b>], from: usize) -> usize {
		self.find_prev_combinator_idx(parts, from).unwrap_or(0)
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
				return Some(t.node_id);
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
		// Should not have a type selector
		if parts.iter().any(|p| matches!(p, QuerySelectorComponent::Type(_))) {
			return false;
		}

		let mut has_meaningful_selector = false;
		let meta = context.metadata.as_ref();

		for part in parts {
			match part {
				QuerySelectorComponent::Attribute(attr) => {
					has_meaningful_selector = true;
					if !self.matches_attribute(attr, context) {
						return false;
					}
				}
				QuerySelectorComponent::PseudoClass(pseudo) => {
					let (is_decl_pseudo, matches) = self.check_declaration_pseudo(pseudo, context, meta);
					if is_decl_pseudo {
						has_meaningful_selector = true;
						if !matches {
							return false;
						}
					}
				}
				QuerySelectorComponent::FunctionalPseudoClass(pseudo) => {
					let (is_decl_pseudo, matches) = self.check_declaration_functional_pseudo(pseudo, context, meta);
					if is_decl_pseudo {
						has_meaningful_selector = true;
						if !matches {
							return false;
						}
					}
				}
				QuerySelectorComponent::Wildcard(_) => {}
				_ => {}
			}
		}

		has_meaningful_selector
	}

	fn check_declaration_pseudo(
		&self,
		pseudo: &QueryPseudoClass,
		context: &MatchContext,
		meta: Option<&CssMetadata>,
	) -> (bool, bool) {
		match pseudo {
			QueryPseudoClass::Important(_, _) => (true, context.is_important),
			QueryPseudoClass::Custom(_, _) => (true, context.is_custom_property),
			QueryPseudoClass::Computed(_, _) => (true, meta.is_some_and(|m| m.has_computed())),
			QueryPseudoClass::Shorthand(_, _) => (true, meta.is_some_and(|m| m.has_shorthands())),
			QueryPseudoClass::Longhand(_, _) => (true, meta.is_some_and(|m| m.has_longhands())),
			QueryPseudoClass::Unknown(_, _) => (true, meta.is_some_and(|m| m.has_unknown())),
			QueryPseudoClass::Prefixed(_, _) => (true, Self::is_prefixed_decl(meta, context, None)),
			_ => (false, true),
		}
	}

	fn check_declaration_functional_pseudo(
		&self,
		pseudo: &QueryFunctionalPseudoClass,
		context: &MatchContext,
		meta: Option<&CssMetadata>,
	) -> (bool, bool) {
		match pseudo {
			QueryFunctionalPseudoClass::Prefixed(p) => {
				let cursor: Cursor = p.vendor.into();
				let filter = cursor.str_slice(self.selector_source);
				(true, Self::is_prefixed_decl(meta, context, Some(filter)))
			}
			QueryFunctionalPseudoClass::PropertyType(p) => {
				let cursor: Cursor = p.group.into();
				let atom = CsskitAtomSet::from_bits(cursor.atom_bits());
				let matches = atom.to_property_group().is_some_and(|group| Self::matches_property_group(meta, group));
				(true, matches)
			}
			_ => (false, true),
		}
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
					if !type_pre_verified && t.node_id != node_id {
						return false;
					}
				}
				QuerySelectorComponent::Wildcard(_) => {}
				QuerySelectorComponent::Attribute(attr) => {
					if !self.matches_attribute(attr, context) {
						return false;
					}
				}
				QuerySelectorComponent::PseudoClass(pseudo) => {
					if !self.matches_pseudo_with_context(pseudo, node_id, context) {
						return false;
					}
				}
				QuerySelectorComponent::FunctionalPseudoClass(pseudo) => {
					if !self.matches_functional_pseudo_with_context(pseudo, node_id, context) {
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

	fn matches_pseudo_with_context(&self, pseudo: &QueryPseudoClass, node_id: NodeId, context: &MatchContext) -> bool {
		let meta = context.metadata.as_ref();
		match pseudo {
			QueryPseudoClass::AtRule(_, _) => Self::is_at_rule(meta),
			QueryPseudoClass::Computed(_, _) => meta.is_some_and(|m| m.has_computed()),
			QueryPseudoClass::Custom(_, _) => context.is_custom_property,
			QueryPseudoClass::FirstChild(_, _) => context.sibling_index == 1,
			QueryPseudoClass::Function(_, _) => Self::is_function(meta, node_id),
			QueryPseudoClass::Important(_, _) => context.is_important,
			QueryPseudoClass::Longhand(_, _) => meta.is_some_and(|m| m.has_longhands()),
			QueryPseudoClass::Nested(_, _) => self.parent_stack.iter().any(|p| p.node_id == Some(NodeId::StyleRule)),
			QueryPseudoClass::Prefixed(_, _) => Self::is_prefixed(meta, None),
			QueryPseudoClass::Root(_, _) => self.parent_stack.is_empty(),
			QueryPseudoClass::Rule(_, _) => Self::is_rule(meta),
			QueryPseudoClass::Shorthand(_, _) => meta.is_some_and(|m| m.has_shorthands()),
			// TODO: Remove tag name fallback once all unknown nodes set DeclarationKind::Unknown in metadata
			QueryPseudoClass::Unknown(_, _) => {
				meta.is_some_and(|m| m.has_unknown()) || node_id.tag_name().contains("unknown")
			}
			QueryPseudoClass::OnlyChild(_, _)
			| QueryPseudoClass::LastChild(_, _)
			| QueryPseudoClass::FirstOfType(_, _)
			| QueryPseudoClass::LastOfType(_, _)
			| QueryPseudoClass::OnlyOfType(_, _)
			| QueryPseudoClass::Empty(_, _) => false, // Handled by deferred matching
		}
	}

	fn matches_functional_pseudo_with_context(
		&self,
		pseudo: &QueryFunctionalPseudoClass,
		node_id: NodeId,
		context: &MatchContext,
	) -> bool {
		let meta = context.metadata.as_ref();
		match pseudo {
			QueryFunctionalPseudoClass::Not(not_pseudo) => {
				let inner_type = self.get_type_from_parts(not_pseudo.selector.parts());
				if let Some(expected) = inner_type {
					return expected != node_id;
				}
				true
			}
			QueryFunctionalPseudoClass::NthChild(p) => p.value.matches(context.sibling_index),
			QueryFunctionalPseudoClass::NthLastChild(_)
			| QueryFunctionalPseudoClass::NthOfType(_)
			| QueryFunctionalPseudoClass::NthLastOfType(_) => false, // Handled by deferred matching
			QueryFunctionalPseudoClass::PropertyType(p) => {
				let cursor: Cursor = p.group.into();
				let atom = CsskitAtomSet::from_bits(cursor.atom_bits());
				atom.to_property_group().is_some_and(|group| Self::matches_property_group(meta, group))
			}
			QueryFunctionalPseudoClass::Prefixed(p) => {
				let cursor: Cursor = p.vendor.into();
				let atom = CsskitAtomSet::from_bits(cursor.atom_bits());
				atom.to_vendor_prefix().is_some_and(|vendor| Self::is_prefixed_filter(meta, vendor))
			}
		}
	}

	fn is_at_rule(meta: Option<&CssMetadata>) -> bool {
		meta.is_some_and(|m| m.node_kinds.contains(NodeKinds::AtRule))
	}

	fn is_rule(meta: Option<&CssMetadata>) -> bool {
		meta.is_some_and(|m| m.node_kinds.intersects(NodeKinds::StyleRule | NodeKinds::AtRule))
	}

	fn is_function(meta: Option<&CssMetadata>, node_id: NodeId) -> bool {
		if meta.is_some_and(|m| m.has_functions()) {
			return true;
		}
		// TODO: Fallback to string matching for nodes that don't yet set NodeKinds::Function, for now.
		node_id.tag_name().ends_with("-function")
	}

	fn is_prefixed(meta: Option<&CssMetadata>, filter: Option<&str>) -> bool {
		// Check metadata for vendor prefix (works for both nodes and declarations)
		// Now that all queryable nodes implement NodeWithMetadata<CssMetadata>, metadata is always available
		let Some(prefix) = meta.and_then(|m| m.single_vendor_prefix()).and_then(CsskitAtomSet::from_vendor_prefix)
		else {
			return false;
		};
		filter.is_none_or(|f| prefix == CsskitAtomSet::from_str(f))
	}

	/// Check if a declaration is vendor-prefixed.
	/// First checks metadata, then falls back to checking property name string for unknown properties.
	fn is_prefixed_decl(meta: Option<&CssMetadata>, context: &MatchContext, filter: Option<&str>) -> bool {
		if meta.is_some_and(|m| !m.vendor_prefixes.is_none()) {
			let Some(prefix) = meta.and_then(|m| m.single_vendor_prefix()).and_then(CsskitAtomSet::from_vendor_prefix)
			else {
				return false;
			};
			return filter.is_none_or(|f| prefix == CsskitAtomSet::from_str(f));
		}

		// TODO: Remove this fallback once all vendor-prefixed properties set VendorPrefixes in metadata
		let Some(cursor) = context.properties.get(PropertyKind::Name) else { return false };
		let name: &str = cursor.str_slice(context.source);
		if !name.starts_with('-') {
			return false;
		}
		let Some(end) = name[1..].find('-') else { return false };
		// end > 0 excludes CSS custom properties (--foo) which would have end = 0
		if end == 0 {
			return false;
		}
		let prefix = &name[1..1 + end];
		filter.is_none_or(|f| prefix.eq_ignore_ascii_case(f))
	}

	/// Check if metadata has a specific vendor prefix (pre-computed filter).
	#[inline]
	fn is_prefixed_filter(meta: Option<&CssMetadata>, filter: VendorPrefixes) -> bool {
		meta.is_some_and(|m| m.vendor_prefixes.contains(filter))
	}

	/// Check if metadata contains a specific property group (pre-computed).
	#[inline]
	fn matches_property_group(meta: Option<&CssMetadata>, group: PropertyGroup) -> bool {
		meta.is_some_and(|m| m.property_groups.contains(group))
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
			properties: PropertyValues(smallvec::smallvec![(PropertyKind::Name, node.name.into())]),
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
mod tests {
	use super::*;
	use crate::assert_query;

	#[test]
	fn match_style_rule() {
		let matches = assert_query!("a { color: red; }", "style-rule", 1);
		assert_eq!(matches[0].node_id, NodeId::StyleRule);
		assert!(!matches[0].span.is_empty());
	}

	#[test]
	fn match_selector_list() {
		assert_query!("a, b { color: red; }", "selector-list", 1);
	}

	#[test]
	fn match_multiple_selectors() {
		assert_query!("a { color: red; } @media screen {}", "style-rule, media-rule", 2);
	}

	#[test]
	fn descendant_combinator() {
		let matches = assert_query!("a { color: red; }", "style-rule selector-list", 1);
		assert_eq!(matches[0].node_id, NodeId::SelectorList);
	}

	#[test]
	fn descendant_combinator_no_match() {
		assert_query!("@media screen {}", "style-rule selector-list", 0);
	}

	#[test]
	fn nested_descendant() {
		let matches = assert_query!("@media screen { a { color: red; } }", "media-rule style-rule selector-list", 1);
		assert_eq!(matches[0].node_id, NodeId::SelectorList);
	}

	#[test]
	fn child_combinator() {
		let matches = assert_query!("a { color: red; }", "style-rule > selector-list", 1);
		assert_eq!(matches[0].node_id, NodeId::SelectorList);
	}

	#[test]
	fn child_combinator_no_match() {
		assert_query!("a { color: red; }", "style-sheet > selector-list", 0);
	}

	#[test]
	fn next_sibling_combinator() {
		assert_query!("a {} b {}", "style-rule + style-rule", 1);
	}

	#[test]
	fn next_sibling_combinator_no_match() {
		assert_query!("a {}", "style-rule + style-rule", 0);
	}

	#[test]
	fn next_sibling_combinator_different_types() {
		let matches = assert_query!("@media screen {} a {}", "media-rule + style-rule", 1);
		assert_eq!(matches[0].node_id, NodeId::StyleRule);
	}

	#[test]
	fn next_sibling_combinator_wrong_order() {
		assert_query!("a {} @media screen {}", "media-rule + style-rule", 0);
	}

	#[test]
	fn subsequent_sibling_combinator() {
		assert_query!("a {} b {} c {}", "style-rule ~ style-rule", 2);
	}

	#[test]
	fn subsequent_sibling_combinator_with_gap() {
		let matches = assert_query!("@media screen {} @keyframes foo {} a {}", "media-rule ~ style-rule", 1);
		assert_eq!(matches[0].node_id, NodeId::StyleRule);
	}

	#[test]
	fn subsequent_sibling_combinator_no_match() {
		assert_query!("a {} @media screen {}", "media-rule ~ style-rule", 0);
	}

	#[test]
	fn match_custom_properties() {
		assert_query!("a { --my-color: red; color: blue; --spacing: 10px; }", "*:custom", 2);
	}

	#[test]
	fn no_match_custom_on_regular_properties() {
		assert_query!("a { color: red; background: blue; }", "*:custom", 0);
	}

	#[test]
	fn attribute_name_selector() {
		assert_query!("a { color: red; background: blue; margin: 10px; }", "[name=color]", 1);
	}

	#[test]
	fn attribute_name_selector_multiple() {
		assert_query!("a { color: red; } b { color: blue; background: green; }", "[name=color]", 2);
	}

	#[test]
	fn attribute_name_selector_quoted() {
		assert_query!("a { background-color: red; }", "[name='background-color']", 1);
	}

	#[test]
	fn attribute_name_selector_no_match() {
		assert_query!("a { color: red; background: blue; }", "[name=margin]", 0);
	}

	#[test]
	fn attribute_name_case_insensitive() {
		assert_query!("a { COLOR: red; }", "[name=color]", 1);
	}

	#[test]
	fn first_child() {
		assert_query!("a {} b {} c {}", "style-rule:first-child", 1);
	}

	#[test]
	fn first_child_no_match() {
		assert_query!("@media screen {} a {}", "style-rule:first-child", 0);
	}

	#[test]
	fn nth_child_index() {
		assert_query!("a {} b {} c {} d {}", "style-rule:nth-child(2)", 1);
	}

	#[test]
	fn nth_child_odd() {
		assert_query!("a {} b {} c {} d {}", "style-rule:nth-child(odd)", 2);
	}

	#[test]
	fn nth_child_even() {
		assert_query!("a {} b {} c {} d {}", "style-rule:nth-child(even)", 2);
	}

	#[test]
	fn nth_child_formula() {
		assert_query!("a {} b {} c {} d {} e {} f {}", "style-rule:nth-child(3n)", 2);
	}

	#[test]
	fn nth_child_formula_with_offset() {
		assert_query!("a {} b {} c {} d {} e {} f {}", "style-rule:nth-child(2n+1)", 3);
	}

	#[test]
	fn only_child() {
		assert_query!("a {}", "style-rule:only-child", 1);
	}

	#[test]
	fn only_child_no_match() {
		assert_query!("a {} b {}", "style-rule:only-child", 0);
	}

	#[test]
	fn style_rules_in_media() {
		assert_query!("@media screen { a {} b {} }", "media-rule style-rule", 2);
	}

	#[test]
	fn last_child() {
		assert_query!("a {} b {} c {}", "style-rule:last-child", 1);
	}

	#[test]
	fn last_child_no_match() {
		// style-rule is not last (media-rule is)
		assert_query!("a {} @media screen {}", "style-rule:last-child", 0);
	}

	#[test]
	fn last_child_single() {
		// Single child is both first and last
		assert_query!("a {}", "style-rule:last-child", 1);
	}

	#[test]
	fn nth_last_child_index() {
		assert_query!("a {} b {} c {}", "style-rule:nth-last-child(1)", 1);
	}

	#[test]
	fn nth_last_child_second() {
		assert_query!("a {} b {} c {}", "style-rule:nth-last-child(2)", 1);
	}

	#[test]
	fn nth_last_child_odd() {
		assert_query!("a {} b {} c {} d {}", "style-rule:nth-last-child(odd)", 2);
	}

	#[test]
	fn nth_last_child_even() {
		assert_query!("a {} b {} c {} d {}", "style-rule:nth-last-child(even)", 2);
	}

	#[test]
	fn nth_last_child_formula() {
		assert_query!("a {} b {} c {} d {} e {} f {}", "style-rule:nth-last-child(2n)", 3);
	}

	#[test]
	fn first_of_type() {
		assert_query!("@media screen {} a {} b {}", "style-rule:first-of-type", 1);
	}

	#[test]
	fn first_of_type_is_first() {
		assert_query!("a {} b {} @media screen {}", "style-rule:first-of-type", 1);
	}

	#[test]
	fn last_of_type() {
		assert_query!("a {} b {} @media screen {}", "style-rule:last-of-type", 1);
	}

	#[test]
	fn last_of_type_at_end() {
		assert_query!("@media screen {} a {} b {}", "style-rule:last-of-type", 1);
	}

	#[test]
	fn only_of_type() {
		assert_query!("@media screen {} a {} @keyframes foo {}", "style-rule:only-of-type", 1);
	}

	#[test]
	fn only_of_type_no_match() {
		assert_query!("a {} b {}", "style-rule:only-of-type", 0);
	}

	#[test]
	fn nth_of_type() {
		assert_query!("@media screen {} a {} b {} c {}", "style-rule:nth-of-type(2)", 1);
	}

	#[test]
	fn nth_of_type_odd() {
		assert_query!("a {} b {} c {} d {}", "style-rule:nth-of-type(odd)", 2);
	}

	#[test]
	fn nth_last_of_type() {
		assert_query!("a {} b {} c {} @media screen {}", "style-rule:nth-last-of-type(2)", 1);
	}

	#[test]
	fn nth_last_of_type_even() {
		assert_query!("a {} b {} c {} d {}", "style-rule:nth-last-of-type(even)", 2);
	}

	#[test]
	fn root() {
		assert_query!("a {}", "style-sheet:root", 1);
	}

	#[test]
	fn root_no_match() {
		assert_query!("a {}", "style-rule:root", 0);
	}

	#[test]
	fn at_rule() {
		assert_query!("@media screen {} a {} @keyframes foo {}", "*:at-rule", 2);
	}

	#[test]
	fn rule() {
		assert_query!("@media screen {} a {}", "*:rule", 2);
	}

	#[test]
	fn function() {
		assert_query!("a { color: rgb(255, 0, 0); }", "*:function", 1);
	}

	#[test]
	fn function_multiple() {
		// linear-gradient and rotate are both functions
		assert_query!("a { background: linear-gradient(red, blue); transform: rotate(45deg); }", "*:function", 2);
	}

	#[test]
	fn at_rule_not_style_rule() {
		assert_query!("a {}", "*:at-rule", 0);
	}

	#[test]
	fn prefixed_declaration() {
		assert_query!("a { -webkit-transform: rotate(45deg); }", "*:prefixed", 1);
	}

	#[test]
	fn prefixed_declaration_multiple() {
		assert_query!("a { -webkit-transform: rotate(45deg); -moz-appearance: none; }", "*:prefixed", 2);
	}

	#[test]
	fn prefixed_declaration_filter_webkit() {
		assert_query!("a { -webkit-transform: rotate(45deg); -moz-appearance: none; }", "*:prefixed(webkit)", 1);
	}

	#[test]
	fn prefixed_declaration_filter_moz() {
		assert_query!("a { -webkit-transform: rotate(45deg); -moz-appearance: none; }", "*:prefixed(moz)", 1);
	}

	#[test]
	fn prefixed_no_match_regular() {
		// Regular properties should not match :prefixed
		assert_query!("a { color: red; margin: 10px; }", "*:prefixed", 0);
	}

	#[test]
	fn prefixed_no_match_custom_properties() {
		// CSS custom properties (--foo) should not match :prefixed
		assert_query!("a { --animate-duration: 1s; --animate-delay: 1s; }", "*:prefixed", 0);
	}

	#[test]
	fn prefixed_unknown_property_filter() {
		assert_query!("a { -webkit-animation-duration: 1s; -moz-unknown: value; }", "*:prefixed(webkit)", 1);
		assert_query!("a { -webkit-animation-duration: 1s; -moz-unknown: value; }", "*:prefixed(moz)", 1);
	}

	#[test]
	fn prefixed_unknown_multiple() {
		assert_query!("a { -webkit-animation-duration: 1s; -webkit-animation-delay: 2s; }", "*:prefixed", 2);
	}

	#[test]
	fn prefixed_node_webkit_keyframes() {
		assert_query!("@-webkit-keyframes spin { to { opacity: 1; } }", "webkit-keyframes-rule:prefixed", 1);
	}

	#[test]
	fn prefixed_node_filter() {
		assert_query!("@-webkit-keyframes spin { to { opacity: 1; } }", "*:prefixed(webkit)", 1);
	}

	// :shorthand and :longhand tests
	#[test]
	fn shorthand() {
		assert_query!("a { margin: 10px; }", "*:shorthand", 1);
	}

	#[test]
	fn shorthand_multiple() {
		assert_query!("a { margin: 10px; padding: 5px; border: 1px solid; }", "*:shorthand", 3);
	}

	#[test]
	fn longhand() {
		assert_query!("a { color: red; padding-top: 5px; }", "*:longhand", 2);
	}

	#[test]
	fn longhand_not_shorthand() {
		assert_query!("a { margin: 10px; }", "*:longhand", 0);
	}

	#[test]
	fn property_type_color() {
		assert_query!("a { color: red; margin: 10px; }", "*:property-type(color)", 1);
	}

	#[test]
	fn property_type_sizing() {
		assert_query!("a { width: 100px; height: 50px; color: red; }", "*:property-type(sizing)", 2);
	}

	#[test]
	fn property_type_animation() {
		assert_query!(
			"a { animation-name: spin; animation-duration: 1s; color: red; }",
			"*:property-type(animation)",
			2
		);
	}

	#[test]
	fn nested_style_rule() {
		assert_query!("a { & b { color: red; } }", "style-rule:nested", 1);
	}

	#[test]
	fn nested_not_top_level() {
		assert_query!("a { color: red; }", "style-rule:nested", 0);
	}

	#[test]
	fn supports_rule() {
		assert_query!("@supports (color: red) { a { color: red; } }", "supports-rule", 1);
	}

	#[test]
	fn supports_condition() {
		assert_query!("@supports (color: red) { a {} }", "supports-condition", 1);
	}

	#[test]
	fn supports_condition_not() {
		assert_query!("@supports not (color: red) { a {} }", "supports-condition", 1);
	}

	#[test]
	fn supports_feature() {
		assert_query!("@supports (color: red) { a {} }", "supports-feature", 1);
	}

	#[test]
	fn supports_descendant() {
		assert_query!("@supports (color: red) { a {} }", "supports-rule style-rule", 1);
	}

	#[test]
	fn supports_nested_media() {
		assert_query!("@supports (color: red) { @media screen { a {} } }", "supports-rule media-rule style-rule", 1);
	}

	#[test]
	fn container_rule() {
		assert_query!("@container (width > 400px) { a { color: red; } }", "container-rule", 1);
	}

	#[test]
	fn container_query() {
		assert_query!("@container (width > 400px) { a {} }", "container-query", 1);
	}

	#[test]
	fn container_query_named() {
		assert_query!("@container sidebar (width > 400px) { a {} }", "container-query", 1);
	}

	#[test]
	fn container_descendant() {
		assert_query!("@container (width > 400px) { a {} }", "container-rule style-rule", 1);
	}

	#[test]
	fn container_nested_supports() {
		assert_query!(
			"@container (width > 400px) { @supports (color: red) { a {} } }",
			"container-rule supports-rule",
			1
		);
	}

	#[test]
	fn not_type_selector() {
		// :not(media-rule) matches all nodes except MediaRule
		assert_query!("a {} @media screen {} b {}", "*", 13);
		assert_query!("a {} @media screen {} b {}", "media-rule", 1);
		assert_query!("a {} @media screen {} b {}", ":not(media-rule)", 12);
	}

	#[test]
	fn not_excludes_type() {
		// :not(style-rule) matches all nodes except StyleRule
		assert_query!("a {} @media screen {} b {}", "*", 13);
		assert_query!("a {} @media screen {} b {}", "style-rule", 2);
		assert_query!("a {} @media screen {} b {}", ":not(style-rule)", 11);
	}

	#[test]
	fn universal_matches_all() {
		assert_query!("a { color: red; }", "*", 9);
	}

	#[test]
	fn universal_with_pseudo() {
		assert_query!("a {} b {}", "*:first-child", 10);
	}

	#[test]
	fn universal_descendant() {
		assert_query!("a { color: red; }", "style-rule *", 7);
	}

	#[test]
	fn important() {
		assert_query!("a { color: red !important; }", "*:important", 1);
	}

	#[test]
	fn important_multiple() {
		assert_query!("a { color: red !important; margin: 10px; padding: 5px !important; }", "*:important", 2);
	}

	#[test]
	fn important_no_match() {
		assert_query!("a { color: red; margin: 10px; }", "*:important", 0);
	}

	#[test]
	fn important_combined_with_name() {
		assert_query!("a { color: red !important; margin: 10px !important; }", "[name=color]:important", 1);
	}

	#[test]
	fn triple_descendant() {
		assert_query!("@media screen { @supports (color: red) { a {} } }", "media-rule supports-rule style-rule", 1);
	}

	#[test]
	fn mixed_combinators() {
		assert_query!("@media screen { a {} b {} }", "media-rule > style-rule selector-list", 2);
	}

	#[test]
	fn sibling_after_descendant() {
		assert_query!("@media screen { a {} b {} }", "media-rule style-rule + style-rule", 1);
	}

	#[test]
	fn child_chain() {
		assert_query!("a { color: red; }", "style-sheet > style-rule > selector-list", 1);
	}

	#[test]
	fn empty_stylesheet() {
		assert_query!("", "style-rule", 0);
	}

	#[test]
	fn comments_only() {
		assert_query!("/* comment */", "style-rule", 0);
	}

	#[test]
	fn whitespace_only() {
		assert_query!("   \n\t   ", "style-rule", 0);
	}

	#[test]
	fn deeply_nested_media() {
		assert_query!("@media screen { @media print { a {} } }", "media-rule media-rule style-rule", 1);
	}

	#[test]
	fn multiple_selector_list() {
		assert_query!("a {} @media screen {} b {}", "style-rule, media-rule", 3);
	}

	#[test]
	fn attribute_with_pseudo() {
		assert_query!("a { color: red !important; margin: 10px !important; }", "[name=margin]:important", 1);
	}

	#[test]
	fn property_type_backgrounds() {
		assert_query!("a { background-color: red; }", "*:property-type(backgrounds)", 1);
	}

	#[test]
	fn declaration_in_keyframes() {
		assert_query!("@keyframes spin { from { opacity: 0; } to { opacity: 1; } }", "[name=opacity]", 2);
	}

	#[test]
	fn declaration_in_font_face() {
		assert_query!("@font-face { font-family: 'Custom'; src: url('font.woff'); }", "[name=font-family]", 1);
	}

	#[test]
	fn custom_property_in_root() {
		assert_query!(":root { --primary: blue; }", "*:custom", 1);
	}

	#[test]
	fn color_function_rgb() {
		assert_query!("a { color: rgb(255, 0, 0); }", "color-function", 1);
	}

	#[test]
	fn color_function_hsl() {
		assert_query!("a { color: hsl(120, 100%, 50%); }", "color-function", 1);
	}

	#[test]
	fn color_function_multiple() {
		assert_query!("a { color: rgb(255, 0, 0); background-color: hsl(120, 100%, 50%); }", "color-function", 2);
	}

	#[test]
	fn linear_gradient() {
		assert_query!("a { background-image: linear-gradient(red, blue); }", "linear-gradient-function", 1);
	}

	#[test]
	fn url_in_background_image() {
		assert_query!("a { background-image: url('image.png'); }", "url", 1);
	}

	#[test]
	fn computed_with_calc() {
		assert_query!("a { width: calc(100% - 20px); }", "*:computed", 1);
	}

	#[test]
	fn computed_with_var() {
		assert_query!("a { color: var(--primary); }", "*:computed", 1);
	}

	#[test]
	fn computed_no_match() {
		assert_query!("a { color: red; width: 100px; }", "*:computed", 0);
	}

	#[test]
	fn unknown_property() {
		assert_query!("a { not-a-real-property: value; }", "*:unknown", 1);
	}

	#[test]
	fn unknown_no_match() {
		assert_query!("a { color: red; margin: 10px; }", "*:unknown", 0);
	}

	#[test]
	fn important_early_exit() {
		assert_query!("a { color: red; margin: 10px; }", "*:important", 0);
	}

	#[test]
	fn custom_early_exit() {
		assert_query!("a { color: red; }", "*:custom", 0);
	}

	#[test]
	fn shorthand_early_exit() {
		assert_query!("a { margin-top: 10px; }", "*:shorthand", 0);
	}

	#[test]
	fn prefixed_early_exit() {
		assert_query!("a { transform: rotate(45deg); }", "*:prefixed", 0);
	}

	#[test]
	fn multiple_selectors_partial_filter() {
		assert_query!("a { color: red; }", "*:important, style-rule", 1);
	}

	#[test]
	fn all_selectors_filtered_out() {
		assert_query!("a { color: red; }", "*:important, *:custom", 0);
	}

	#[test]
	fn at_rule_early_exit() {
		assert_query!("a { color: red; }", "*:at-rule", 0);
	}

	#[test]
	fn keyframes_name_attribute() {
		assert_query!("@keyframes spin { to { opacity: 1; } }", "keyframes-rule[name=spin]", 1);
	}

	#[test]
	fn keyframes_name_attribute_no_match() {
		assert_query!("@keyframes spin { to { opacity: 1; } }", "keyframes-rule[name=bounce]", 0);
	}

	#[test]
	fn webkit_keyframes_name_attribute() {
		assert_query!("@-webkit-keyframes spin { to { opacity: 1; } }", "webkit-keyframes-rule[name=spin]", 1);
	}

	#[test]
	fn property_rule_name_attribute() {
		assert_query!(
			"@property --my-color { syntax: '<color>'; inherits: false; initial-value: red; }",
			"property-rule[name='--my-color']",
			1
		);
	}

	#[test]
	fn name_attribute_early_exit_no_named_nodes() {
		assert_query!("a { color: red; }", "style-rule[name=foo]", 0);
	}

	#[test]
	fn container_rule_name_attribute() {
		assert_query!("@container sidebar (width > 400px) { a {} }", "container-rule[name=sidebar]", 1);
	}

	#[test]
	fn container_rule_name_attribute_no_match() {
		assert_query!("@container sidebar (width > 400px) { a {} }", "container-rule[name=main]", 0);
	}

	#[test]
	fn container_rule_unnamed() {
		assert_query!("@container (width > 400px) { a {} }", "container-rule[name]", 0);
	}

	#[test]
	fn container_rule_named_presence() {
		assert_query!("@container sidebar (width > 400px) { a {} }", "container-rule[name]", 1);
	}

	#[test]
	fn keyframes_rule_name_presence() {
		assert_query!("@keyframes spin { to { opacity: 1; } }", "keyframes-rule[name]", 1);
	}

	#[test]
	fn attribute_prefix_operator() {
		assert_query!("a { background-color: red; }", "[name^=background]", 1);
		assert_query!("a { color: red; }", "[name^=background]", 0);
	}

	#[test]
	fn attribute_suffix_operator() {
		assert_query!("a { background-color: red; }", "[name$=color]", 1);
		assert_query!("a { background-image: url(x); }", "[name$=color]", 0);
	}

	#[test]
	fn attribute_contains_operator() {
		assert_query!("a { background-color: red; }", "[name*=ground]", 1);
		assert_query!("a { color: red; }", "[name*=ground]", 0);
	}

	#[test]
	fn attribute_spacelist_operator() {
		// For property names this is less common, but we test it works
		assert_query!("@keyframes slide-in { to { opacity: 1; } }", "keyframes-rule[name~=slide-in]", 1);
	}

	#[test]
	fn attribute_langprefix_operator() {
		assert_query!("@keyframes slide-in { to { opacity: 1; } }", "keyframes-rule[name|=slide]", 1);
		assert_query!("@keyframes slide { to { opacity: 1; } }", "keyframes-rule[name|=slide]", 1);
		assert_query!("@keyframes slideshow { to { opacity: 1; } }", "keyframes-rule[name|=slide]", 0);
	}

	#[test]
	fn attribute_operators_case_insensitive() {
		assert_query!("a { BACKGROUND-COLOR: red; }", "[name^=background]", 1);
		assert_query!("a { BACKGROUND-COLOR: red; }", "[name$=color]", 1);
		assert_query!("a { BACKGROUND-COLOR: red; }", "[name*=ground]", 1);
	}

	#[test]
	fn attribute_prefix_multiple_matches() {
		assert_query!("a { background-color: red; background-image: url(x); color: blue; }", "[name^=background]", 2);
	}

	#[test]
	fn attribute_suffix_multiple_matches() {
		assert_query!("a { background-color: red; border-color: blue; color: green; }", "[name$=color]", 3);
	}

	#[test]
	fn attribute_contains_multiple_matches() {
		assert_query!("a { margin-top: 1px; margin-bottom: 2px; padding: 3px; }", "[name*=margin]", 2);
	}
}
