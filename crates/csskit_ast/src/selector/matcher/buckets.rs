use super::NodeData;
use crate::{QueryCompoundSelector, SelectorRequirements, SelectorStructure};
use css_ast::{CssMetadata, PropertyKind, visit::NodeId};
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
	/// Aggregated structure flags.
	all_structure: SelectorStructure,
}

impl<'a, 'b> SelectorBuckets<'a, 'b> {
	pub(crate) fn new(selectors: &[&'a QueryCompoundSelector<'b>]) -> Self {
		let mut buckets = Self {
			by_type: HashMap::new(),
			by_attribute: HashMap::new(),
			by_pseudo: HashMap::new(),
			other: Vec::new(),
			all_requirements: SelectorRequirements::none(),
			all_structure: SelectorStructure::none(),
		};

		for &selector in selectors {
			buckets.add_selector(selector);
		}

		buckets
	}

	fn add_selector(&mut self, selector: &'a QueryCompoundSelector<'b>) {
		let meta = selector.metadata();
		self.all_requirements |= meta.requirements;
		self.all_structure |= meta.structure;

		if let Some(type_id) = meta.rightmost_type_id {
			self.by_type.entry(type_id).or_default().push(selector);
			return;
		}

		if !meta.attribute_filter.is_none() {
			self.by_attribute.entry(meta.attribute_filter).or_default().push(selector);
			return;
		}

		if !meta.requirements.is_none() && !meta.requirements.contains(SelectorRequirements::Prefixed) {
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
				if meta.requirements.contains(req) {
					self.by_pseudo.entry(req).or_default().push(selector);
					return;
				}
			}
		}
		self.other.push(selector);
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
}
