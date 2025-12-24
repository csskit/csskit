use bitmask_enum::bitmask;
use css_ast::{AtRuleId, CssMetadata, PropertyGroup, PropertyKind, VendorPrefixes, visit::NodeId};
use css_parse::NodeMetadata;

/// Metadata about a query selector, computed at parse time.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QuerySelectorMetadata {
	/// What CSS features this selector requires to possibly match.
	pub requirements: SelectorRequirements,
	/// Structural flags about the selector.
	pub structure: SelectorStructure,
	/// Pre-computed NodeId of the rightmost type selector (if any).
	/// None if the rightmost simple selector is a wildcard or has no type.
	/// This enables fast type checking without iterating through parts.
	pub rightmost_type_id: Option<NodeId>,
	/// Pre-computed at-rule filter from type selectors (e.g., `media-rule`).
	/// If set, the CSS must contain at least one of these at-rule types to match.
	pub at_rule_filter: AtRuleId,
	/// Pre-computed property groups from :property-type() pseudo-classes.
	pub property_groups: PropertyGroup,
	/// Pre-computed vendor filter from :prefixed() pseudo-class.
	pub vendor_filter: VendorPrefixes,
	/// Pre-computed attribute filter from attribute selectors (e.g., `[name=...]`).
	/// Maps to PropertyKind flags that nodes must have to match.
	pub attribute_filter: PropertyKind,
	/// True if selector has pseudo-classes requiring deferred matching (sibling/child info).
	pub deferred: bool,
	/// True if deferred matching needs type tracking (for :first-of-type, :nth-of-type, etc.).
	pub needs_type_tracking: bool,
	/// For :not(type), the excluded type. None if :not() doesn't contain a simple type.
	pub not_type: Option<NodeId>,
	/// True if selector is just a single type (e.g., "style-rule") with no other components.
	pub is_type_only: bool,
}

impl Default for QuerySelectorMetadata {
	fn default() -> Self {
		Self {
			requirements: SelectorRequirements::none(),
			structure: SelectorStructure::none(),
			rightmost_type_id: None,
			at_rule_filter: AtRuleId::none(),
			property_groups: PropertyGroup::none(),
			vendor_filter: VendorPrefixes::none(),
			attribute_filter: PropertyKind::none(),
			deferred: false,
			needs_type_tracking: false,
			not_type: None,
			is_type_only: false,
		}
	}
}

impl NodeMetadata for QuerySelectorMetadata {
	/// Merge two metadata instances. Used when aggregating metadata from selector components.
	/// Bitmask fields are merged with OR. `rightmost_type_id` uses the other's value if set
	/// (since we merge left-to-right, later values represent the rightmost component).
	fn merge(mut self, other: Self) -> Self {
		self.requirements |= other.requirements;
		self.structure |= other.structure;
		self.at_rule_filter |= other.at_rule_filter;
		self.property_groups |= other.property_groups;
		self.vendor_filter |= other.vendor_filter;
		self.attribute_filter |= other.attribute_filter;
		self.deferred |= other.deferred;
		self.needs_type_tracking |= other.needs_type_tracking;
		if other.rightmost_type_id.is_some() {
			self.rightmost_type_id = other.rightmost_type_id;
		}
		if other.not_type.is_some() {
			self.not_type = other.not_type;
		}
		self
	}
}

impl QuerySelectorMetadata {
	/// Returns true if the selector can possibly match given the CSS metadata.
	/// This enables early filtering of selectors that can't match.
	#[inline]
	pub fn can_match(&self, css_meta: &CssMetadata) -> bool {
		self.requirements.can_match(css_meta)
			&& (self.at_rule_filter.is_none() || css_meta.used_at_rules.intersects(self.at_rule_filter))
			&& (self.attribute_filter.is_none() || css_meta.property_kinds.contains(self.attribute_filter))
			&& (self.property_groups.is_none() || css_meta.property_groups.intersects(self.property_groups))
			&& (self.vendor_filter.is_none() || css_meta.vendor_prefixes.intersects(self.vendor_filter))
	}

	/// Fast type rejection: returns true if the node_id definitely cannot match.
	#[inline]
	pub fn rejects_type(&self, node_id: NodeId) -> bool {
		self.rightmost_type_id.is_some_and(|t| t != node_id) || self.not_type.is_some_and(|t| t == node_id)
	}
}

/// Requirements a selector has that can be checked against CssMetadata.
/// If any required flag is set but the CSS metadata doesn't have it, the selector can't match.
#[bitmask(u16)]
#[bitmask_config(vec_debug)]
#[derive(Default)]
pub enum SelectorRequirements {
	Important,
	Custom,
	Computed,
	Shorthand,
	Longhand,
	Unknown,
	Prefixed,
	Rule,
	AtRule,
	Empty,
}

impl SelectorRequirements {
	/// Returns true if the selector can possibly match given the CSS metadata.
	#[inline]
	pub fn can_match(self, meta: &CssMetadata) -> bool {
		(!self.contains(Self::Important) || meta.has_important())
			&& (!self.contains(Self::Custom) || meta.has_custom_properties())
			&& (!self.contains(Self::Computed) || meta.has_computed())
			&& (!self.contains(Self::Shorthand) || meta.has_shorthands())
			&& (!self.contains(Self::Longhand) || meta.has_longhands())
			&& (!self.contains(Self::Unknown) || meta.has_unknown())
			&& (!self.contains(Self::Prefixed) || meta.has_vendor_prefixes())
			&& (!self.contains(Self::Rule) || meta.has_rules())
			&& (!self.contains(Self::AtRule) || meta.has_at_rules())
			&& (!self.contains(Self::Empty) || meta.is_empty_container())
	}
}

/// Structural information about the selector.
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[derive(Default)]
pub enum SelectorStructure {
	/// Selector contains an attribute selector (e.g., `[name=color]`)
	HasAttribute,
	/// Selector contains a combinator (e.g., `>`, `+`, `~`, or descendant)
	HasCombinator,
	/// Selector contains a pseudo-class (e.g., `:important`)
	HasPseudo,
	/// Selector contains a functional pseudo-class (e.g., `:not()`)
	HasFunctionalPseudo,
}
