#![deny(warnings)]

//! CSS AST query utilities using selector-like syntax.

pub mod diagnostics;
pub mod selector;
#[cfg(test)]
mod test_helpers;

use css_ast::{PropertyGroup, VendorPrefixes};
use css_parse::AtomSet;
use derive_atom_set::AtomSet;

pub use selector::{
	MatchOutput, QueryAttribute, QueryAttributeValue, QueryCombinator, QueryCompoundSelector,
	QueryFunctionalPseudoClass, QueryNotPseudo, QueryNthPseudo, QueryPrefixedPseudo, QueryPropertyTypePseudo,
	QueryPseudoClass, QuerySelectorComponent, QuerySelectorList, QueryType, QueryWildcard, SelectorMatcher,
};

#[derive(AtomSet, Debug, Default, Copy, Clone, PartialEq, Eq)]
pub enum CsskitAtomSet {
	#[default]
	_None,

	// Vendor prefixes
	Webkit,
	Moz,
	Ms,
	O,

	// Pseudo-classes
	Important,
	Custom,
	Prefixed,
	Unknown,
	Computed,
	Shorthand,
	Longhand,
	#[atom("property-type")]
	PropertyType,
	Empty,
	Nested,
	Root,
	#[atom("first-child")]
	FirstChild,
	#[atom("last-child")]
	LastChild,
	#[atom("only-child")]
	OnlyChild,
	#[atom("nth-child")]
	NthChild,
	#[atom("nth-last-child")]
	NthLastChild,
	#[atom("first-of-type")]
	FirstOfType,
	#[atom("last-of-type")]
	LastOfType,
	#[atom("only-of-type")]
	OnlyOfType,
	#[atom("nth-of-type")]
	NthOfType,
	#[atom("nth-last-of-type")]
	NthLastOfType,
	Not,
	#[atom("at-rule")]
	AtRule,
	Rule,
	Function,
	Block,

	// Property groups
	Align,
	Anchor,
	#[atom("anchor-position")]
	AnchorPosition,
	Animation,
	Animations,
	Background,
	Backgrounds,
	Border,
	Borders,
	Box,
	Break,
	Cascade,
	Color,
	#[atom("color-adjust")]
	ColorAdjust,
	#[atom("color-hdr")]
	ColorHdr,
	Conditional,
	Contain,
	Content,
	Display,
	Exclusions,
	Flex,
	Flexbox,
	Font,
	Fonts,
	Forms,
	Gap,
	Gaps,
	Gcpm,
	Grid,
	Image,
	Images,
	Inline,
	#[atom("line-grid")]
	LineGrid,
	#[atom("link-params")]
	LinkParams,
	List,
	Lists,
	Logical,
	Mask,
	Masking,
	Multicol,
	Nav,
	Overflow,
	Overscroll,
	Page,
	#[atom("page-floats")]
	PageFloats,
	Position,
	Regions,
	Rhythm,
	#[atom("round-display")]
	RoundDisplay,
	Ruby,
	#[atom("scroll-anchoring")]
	ScrollAnchoring,
	#[atom("scroll-snap")]
	ScrollSnap,
	Scrollbar,
	Scrollbars,
	Shaders,
	Shape,
	Shapes,
	#[atom("size-adjust")]
	SizeAdjust,
	Sizing,
	Speech,
	Table,
	Tables,
	Text,
	#[atom("text-decor")]
	TextDecor,
	#[atom("text-decoration")]
	TextDecoration,
	Transform,
	Transforms,
	Transition,
	Transitions,
	Ui,
	Values,
	Variables,
	#[atom("view-transitions")]
	ViewTransitions,
	Viewport,
	#[atom("will-change")]
	WillChange,
	#[atom("writing-modes")]
	WritingModes,
}

impl CsskitAtomSet {
	pub const ATOMS: CsskitAtomSet = CsskitAtomSet::_None;
}

impl CsskitAtomSet {
	/// Converts VendorPrefixes bitmask to CsskitAtomSet.
	/// Returns None if no prefix or multiple prefixes are set.
	pub fn from_vendor_prefix(prefix: VendorPrefixes) -> Option<Self> {
		if prefix.bits().count_ones() != 1 {
			return None;
		}
		if prefix.contains(VendorPrefixes::WebKit) {
			Some(Self::Webkit)
		} else if prefix.contains(VendorPrefixes::Moz) {
			Some(Self::Moz)
		} else if prefix.contains(VendorPrefixes::Ms) {
			Some(Self::Ms)
		} else if prefix.contains(VendorPrefixes::O) {
			Some(Self::O)
		} else {
			None
		}
	}

