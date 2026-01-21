#[cfg(feature = "visitable")]
use crate::visit::NodeId;
use crate::{
	CssAtomSet,
	traits::{AppliesTo, BoxPortion, BoxSide, PropertyGroup},
};
use bitmask_enum::bitmask;
use css_lexer::{Span, ToSpan};
use css_parse::{NodeMetadata, SemanticEq, ToCursors};

/// How unitless zero (0 without a unit) resolves in a given context.
///
/// For most Style Values, a `0` can be a drop-in replacement for `0px`, but
/// certain style values will provide discrete syntax for `0px` and `0`, meaning
/// they resolve to different things. For properties that accept both `<number>`
/// and `<length>`, unitless zero may resolve to a _different value_. Using a
/// piece of metadata to describe this can be helpful for linting/minifying -
/// avoiding a reduction in semantic meaning.
///
/// Examples:
/// - `width: 0px` == `width: 0` (unitless zero resolves to length)
/// - `line-height: 0px` != `line-height: 0` (unitless zero resolves to number = 0x multiplier)
/// - `tab-size: 0px` != `tab-size: 0` (unitless zero resolves to number = 0 tab characters)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum UnitlessZeroResolves {
	/// Unitless zero resolves to a length (0 = 0px).
	#[default]
	Length,
	/// Unitless zero resolves to a number or percentage. NOT safe to reduce.
	Number,
}

#[bitmask(u32)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AtRuleId {
	Charset,
	ColorProfile,
	Container,
	CounterStyle,
	FontFace,
	FontFeatureValues,
	FontPaletteValues,
	Import,
	Keyframes,
	Layer,
	Media,
	Namespace,
	Page,
	Property,
	Scope,
	StartingStyle,
	Supports,
	Document,
	WebkitKeyframes,
	MozDocument,
}

#[cfg(feature = "visitable")]
impl NodeId {
	/// Converts a NodeId to an AtRuleId if the node is an at-rule type.
	/// Returns `None` for non-at-rule nodes like StyleRule, Declaration, etc.
	pub fn to_at_rule_id(self) -> Option<AtRuleId> {
		match self {
			Self::CharsetRule => Some(AtRuleId::Charset),
			Self::ContainerRule => Some(AtRuleId::Container),
			Self::CounterStyleRule => Some(AtRuleId::CounterStyle),
			Self::DocumentRule => Some(AtRuleId::Document),
			Self::FontFaceRule => Some(AtRuleId::FontFace),
			Self::KeyframesRule => Some(AtRuleId::Keyframes),
			Self::LayerRule => Some(AtRuleId::Layer),
			Self::MediaRule => Some(AtRuleId::Media),
			Self::MozDocumentRule => Some(AtRuleId::MozDocument),
			Self::NamespaceRule => Some(AtRuleId::Namespace),
			Self::PageRule => Some(AtRuleId::Page),
			Self::PropertyRule => Some(AtRuleId::Property),
			Self::StartingStyleRule => Some(AtRuleId::StartingStyle),
			Self::SupportsRule => Some(AtRuleId::Supports),
			Self::WebkitKeyframesRule => Some(AtRuleId::WebkitKeyframes),
			_ => None,
		}
	}
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum VendorPrefixes {
	Moz,
	WebKit,
	O,
	Ms,
}

impl TryFrom<CssAtomSet> for VendorPrefixes {
	type Error = ();
	fn try_from(atom: CssAtomSet) -> Result<Self, Self::Error> {
		const VENDOR_FLAG: u32 = 0b00000000_10000000_00000000_00000000;
		const VENDORS: [VendorPrefixes; 4] =
			[VendorPrefixes::WebKit, VendorPrefixes::Moz, VendorPrefixes::Ms, VendorPrefixes::O];

		let atom_bits = atom as u32;
		if atom_bits & VENDOR_FLAG == 0 {
			return Err(());
		}
		let index = (atom_bits >> 21) & 0b11;
		Ok(VENDORS[index as usize])
	}
}

#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum DeclarationKind {
	/// If a declaration has !important
	Important,
	/// If a declaration used a css-wide keyword, e.g. `inherit` or `revert-layer`.
	CssWideKeywords,
	/// If a declaration is custom, e.g `--foo`
	Custom,
	/// If a declaration is computed-time, e.g. using `calc()` or `var()`
	Computed,
	/// If a declaration is shorthand
	Shorthands,
	/// If a declaration is longhand
	Longhands,
}

