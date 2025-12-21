use css_ast::{
	visit::{NodeId, QueryableNode, Visit, Visitable},
	*,
};
use css_lexer::{AtomSet, Span};
use css_parse::{Cursor, Declaration, DeclarationValue, NodeWithMetadata, ToSpan};

use super::output::MatchOutput;
use super::query::{
	QueryAttribute, QueryCombinator, QueryCompoundSelector, QueryFunctionalPseudoClass, QueryPseudoClass,
	QuerySelectorComponent, QuerySelectorList,
};

use crate::CsskitAtomSet;

fn parse_vendor_prefix(name: &str) -> Option<CsskitAtomSet> {
	let name = name.strip_prefix('-').unwrap_or(name);
	if name.starts_with("webkit") {
		Some(CsskitAtomSet::Webkit)
	} else if name.starts_with("moz") {
		Some(CsskitAtomSet::Moz)
	} else if name.starts_with("ms") {
		Some(CsskitAtomSet::Ms)
	} else if name.starts_with("o-") || name == "o" {
		Some(CsskitAtomSet::O)
	} else {
		None
	}
}

#[derive(Default, Clone)]
struct MatchContext<'a> {
	is_important: bool,
	is_custom_property: bool,
	is_computed: bool,
	is_shorthand: bool,
	is_longhand: bool,
	is_unknown: bool,
	property_group: Option<css_ast::PropertyGroup>,
	property_name: Option<Cursor>,
	source: &'a str,
	sibling_index: i32,
	vendor_prefix: Option<CsskitAtomSet>,
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

#[derive(Default)]
struct DeferredNeeds {
	only_child: bool,
	last_child: bool,
	nth_last_child: Option<Nth>,
	first_of_type: bool,
	last_of_type: bool,
	only_of_type: bool,
	nth_of_type: Option<Nth>,
	nth_last_of_type: Option<Nth>,
	empty: bool,
}

impl DeferredNeeds {
	fn any(&self) -> bool {
		self.only_child
			|| self.last_child
			|| self.nth_last_child.is_some()
			|| self.first_of_type
			|| self.last_of_type
			|| self.only_of_type
			|| self.nth_of_type.is_some()
			|| self.nth_last_of_type.is_some()
			|| self.empty
	}

	fn needs_type_tracking(&self) -> bool {
		self.first_of_type
			|| self.last_of_type
			|| self.only_of_type
			|| self.nth_of_type.is_some()
			|| self.nth_last_of_type.is_some()
	}
}

pub struct SelectorMatcher<'a, 'b> {
	selectors: &'a QuerySelectorList<'b>,
	selector_source: &'b str,
	source: &'a str,
	matches: Vec<MatchOutput>,
	parent_stack: Vec<ParentEntry<'a>>,
}

impl<'a, 'b> SelectorMatcher<'a, 'b> {
	pub fn new(selectors: &'a QuerySelectorList<'b>, selector_source: &'b str, source: &'a str) -> Self {
		Self { selectors, selector_source, source, matches: Vec::new(), parent_stack: Vec::new() }
	}

	pub fn run<T: Visitable>(mut self, root: &T) -> Vec<MatchOutput> {
		root.accept(&mut self);
		self.matches
	}

	fn check_match<T: QueryableNode + ToSpan>(&mut self, node: &T) {
		let node_id = T::NODE_ID;
		let span = node.to_span();
		let sibling_index = self.parent_stack.last().map(|p| p.visited_children.len() as i32 + 1).unwrap_or(1);
		let context = MatchContext { source: self.source, sibling_index, ..Default::default() };
		self.check_match_with_context(node_id, span, &context);
		if let Some(parent) = self.parent_stack.last_mut() {
			parent.visited_children.push(SiblingInfo { node_id: Some(node_id), span });
		}
		self.parent_stack.push(ParentEntry { node_id: Some(node_id), span, context, visited_children: Vec::new() });
	}

	fn exit_node<T: QueryableNode>(&mut self, _node: &T) {
		if let Some(exiting) = self.parent_stack.last() {
			let children = exiting.visited_children.clone();
			self.check_deferred_matches(&children);
		}
		self.parent_stack.pop();
	}

