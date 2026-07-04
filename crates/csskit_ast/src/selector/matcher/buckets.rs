use super::NodeData;
use crate::{QueryCompoundSelector, SelectorRequirements, SelectorStructure};
use css_ast::{AtRuleId, CssMetadata, NodeKinds, PropertyKind, visit::NodeId};
use std::collections::HashMap;

/// Type for requirement check functions used in bucketing.
type RequirementCheck = (SelectorRequirements, fn(&CssMetadata) -> bool);

/// Selectors are bucketed by their rightmost component characteristics.
pub(crate) struct SelectorBuckets<'a, 'b> {
	/// By rightmost type (e.g., `style-rule`).
	by_type: HashMap<NodeId, Vec<&'a QueryCompoundSelector<'b>>>,
	/// By attribute name (e.g., `[name=color]`).
	by_attribute: HashMap<PropertyKind, Vec<&'a QueryCompoundSelector<'b>>>,
	/// By pseudo requirement (e.g., `:important`).
	by_pseudo: HashMap<SelectorRequirements, Vec<&'a QueryCompoundSelector<'b>>>,
	/// Catch-all bucket for wildcards and complex selectors.
	other: Vec<&'a QueryCompoundSelector<'b>>,
	/// Aggregated requirements across all selectors for quick filtering.
	all_requirements: SelectorRequirements,
	/// True if any type-bucketed selector targets `StyleRule` nodes; checked by `subtree_can_match`
	/// against `CssMetadata::node_kinds`.
	wants_style_rule: bool,
	/// Union of `AtRuleId`s that type-bucketed selectors target; checked by `subtree_can_match`
	/// against `CssMetadata::used_at_rules`.
	wanted_at_rules: AtRuleId,
	/// Union of attribute property kinds any selector targets; checked by `subtree_can_match`
	/// against `CssMetadata::property_kinds`.
	wanted_attributes: PropertyKind,
	/// True when subtree pruning is unsafe (a selector has a combinator, pseudo-class, or
	/// functional pseudo-class - ancestors/siblings/inner targets must stay reachable) or
	/// unnecessary (a selector has no closed-domain aggregate to prune against - declarations,
	/// functions, value types, or the wildcard/complex catch-all bucket). When set,
	/// `subtree_can_match` always returns `true`.
	has_unconstrained: bool,
}

impl<'a, 'b> SelectorBuckets<'a, 'b> {
	pub(crate) fn new(selectors: &[&'a QueryCompoundSelector<'b>]) -> Self {
		let mut buckets = Self {
			by_type: HashMap::new(),
			by_attribute: HashMap::new(),
			by_pseudo: HashMap::new(),
			other: Vec::new(),
			all_requirements: SelectorRequirements::none(),
			wants_style_rule: false,
			wanted_at_rules: AtRuleId::none(),
			wanted_attributes: PropertyKind::none(),
			has_unconstrained: false,
		};

		for &selector in selectors {
			buckets.add_selector(selector);
		}

		buckets
	}

	fn add_selector(&mut self, selector: &'a QueryCompoundSelector<'b>) {
		let meta = selector.metadata();
		self.all_requirements |= meta.self_requirements;

		let unsafe_structure =
			SelectorStructure::HasCombinator | SelectorStructure::HasPseudo | SelectorStructure::HasFunctionalPseudo;
		if meta.structure.intersects(unsafe_structure) {
			self.has_unconstrained = true;
		}

		if let Some(type_id) = meta.rightmost_type_id {
			self.by_type.entry(type_id).or_default().push(selector);
			if type_id == NodeId::StyleRule {
				self.wants_style_rule = true;
			} else if let Some(at_rule_id) = type_id.to_at_rule_id() {
				self.wanted_at_rules |= at_rule_id;
			} else {
				self.has_unconstrained = true;
			}
			return;
		}

		// Use self_attribute_filter for bucketing - excludes :has() inner requirements
		if !meta.self_attribute_filter.is_none() {
			self.by_attribute.entry(meta.self_attribute_filter).or_default().push(selector);
			self.wanted_attributes |= meta.self_attribute_filter;
			return;
		}

		// Use self_requirements for bucketing - excludes :has() inner requirements
		if !meta.self_requirements.is_none() && !meta.self_requirements.contains(SelectorRequirements::Prefixed) {
			// Skip :prefixed, :property-type, :unknown, :empty which can go into "other"
			const BUCKETED_REQUIREMENTS: [SelectorRequirements; 7] = [
				SelectorRequirements::Important,
				SelectorRequirements::Custom,
				SelectorRequirements::Shorthand,
				SelectorRequirements::Longhand,
				SelectorRequirements::Computed,
				SelectorRequirements::Rule,
				SelectorRequirements::AtRule,
			];
			for req in BUCKETED_REQUIREMENTS {
				if meta.self_requirements.contains(req) {
					self.by_pseudo.entry(req).or_default().push(selector);
					return;
				}
			}
		}
		self.other.push(selector);
		self.has_unconstrained = true;
	}

	/// Returns an iterator over selectors that might match the given node.
	pub(crate) fn selectors_for_node(&self, node: &NodeData) -> impl Iterator<Item = &&'a QueryCompoundSelector<'b>> {
		let type_selectors = self.by_type.get(&node.node_id).into_iter().flat_map(|v| v.iter());
		let attr_selectors = self
			.by_attribute
			.iter()
			.filter(move |&(&kind, _)| node.metadata.property_kinds.contains(kind))
			.flat_map(|(_, v)| v.iter());
		let pseudo_selectors = self.pseudo_selectors_for_node(node);
		type_selectors.chain(attr_selectors).chain(pseudo_selectors).chain(self.other.iter())
	}

	/// Returns selectors from pseudo buckets that match the node's metadata.
	fn pseudo_selectors_for_node(&self, node: &NodeData) -> impl Iterator<Item = &&'a QueryCompoundSelector<'b>> {
		const REQUIREMENTS: [RequirementCheck; 7] = [
			(SelectorRequirements::Important, CssMetadata::has_important),
			(SelectorRequirements::Custom, CssMetadata::has_custom_properties),
			(SelectorRequirements::Computed, CssMetadata::has_computed),
			(SelectorRequirements::Shorthand, CssMetadata::has_shorthands),
			(SelectorRequirements::Longhand, CssMetadata::has_longhands),
			(SelectorRequirements::Rule, CssMetadata::has_rules),
			(SelectorRequirements::AtRule, CssMetadata::has_at_rules),
		];

		REQUIREMENTS
			.iter()
			.filter(move |(req, check_fn)| check_fn(&node.metadata) && self.all_requirements.contains(*req))
			.filter_map(move |(req, _)| self.by_pseudo.get(req))
			.flat_map(|v| v.iter())
	}

	/// Returns `false` if the subtree rooted at a node with `subtree_meta` provably
	/// contains no node that could match any live selector's rightmost component.
	///
	/// `subtree_meta` must be the aggregated metadata for the node (self + descendants).
	#[inline]
	pub(crate) fn subtree_can_match(&self, subtree_meta: &CssMetadata) -> bool {
		if self.has_unconstrained {
			return true;
		}
		if self.wants_style_rule && subtree_meta.node_kinds.contains(NodeKinds::StyleRule) {
			return true;
		}
		if !self.wanted_at_rules.is_none() && subtree_meta.used_at_rules.intersects(self.wanted_at_rules) {
			return true;
		}
		if !self.wanted_attributes.is_none() && subtree_meta.property_kinds.intersects(self.wanted_attributes) {
			return true;
		}
		false
	}
}