/// Categories of nodes present in metadata, used for selector filtering.
#[bitmask(u16)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeKinds {
	/// Contains unknown nodes
	Unknown,
	/// Contains style rules
	StyleRule,
	/// Contains at-rules (media, keyframes, etc.)
	AtRule,
	/// Contains Declarations
	Declaration,
	/// Contains function nodes
	Function,
	/// Node has an empty prelude
	EmptyPrelude,
	/// Node has an empty block (no declarations, no nested rules)
	EmptyBlock,
	/// Node is nested within another node
	Nested,
	/// Node is deprecated (non-conforming, obsolete)
	Deprecated,
	/// Node is experimental (not yet standardized)
	Experimental,
	/// Node is non-standard (vendor-specific, not in spec)
	NonStandard,
	/// Node is a dimension value (length, angle, time, flex, etc.)
	Dimension,
	/// Node is a custom element or custom property
	Custom,
}

/// Queryable properties a node exposes for selector matching.
/// Used by attribute selectors like `[name]` or `[name=value]`.
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PropertyKind {
	/// Node has a queryable `name` property (declarations, named at-rules, functions)
	Name,
}

/// All PropertyKind variants for iteration.
pub const PROPERTY_KIND_VARIANTS: &[PropertyKind] = &[PropertyKind::Name];

/// Aggregated metadata computed from declarations within a block.
/// This allows efficient checking of what types of properties a block contains
/// without iterating through all declarations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CssMetadata {
	/// Bitwise OR of all PropertyGroup values
	pub property_groups: PropertyGroup,
	/// Bitwise OR of all AppliesTo values
	pub applies_to: AppliesTo,
	/// Bitwise OR of all BoxSide values
	pub box_sides: BoxSide,
	/// Bitwise OR of all BoxPortion values
	pub box_portions: BoxPortion,
	/// Bitwise OR of all DeclarationKind values
	pub declaration_kinds: DeclarationKind,
	/// Bitwise OR of all AtRuleIds in a Node
	pub used_at_rules: AtRuleId,
	/// Bitwise OR of all VendorPrefixes in a Node
	pub vendor_prefixes: VendorPrefixes,
	/// Bitwise OR of node categories present
	pub node_kinds: NodeKinds,
	/// Bitwise OR of queryable properties present
	pub property_kinds: PropertyKind,
	/// How unitless zero resolves in this context (Length or Number)
	pub unitless_zero_resolves: UnitlessZeroResolves,
	/// Size of vector-based nodes (e.g., number of declarations, selector list length)
	pub size: u16,
}

impl Default for CssMetadata {
	fn default() -> Self {
		Self {
			property_groups: PropertyGroup::none(),
			applies_to: AppliesTo::none(),
			box_sides: BoxSide::none(),
			box_portions: BoxPortion::none(),
			declaration_kinds: DeclarationKind::none(),
			used_at_rules: AtRuleId::none(),
			vendor_prefixes: VendorPrefixes::none(),
			node_kinds: NodeKinds::none(),
			property_kinds: PropertyKind::none(),
			unitless_zero_resolves: UnitlessZeroResolves::default(),
			size: 0,
		}
	}
}

