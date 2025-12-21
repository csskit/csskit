use crate::{
	CssAtomSet,
	traits::{AppliesTo, BoxPortion, BoxSide, PropertyGroup},
};
use bitmask_enum::bitmask;
use css_lexer::{Span, ToSpan};
use css_parse::{NodeMetadata, SemanticEq, ToCursors};

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
	/// If an unknown declaration was used
	Unknown,
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

#[bitmask(u16)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum RuleKind {
	/// If a rule is unknown
	Unknown,
	/// If a rule is a nested at-rules
	NestedAtRule,
	/// If a rule is a nested style-rule
	NestedStyleRules,
}

/// Categories of nodes present in metadata, used for selector filtering.
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum NodeKinds {
	/// Contains style rules
	StyleRule,
	/// Contains at-rules (media, keyframes, etc.)
	AtRule,
	/// Contains function nodes
	Function,
}

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
	/// Bitwise OR of all RuleKind values
	pub rule_kinds: RuleKind,
	/// Bitwise OR of all AtRuleIds in a Node
	pub used_at_rules: AtRuleId,
	/// Bitwise OR of all VendorPrefixes in a Node
	pub vendor_prefixes: VendorPrefixes,
	/// Bitwise OR of node categories present
	pub node_kinds: NodeKinds,
}

impl Default for CssMetadata {
	fn default() -> Self {
		Self {
			property_groups: PropertyGroup::none(),
			applies_to: AppliesTo::none(),
			box_sides: BoxSide::none(),
			box_portions: BoxPortion::none(),
			declaration_kinds: DeclarationKind::none(),
			rule_kinds: RuleKind::none(),
			used_at_rules: AtRuleId::none(),
			vendor_prefixes: VendorPrefixes::none(),
			node_kinds: NodeKinds::none(),
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
			&& self.rule_kinds == RuleKind::none()
			&& self.used_at_rules == AtRuleId::none()
			&& self.vendor_prefixes == VendorPrefixes::none()
			&& self.node_kinds == NodeKinds::none()
	}

	/// Returns true if this block modifies any positioning-related properties.
	#[inline]
	pub fn modifies_box(&self) -> bool {
		!self.box_portions.is_none()
	}

	/// Returns true if this block contains any color-related properties.
	#[inline]
	pub fn has_colors(&self) -> bool {
		self.property_groups.intersects(PropertyGroup::Color | PropertyGroup::ColorHdr | PropertyGroup::ColorAdjust)
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
		self.rule_kinds |= other.rule_kinds;
		self.used_at_rules |= other.used_at_rules;
		self.vendor_prefixes |= other.vendor_prefixes;
		self.node_kinds |= other.node_kinds;
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
		assert!(metadata.has_colors());
		assert!(metadata.declaration_kinds.contains(DeclarationKind::Longhands));
	}

	#[test]
	fn test_stylesheet_metadata_with_important() {
		let css = "body { color: red !important; }";
		let bump = bumpalo::Bump::new();
		let lexer = Lexer::new(&CssAtomSet::ATOMS, css);
		let mut parser = Parser::new(&bump, css, lexer);
		let stylesheet = parser.parse::<StyleSheet>().unwrap();

		let metadata = stylesheet.metadata();

		assert!(metadata.declaration_kinds.contains(DeclarationKind::Important));
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

		assert!(metadata.declaration_kinds.contains(DeclarationKind::Custom));
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
		assert!(metadata.has_colors());
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