	fn check_deferred_matches(&mut self, children: &[SiblingInfo]) {
		let total = children.len();
		for selector in self.selectors.selectors() {
			let needs = self.get_deferred_pseudo_needs(selector);
			if !needs.any() {
				continue;
			}
			if needs.empty && total == 0 {
				continue;
			}

			let type_info = if needs.needs_type_tracking() { self.compute_type_indices(children) } else { Vec::new() };

			for (index, child) in children.iter().enumerate() {
				let Some(node_id) = child.node_id else {
					continue;
				};

				let child_index = (index + 1) as i32;
				let index_from_end = (total - index) as i32;
				let (type_index, type_index_from_end, type_count) =
					if !type_info.is_empty() { type_info[index] } else { (1, 1, 1) };
				if self.child_matches_deferred(
					&needs,
					node_id,
					child_index,
					index_from_end,
					total,
					type_index,
					type_index_from_end,
					type_count,
				) && self.matches_deferred_selector(selector, node_id, &needs)
				{
					self.matches.push(MatchOutput { node_id, span: child.span });
				}
			}
		}

		if total == 0 {
			self.check_empty_match();
		}
	}

	fn compute_type_indices(&self, children: &[SiblingInfo]) -> Vec<(i32, i32, usize)> {
		use std::collections::HashMap;

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

		for selector in self.selectors.selectors() {
			let needs = self.get_deferred_pseudo_needs(selector);
			if !needs.empty {
				continue;
			}
			if self.matches_deferred_selector(selector, node_id, &needs) {
				self.matches.push(MatchOutput { node_id, span: exiting.span });
			}
		}
	}

	fn get_deferred_pseudo_needs(&self, selector: &QueryCompoundSelector) -> DeferredNeeds {
		let mut needs = DeferredNeeds::default();
		for part in selector.parts() {
			match part {
				QuerySelectorComponent::PseudoClass(pseudo) => match pseudo {
					QueryPseudoClass::OnlyChild(_, _) => needs.only_child = true,
					QueryPseudoClass::LastChild(_, _) => needs.last_child = true,
					QueryPseudoClass::FirstOfType(_, _) => needs.first_of_type = true,
					QueryPseudoClass::LastOfType(_, _) => needs.last_of_type = true,
					QueryPseudoClass::OnlyOfType(_, _) => needs.only_of_type = true,
					QueryPseudoClass::Empty(_, _) => needs.empty = true,
					_ => {}
				},
				QuerySelectorComponent::FunctionalPseudoClass(fpc) => match fpc {
					QueryFunctionalPseudoClass::NthLastChild(p) => needs.nth_last_child = Some(p.value.clone()),
					QueryFunctionalPseudoClass::NthOfType(p) => needs.nth_of_type = Some(p.value.clone()),
					QueryFunctionalPseudoClass::NthLastOfType(p) => needs.nth_last_of_type = Some(p.value.clone()),
					_ => {}
				},
				_ => {}
			}
		}
		needs
	}

	#[allow(clippy::too_many_arguments)]
	fn child_matches_deferred(
		&self,
		needs: &DeferredNeeds,
		_node_id: NodeId,
		_child_index: i32,
		index_from_end: i32,
		total: usize,
		type_index: i32,
		type_index_from_end: i32,
		type_count: usize,
	) -> bool {
		!((needs.only_child && total != 1)
			|| (needs.last_child && index_from_end != 1)
			|| (needs.first_of_type && type_index != 1)
			|| (needs.last_of_type && type_index_from_end != 1)
			|| (needs.only_of_type && type_count != 1))
			&& needs.nth_last_child.as_ref().is_none_or(|p| p.matches(index_from_end))
			&& needs.nth_of_type.as_ref().is_none_or(|p| p.matches(type_index))
			&& needs.nth_last_of_type.as_ref().is_none_or(|p| p.matches(type_index_from_end))
	}