impl CssMetadata {
	/// Returns true if this metadata is empty (contains no properties or at-rules)
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.property_groups == PropertyGroup::none()
			&& self.applies_to == AppliesTo::none()
			&& self.box_sides == BoxSide::none()
			&& self.box_portions == BoxPortion::none()
			&& self.declaration_kinds == DeclarationKind::none()
			&& self.used_at_rules == AtRuleId::none()
			&& self.vendor_prefixes == VendorPrefixes::none()
			&& self.node_kinds == NodeKinds::none()
			&& self.property_kinds == PropertyKind::none()
			&& self.unitless_zero_resolves == UnitlessZeroResolves::Length
			&& self.size == 0
	}

	/// Returns true if this block modifies any positioning-related properties.
	#[inline]
	pub fn modifies_box(&self) -> bool {
		!self.box_portions.is_none()
	}

	/// Returns true if metadata contains important declarations.
	#[inline]
	pub fn has_important(&self) -> bool {
		self.declaration_kinds.contains(DeclarationKind::Important)
	}

	/// Returns true if metadata contains custom properties.
	#[inline]
	pub fn has_custom_properties(&self) -> bool {
		self.declaration_kinds.contains(DeclarationKind::Custom)
	}

	/// Returns true if metadata contains computed values.
	#[inline]
	pub fn has_computed(&self) -> bool {
		self.declaration_kinds.contains(DeclarationKind::Computed)
	}

	/// Returns true if metadata contains shorthand properties.
	#[inline]
	pub fn has_shorthands(&self) -> bool {
		self.declaration_kinds.contains(DeclarationKind::Shorthands)
	}

	/// Returns true if metadata contains longhand properties.
	#[inline]
	pub fn has_longhands(&self) -> bool {
		self.declaration_kinds.contains(DeclarationKind::Longhands)
	}

	/// Returns true if metadata contains unknown nodes.
	#[inline]
	pub fn has_unknown(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Unknown)
	}

	/// Returns true if metadata contains vendor-prefixed properties.
	#[inline]
	pub fn has_vendor_prefixes(&self) -> bool {
		!self.vendor_prefixes.is_none()
	}

	/// Returns the vendor prefix if exactly one is present, None otherwise.
	#[inline]
	pub fn single_vendor_prefix(&self) -> Option<VendorPrefixes> {
		if self.vendor_prefixes.is_none() || self.vendor_prefixes.bits().count_ones() != 1 {
			None
		} else {
			Some(self.vendor_prefixes)
		}
	}

	/// Returns true if metadata contains any rule nodes.
	#[inline]
	pub fn has_rules(&self) -> bool {
		self.node_kinds.intersects(NodeKinds::StyleRule | NodeKinds::AtRule)
	}

	/// Returns true if metadata contains style rules.
	#[inline]
	pub fn has_style_rules(&self) -> bool {
		self.node_kinds.contains(NodeKinds::StyleRule)
	}

	/// Returns true if metadata contains at-rules.
	#[inline]
	pub fn has_at_rules(&self) -> bool {
		self.node_kinds.contains(NodeKinds::AtRule)
	}

	/// Returns true if metadata contains function nodes.
	#[inline]
	pub fn has_functions(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Function)
	}

	/// Returns true if metadata contains deprecated nodes.
	#[inline]
	pub fn is_deprecated(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Deprecated)
	}

	/// Returns true if metadata contains experimental nodes.
	#[inline]
	pub fn is_experimental(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Experimental)
	}

	/// Returns true if metadata contains non-standard nodes.
	#[inline]
	pub fn is_non_standard(&self) -> bool {
		self.node_kinds.contains(NodeKinds::NonStandard)
	}

	/// Returns true if metadata contains dimension values.
	#[inline]
	pub fn is_dimension(&self) -> bool {
		self.node_kinds.contains(NodeKinds::Dimension)
	}

	/// Returns true if metadata contains nodes with the given property kind.
	#[inline]
	pub fn has_property_kind(&self, kind: PropertyKind) -> bool {
		self.property_kinds.contains(kind)
	}

	/// Returns true if this is an empty container (no declarations, no nested rules).
	#[inline]
	pub fn is_empty_container(&self) -> bool {
		self.node_kinds.contains(NodeKinds::EmptyBlock)
	}

	/// Returns true if this node can be a container (has StyleRule or AtRule kind).
	#[inline]
	pub fn can_be_empty(&self) -> bool {
		self.node_kinds.intersects(NodeKinds::StyleRule | NodeKinds::AtRule)
	}
}

impl NodeMetadata for CssMetadata {
	#[inline]
	fn merge(mut self, other: Self) -> Self {
		self.property_groups |= other.property_groups;
		self.applies_to |= other.applies_to;
		self.box_sides |= other.box_sides;
		self.box_portions |= other.box_portions;
		self.declaration_kinds |= other.declaration_kinds;
		self.used_at_rules |= other.used_at_rules;
		self.vendor_prefixes |= other.vendor_prefixes;
		self.node_kinds |= other.node_kinds;
		self.property_kinds |= other.property_kinds;
		// For unitless_zero_resolves, we keep Number if either side has it (conservative)
		if other.unitless_zero_resolves == UnitlessZeroResolves::Number {
			self.unitless_zero_resolves = UnitlessZeroResolves::Number;
		}
		self.size = self.size.max(other.size);
		self
	}