	/// Converts a CsskitAtomSet representing a vendor name to VendorPrefixes.
	pub fn to_vendor_prefix(self) -> Option<VendorPrefixes> {
		match self {
			Self::Webkit => Some(VendorPrefixes::WebKit),
			Self::Moz => Some(VendorPrefixes::Moz),
			Self::Ms => Some(VendorPrefixes::Ms),
			Self::O => Some(VendorPrefixes::O),
			_ => None,
		}
	}

	/// Converts a CsskitAtomSet representing a property group name to PropertyGroup.
	pub fn to_property_group(self) -> Option<PropertyGroup> {
		match self {
			Self::Align => Some(PropertyGroup::Align),
			Self::Anchor | Self::AnchorPosition => Some(PropertyGroup::AnchorPosition),
			Self::Animation | Self::Animations => Some(PropertyGroup::Animations),
			Self::Background | Self::Backgrounds => Some(PropertyGroup::Backgrounds),
			Self::Border | Self::Borders => Some(PropertyGroup::Borders),
			Self::Box => Some(PropertyGroup::Box),
			Self::Break => Some(PropertyGroup::Break),
			Self::Cascade => Some(PropertyGroup::Cascade),
			Self::Color => Some(PropertyGroup::Color),
			Self::ColorAdjust => Some(PropertyGroup::ColorAdjust),
			Self::ColorHdr => Some(PropertyGroup::ColorHdr),
			Self::Conditional => Some(PropertyGroup::Conditional),
			Self::Contain => Some(PropertyGroup::Contain),
			Self::Content => Some(PropertyGroup::Content),
			Self::Display => Some(PropertyGroup::Display),
			Self::Exclusions => Some(PropertyGroup::Exclusions),
			Self::Flex | Self::Flexbox => Some(PropertyGroup::Flexbox),
			Self::Font | Self::Fonts => Some(PropertyGroup::Fonts),
			Self::Forms => Some(PropertyGroup::Forms),
			Self::Gap | Self::Gaps => Some(PropertyGroup::Gaps),
			Self::Gcpm => Some(PropertyGroup::Gcpm),
			Self::Grid => Some(PropertyGroup::Grid),
			Self::Image | Self::Images => Some(PropertyGroup::Images),
			Self::Inline => Some(PropertyGroup::Inline),
			Self::LineGrid => Some(PropertyGroup::LineGrid),
			Self::LinkParams => Some(PropertyGroup::LinkParams),
			Self::List | Self::Lists => Some(PropertyGroup::Lists),
			Self::Logical => Some(PropertyGroup::Logical),
			Self::Mask | Self::Masking => Some(PropertyGroup::Masking),
			Self::Multicol => Some(PropertyGroup::Multicol),
			Self::Nav => Some(PropertyGroup::Nav),
			Self::Overflow => Some(PropertyGroup::Overflow),
			Self::Overscroll => Some(PropertyGroup::Overscroll),
			Self::Page => Some(PropertyGroup::Page),
			Self::PageFloats => Some(PropertyGroup::PageFloats),
			Self::Position => Some(PropertyGroup::Position),
			Self::Regions => Some(PropertyGroup::Regions),
			Self::Rhythm => Some(PropertyGroup::Rhythm),
			Self::RoundDisplay => Some(PropertyGroup::RoundDisplay),
			Self::Ruby => Some(PropertyGroup::Ruby),
			Self::ScrollAnchoring => Some(PropertyGroup::ScrollAnchoring),
			Self::ScrollSnap => Some(PropertyGroup::ScrollSnap),
			Self::Scrollbar | Self::Scrollbars => Some(PropertyGroup::Scrollbars),
			Self::Shaders => Some(PropertyGroup::Shaders),
			Self::Shape | Self::Shapes => Some(PropertyGroup::Shapes),
			Self::SizeAdjust => Some(PropertyGroup::SizeAdjust),
			Self::Sizing => Some(PropertyGroup::Sizing),
			Self::Speech => Some(PropertyGroup::Speech),
			Self::Table | Self::Tables => Some(PropertyGroup::Tables),
			Self::Text => Some(PropertyGroup::Text),
			Self::TextDecor | Self::TextDecoration => Some(PropertyGroup::TextDecor),
			Self::Transform | Self::Transforms => Some(PropertyGroup::Transforms),
			Self::Transition | Self::Transitions => Some(PropertyGroup::Transitions),
			Self::Ui => Some(PropertyGroup::Ui),
			Self::Values => Some(PropertyGroup::Values),
			Self::Variables => Some(PropertyGroup::Variables),
			Self::ViewTransitions => Some(PropertyGroup::ViewTransitions),
			Self::Viewport => Some(PropertyGroup::Viewport),
			Self::WillChange => Some(PropertyGroup::WillChange),
			Self::WritingModes => Some(PropertyGroup::WritingModes),
			_ => None,
		}
	}
}