	fn matches_deferred_selector(
		&self,
		selector: &QueryCompoundSelector,
		node_id: NodeId,
		needs: &DeferredNeeds,
	) -> bool {
		let parts = selector.parts();
		if parts.is_empty() {
			return false;
		}

		// Find the rightmost type or wildcard
		let rightmost_type = self.get_rightmost_type(parts);
		if let Some(expected) = rightmost_type
			&& expected != node_id
		{
			return false;
		}

		let context = MatchContext { source: self.source, sibling_index: 1, ..Default::default() };

		// Check all pseudo-classes
		for part in parts {
			match part {
				QuerySelectorComponent::PseudoClass(pseudo) => {
					let skip = match pseudo {
						QueryPseudoClass::OnlyChild(_, _) => needs.only_child,
						QueryPseudoClass::LastChild(_, _) => needs.last_child,
						QueryPseudoClass::FirstOfType(_, _) => needs.first_of_type,
						QueryPseudoClass::LastOfType(_, _) => needs.last_of_type,
						QueryPseudoClass::OnlyOfType(_, _) => needs.only_of_type,
						QueryPseudoClass::Empty(_, _) => needs.empty,
						_ => false,
					};
					if skip {
						continue;
					}
					if !self.matches_pseudo_with_context(pseudo, node_id, &context) {
						return false;
					}
				}
				QuerySelectorComponent::FunctionalPseudoClass(fpc) => {
					let skip = match fpc {
						QueryFunctionalPseudoClass::NthLastChild(_) => needs.nth_last_child.is_some(),
						QueryFunctionalPseudoClass::NthOfType(_) => needs.nth_of_type.is_some(),
						QueryFunctionalPseudoClass::NthLastOfType(_) => needs.nth_last_of_type.is_some(),
						_ => false,
					};
					if skip {
						continue;
					}
					if !self.matches_functional_pseudo_with_context(fpc, node_id, &context) {
						return false;
					}
				}
				_ => {}
			}
		}
		// Only match if this is a simple selector (no combinators leading to ancestors)
		!self.has_ancestor_parts(parts)
	}

	fn get_rightmost_type(&self, parts: &[QuerySelectorComponent]) -> Option<NodeId> {
		for part in parts.iter().rev() {
			match part {
				QuerySelectorComponent::Type(t) => return Some(t.node_id(self.selector_source)),
				QuerySelectorComponent::Wildcard(_) => return None,
				QuerySelectorComponent::Combinator(_) => return None,
				_ => continue,
			}
		}
		None
	}

	fn has_ancestor_parts(&self, parts: &[QuerySelectorComponent]) -> bool {
		parts.iter().any(|p| matches!(p, QuerySelectorComponent::Combinator(_)))
	}

	fn check_match_with_context(&mut self, node_id: NodeId, span: Span, context: &MatchContext) {
		for selector in self.selectors.selectors() {
			if self.matches_selector_with_context(selector, node_id, context) {
				self.matches.push(MatchOutput { node_id, span });
			}
		}
	}

	fn check_declaration_match(&mut self, span: Span, context: &MatchContext) {
		for selector in self.selectors.selectors() {
			if self.matches_declaration_selector(selector, context) {
				self.matches.push(MatchOutput { node_id: NodeId::StyleRule, span });
			}
		}
	}

	fn matches_declaration_selector(&self, selector: &QueryCompoundSelector, context: &MatchContext) -> bool {
		let parts = selector.parts();
		// Declaration selectors should not have combinators
		if parts.iter().any(|p| matches!(p, QuerySelectorComponent::Combinator(_))) {
			return false;
		}
		// Should not have a type selector
		if parts.iter().any(|p| matches!(p, QuerySelectorComponent::Type(_))) {
			return false;
		}

		self.matches_declaration_parts(parts, context)
	}