	#[inline]
	fn with_size(mut self, size: u16) -> Self {
		self.size = size;
		self
	}
}

// Metadata is not serialized to tokens but providing these simplifies ToCursors/ToSpan impls
impl ToCursors for CssMetadata {
	fn to_cursors(&self, _: &mut impl css_parse::CursorSink) {}
}
impl ToSpan for CssMetadata {
	fn to_span(&self) -> Span {
		Span::DUMMY
	}
}

impl SemanticEq for CssMetadata {
	fn semantic_eq(&self, other: &Self) -> bool {
		self == other
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CssAtomSet, StyleSheet};
	use css_lexer::Lexer;
	use css_parse::{NodeMetadata, NodeWithMetadata, Parser};

	#[test]
	fn test_block_metadata_merge() {
		let mut meta1 = CssMetadata::default();
		meta1.property_groups = PropertyGroup::Color;
		meta1.declaration_kinds = DeclarationKind::Important;

		let mut meta2 = CssMetadata::default();
		meta2.property_groups = PropertyGroup::Position;
		meta2.declaration_kinds = DeclarationKind::Custom;

		let merged = meta1.merge(meta2);

		assert!(merged.property_groups.contains(PropertyGroup::Color));
		assert!(merged.property_groups.contains(PropertyGroup::Position));
		assert!(merged.declaration_kinds.contains(DeclarationKind::Important));
		assert!(merged.declaration_kinds.contains(DeclarationKind::Custom));
	}

	#[test]
	fn test_stylesheet_metadata_simple() {
		let css = "body { color: red; width: 100px; }";
		let bump = bumpalo::Bump::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&bump, css, lexer);
		let stylesheet = parser.parse::<StyleSheet>().unwrap();

		let metadata = stylesheet.metadata();

		assert!(metadata.property_groups.contains(PropertyGroup::Color));
		assert!(metadata.property_groups.contains(PropertyGroup::Sizing));
		assert!(metadata.modifies_box());
		assert!(metadata.has_longhands());
	}

	#[test]
	fn test_stylesheet_metadata_with_important() {
		let css = "body { color: red !important; }";
		let bump = bumpalo::Bump::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&bump, css, lexer);
		let stylesheet = parser.parse::<StyleSheet>().unwrap();

		let metadata = stylesheet.metadata();

		assert!(metadata.has_important());
		assert!(metadata.property_groups.contains(PropertyGroup::Color));
	}

	#[test]
	fn test_stylesheet_metadata_custom_properties() {
		let css = "body { --custom: value; }";
		let bump = bumpalo::Bump::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&bump, css, lexer);
		let stylesheet = parser.parse::<StyleSheet>().unwrap();

		let metadata = stylesheet.metadata();

		assert!(metadata.has_custom_properties());
	}

	#[test]
	fn test_stylesheet_metadata_nested_media() {
		let css = "@media screen { body { color: red; } }";
		let bump = bumpalo::Bump::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&bump, css, lexer);
		let stylesheet = parser.parse::<StyleSheet>().unwrap();

		let metadata = stylesheet.metadata();

		assert!(metadata.property_groups.contains(PropertyGroup::Color));
		assert!(metadata.used_at_rules.contains(AtRuleId::Media));
	}

	#[test]
	fn test_vendor_prefixes_try_from() {
		// Vendor-prefixed atoms should convert successfully
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_WebkitTransform), Ok(VendorPrefixes::WebKit));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_WebkitAnimation), Ok(VendorPrefixes::WebKit));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::WebkitLineClamp), Ok(VendorPrefixes::WebKit));

		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_MozAppearance), Ok(VendorPrefixes::Moz));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_MozAny), Ok(VendorPrefixes::Moz));

		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_MsFullscreen), Ok(VendorPrefixes::Ms));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_MsBackdrop), Ok(VendorPrefixes::Ms));

		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_OPlaceholder), Ok(VendorPrefixes::O));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::_OScrollbar), Ok(VendorPrefixes::O));

		// Non-vendor atoms should fail
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::Px), Err(()));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::Em), Err(()));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::Auto), Err(()));
		assert_eq!(VendorPrefixes::try_from(CssAtomSet::Transform), Err(()));
	}
}