	fn matches_attribute(&self, attr: &QueryAttribute, context: &MatchContext) -> bool {
		// Only support [name=...] attribute selector
		if attr.attr_name(self.selector_source) != "name" {
			return false;
		}
		let Some(cursor) = context.property_name else {
			return false;
		};
		let expected_name = attr.attr_value(self.selector_source);
		let expected_atom = CssAtomSet::from_str(expected_name);
		if expected_atom != CssAtomSet::_None {
			return CssAtomSet::from_bits(cursor.atom_bits()) == expected_atom;
		}
		cursor.str_slice(context.source).eq_ignore_ascii_case(expected_name)
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

		// Split at the last combinator to get the rightmost simple selector
		let (ancestor_parts, rightmost_parts) = self.split_at_last_combinator(parts);

		// Check if current node matches the rightmost simple selector
		if !self.matches_simple_parts(rightmost_parts, node_id, context) {
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
				return Some(t.node_id(self.selector_source));
			}
		}
		None
	}

	fn matches_parent_entry_parts(&self, parts: &[QuerySelectorComponent<'b>], parent: &ParentEntry) -> bool {
		match parent.node_id {
			Some(node_id) => self.matches_simple_parts(parts, node_id, &parent.context),
			None => self.matches_declaration_parts(parts, &parent.context),
		}
	}

	fn matches_declaration_parts(&self, parts: &[QuerySelectorComponent<'b>], context: &MatchContext) -> bool {
		// Should not have a type selector
		if parts.iter().any(|p| matches!(p, QuerySelectorComponent::Type(_))) {
			return false;
		}

		let mut has_meaningful_selector = false;

		for part in parts {
			match part {
				QuerySelectorComponent::Attribute(attr) => {
					has_meaningful_selector = true;
					if !self.matches_attribute(attr, context) {
						return false;
					}
				}
				QuerySelectorComponent::PseudoClass(pseudo) => {
					let (is_decl_pseudo, matches) = self.check_declaration_pseudo(pseudo, context);
					if is_decl_pseudo {
						has_meaningful_selector = true;
						if !matches {
							return false;
						}
					}
				}
				QuerySelectorComponent::FunctionalPseudoClass(fpc) => {
					let (is_decl_pseudo, matches) = self.check_declaration_functional_pseudo(fpc, context);
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

	fn check_declaration_pseudo(&self, pseudo: &QueryPseudoClass, context: &MatchContext) -> (bool, bool) {
		match pseudo {
			QueryPseudoClass::Important(_, _) => (true, context.is_important),
			QueryPseudoClass::Custom(_, _) => (true, context.is_custom_property),
			QueryPseudoClass::Computed(_, _) => (true, context.is_computed),
			QueryPseudoClass::Shorthand(_, _) => (true, context.is_shorthand),
			QueryPseudoClass::Longhand(_, _) => (true, context.is_longhand),
			QueryPseudoClass::Unknown(_, _) => (true, context.is_unknown),
			QueryPseudoClass::Prefixed(_, _) => (true, context.vendor_prefix.is_some()),
			_ => (false, true),
		}
	}

	fn check_declaration_functional_pseudo(
		&self,
		fpc: &QueryFunctionalPseudoClass,
		context: &MatchContext,
	) -> (bool, bool) {
		match fpc {
			QueryFunctionalPseudoClass::Prefixed(p) => {
				let cursor: Cursor = p.vendor.into();
				let vendor_atom = CsskitAtomSet::from_bits(cursor.atom_bits());
				(true, context.vendor_prefix.is_some_and(|prefix| prefix == vendor_atom))
			}
			QueryFunctionalPseudoClass::PropertyType(p) => {
				let cursor: Cursor = p.group.into();
				let group_atom = CsskitAtomSet::from_bits(cursor.atom_bits());
				(true, self.matches_property_type(context, group_atom))
			}
			_ => (false, true),
		}
	}

	fn matches_simple_parts(
		&self,
		parts: &[QuerySelectorComponent<'b>],
		node_id: NodeId,
		context: &MatchContext,
	) -> bool {
		for part in parts {
			match part {
				QuerySelectorComponent::Type(t) => {
					if t.node_id(self.selector_source) != node_id {
						return false;
					}
				}
				QuerySelectorComponent::Wildcard(_) => {}
				QuerySelectorComponent::Attribute(attr) => {
					// Attribute selectors on non-declaration context are invalid
					if context.property_name.is_none() {
						return false;
					}
					if !self.matches_attribute(attr, context) {
						return false;
					}
				}
				QuerySelectorComponent::PseudoClass(pseudo) => {
					if !self.matches_pseudo_with_context(pseudo, node_id, context) {
						return false;
					}
				}
				QuerySelectorComponent::FunctionalPseudoClass(fpc) => {
					if !self.matches_functional_pseudo_with_context(fpc, node_id, context) {
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
		match pseudo {
			QueryPseudoClass::AtRule(_, _) => self.is_at_rule(node_id),
			QueryPseudoClass::Computed(_, _) => context.is_computed,
			QueryPseudoClass::Custom(_, _) => context.is_custom_property,
			QueryPseudoClass::FirstChild(_, _) => context.sibling_index == 1,
			QueryPseudoClass::Function(_, _) => self.is_function(node_id),
			QueryPseudoClass::Important(_, _) => context.is_important,
			QueryPseudoClass::Longhand(_, _) => context.is_longhand,
			QueryPseudoClass::Nested(_, _) => self.parent_stack.iter().any(|p| p.node_id == Some(NodeId::StyleRule)),
			QueryPseudoClass::Prefixed(_, _) => self.is_prefixed(node_id, context, None),
			QueryPseudoClass::Root(_, _) => self.parent_stack.is_empty(),
			QueryPseudoClass::Rule(_, _) => self.is_rule(node_id),
			QueryPseudoClass::Shorthand(_, _) => context.is_shorthand,
			QueryPseudoClass::Unknown(_, _) => context.is_unknown || node_id.tag_name().contains("unknown"),
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
		fpc: &QueryFunctionalPseudoClass,
		node_id: NodeId,
		context: &MatchContext,
	) -> bool {
		match fpc {
			QueryFunctionalPseudoClass::Not(not_pseudo) => {
				// Check if the inner selector matches
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
				let group_atom = CsskitAtomSet::from_bits(cursor.atom_bits());
				self.matches_property_type(context, group_atom)
			}
			QueryFunctionalPseudoClass::Prefixed(p) => {
				let cursor: Cursor = p.vendor.into();
				let vendor_atom = CsskitAtomSet::from_bits(cursor.atom_bits());
				self.is_prefixed(node_id, context, Some(vendor_atom))
			}
		}
	}

	fn is_at_rule(&self, node_id: NodeId) -> bool {
		let name = node_id.tag_name();
		name.ends_with("-rule") && name != "style-rule"
	}

	fn is_rule(&self, node_id: NodeId) -> bool {
		node_id.tag_name().ends_with("-rule")
	}

	fn is_function(&self, node_id: NodeId) -> bool {
		let name = node_id.tag_name();
		name.ends_with("-function") || name.ends_with("-pseudo-function")
	}

	fn is_prefixed(&self, node_id: NodeId, context: &MatchContext, filter: Option<CsskitAtomSet>) -> bool {
		if let Some(prefix) = parse_vendor_prefix(node_id.tag_name()) {
			return filter.is_none_or(|f| prefix == f);
		}
		if let Some(prefix) = context.vendor_prefix {
			return filter.is_none_or(|f| prefix == f);
		}
		false
	}

	fn matches_property_type(&self, context: &MatchContext, group: crate::CsskitAtomSet) -> bool {
		use crate::CsskitAtomSet::*;
		let Some(property_group) = context.property_group else {
			return false;
		};
		match group {
			Align => property_group.contains(PropertyGroup::Align),
			Anchor | AnchorPosition => property_group.contains(PropertyGroup::AnchorPosition),
			Animation | Animations => property_group.contains(PropertyGroup::Animations),
			Background | Backgrounds => property_group.contains(PropertyGroup::Backgrounds),
			Border | Borders => property_group.contains(PropertyGroup::Borders),
			Box => property_group.contains(PropertyGroup::Box),
			Break => property_group.contains(PropertyGroup::Break),
			Cascade => property_group.contains(PropertyGroup::Cascade),
			Color => property_group.contains(PropertyGroup::Color),
			ColorAdjust => property_group.contains(PropertyGroup::ColorAdjust),
			ColorHdr => property_group.contains(PropertyGroup::ColorHdr),
			Conditional => property_group.contains(PropertyGroup::Conditional),
			Contain => property_group.contains(PropertyGroup::Contain),
			Content => property_group.contains(PropertyGroup::Content),
			Display => property_group.contains(PropertyGroup::Display),
			Exclusions => property_group.contains(PropertyGroup::Exclusions),
			Flex | Flexbox => property_group.contains(PropertyGroup::Flexbox),
			Font | Fonts => property_group.contains(PropertyGroup::Fonts),
			Forms => property_group.contains(PropertyGroup::Forms),
			Gap | Gaps => property_group.contains(PropertyGroup::Gaps),
			Gcpm => property_group.contains(PropertyGroup::Gcpm),
			Grid => property_group.contains(PropertyGroup::Grid),
			Image | Images => property_group.contains(PropertyGroup::Images),
			Inline => property_group.contains(PropertyGroup::Inline),
			LineGrid => property_group.contains(PropertyGroup::LineGrid),
			LinkParams => property_group.contains(PropertyGroup::LinkParams),
			List | Lists => property_group.contains(PropertyGroup::Lists),
			Logical => property_group.contains(PropertyGroup::Logical),
			Mask | Masking => property_group.contains(PropertyGroup::Masking),
			Multicol => property_group.contains(PropertyGroup::Multicol),
			Nav => property_group.contains(PropertyGroup::Nav),
			Overflow => property_group.contains(PropertyGroup::Overflow),
			Overscroll => property_group.contains(PropertyGroup::Overscroll),
			Page => property_group.contains(PropertyGroup::Page),
			PageFloats => property_group.contains(PropertyGroup::PageFloats),
			Position => property_group.contains(PropertyGroup::Position),
			Regions => property_group.contains(PropertyGroup::Regions),
			Rhythm => property_group.contains(PropertyGroup::Rhythm),
			RoundDisplay => property_group.contains(PropertyGroup::RoundDisplay),
			Ruby => property_group.contains(PropertyGroup::Ruby),
			ScrollAnchoring => property_group.contains(PropertyGroup::ScrollAnchoring),
			ScrollSnap => property_group.contains(PropertyGroup::ScrollSnap),
			Scrollbar | Scrollbars => property_group.contains(PropertyGroup::Scrollbars),
			Shaders => property_group.contains(PropertyGroup::Shaders),
			Shape | Shapes => property_group.contains(PropertyGroup::Shapes),
			SizeAdjust => property_group.contains(PropertyGroup::SizeAdjust),
			Sizing => property_group.contains(PropertyGroup::Sizing),
			Speech => property_group.contains(PropertyGroup::Speech),
			Table | Tables => property_group.contains(PropertyGroup::Tables),
			Text => property_group.contains(PropertyGroup::Text),
			TextDecor | TextDecoration => property_group.contains(PropertyGroup::TextDecor),
			Transform | Transforms => property_group.contains(PropertyGroup::Transforms),
			Transition | Transitions => property_group.contains(PropertyGroup::Transitions),
			Ui => property_group.contains(PropertyGroup::Ui),
			Values => property_group.contains(PropertyGroup::Values),
			Variables => property_group.contains(PropertyGroup::Variables),
			ViewTransitions => property_group.contains(PropertyGroup::ViewTransitions),
			Viewport => property_group.contains(PropertyGroup::Viewport),
			WillChange => property_group.contains(PropertyGroup::WillChange),
			WritingModes => property_group.contains(PropertyGroup::WritingModes),
			_ => false,
		}
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

		// Determine vendor prefix from property name
		let property_cursor: Cursor = node.name.into();
		let property_name_str = property_cursor.str_slice(self.source);
		let vendor_prefix = parse_vendor_prefix(property_name_str);

		// Get metadata for computed/property-type checks
		let metadata = node.metadata();
		let declaration_kinds = metadata.declaration_kinds;

		// Check shorthand/longhand using the property name directly
		let property_atom = CssAtomSet::from_bits(property_cursor.atom_bits());
		let is_shorthand = StyleValue::is_shorthand_by_name(property_atom);
		// A property is longhand if it's a known property that's not a shorthand
		let is_longhand = property_atom != CssAtomSet::_None && !is_shorthand;

		// Build context for pseudo-class matching
		let context = MatchContext {
			is_important: node.important.is_some(),
			is_custom_property: node.name.is_dashed_ident(),
			is_computed: declaration_kinds.contains(DeclarationKind::Computed),
			is_shorthand,
			is_longhand,
			is_unknown: declaration_kinds.contains(DeclarationKind::Unknown),
			property_group: if metadata.property_groups.is_none() { None } else { Some(metadata.property_groups) },
			property_name: Some(node.name.into()),
			source: self.source,
			sibling_index,
			vendor_prefix,
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
	fn prefixed_node_webkit_keyframes() {
		// @-webkit-keyframes is a prefixed node type
		assert_query!("@-webkit-keyframes spin { to { opacity: 1; } }", "webkit-keyframes-rule:prefixed", 1);
	}

	#[test]
	fn prefixed_node_filter() {
		// webkit-keyframes-rule should match :prefixed(webkit)
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
}
