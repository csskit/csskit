use bitmask_enum::bitmask;

use crate::CssAtomSet;

/// The CSS specification/module that a property belongs to.
#[bitmask(u64)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum PropertyGroup {
	Align,
	AnchorPosition,
	Animations,
	Backgrounds,
	Borders,
	Box,
	Break,
	Cascade,
	Color,
	ColorAdjust,
	ColorHdr,
	Conditional,
	Contain,
	Content,
	Display,
	Exclusions,
	Flexbox,
	Fonts,
	Forms,
	Gaps,
	Gcpm,
	Grid,
	Images,
	Inline,
	LineGrid,
	LinkParams,
	Lists,
	Logical,
	Multicol,
	Nav,
	Overflow,
	Overscroll,
	Page,
	PageFloats,
	Position,
	Regions,
	Rhythm,
	RoundDisplay,
	Ruby,
	ScrollAnchoring,
	ScrollSnap,
	Scrollbars,
	Shapes,
	SizeAdjust,
	Sizing,
	Speech,
	Tables,
	Text,
	TextDecor,
	Transforms,
	Transitions,
	Ui,
	Values,
	Variables,
	ViewTransitions,
	Viewport,
	WillChange,
	WritingModes,
}

pub enum Inherits {
	False,
	True,
	Unknown,
}

impl Inherits {
	pub fn to_bool(self, unknown: bool) -> bool {
		match self {
			Self::False => false,
			Self::True => true,
			Self::Unknown => unknown,
		}
	}
}

pub enum Percentages {
	/// This style value has no way of expressing values as a percentage.
	None,
	/// Any percentage expressed in this value pertains to the size of the containing block.
	ContainingBlock,
	/// Any percentage expressed in this value pertains to the size of the border box.
	BorderBox,
	/// Any percentage expressed in this value is a syntax affordance; a Number token would be the equivalent value.
	Number,
	/// Relative to the 1em Font-Size
	FontSize,
	/// Relative to the Font-Size of the parent element
	ParentFontSize,
	/// Relative to the scroll container's scrollport
	Scrollport,
	/// Relative to the content area dimension
	ContentArea,
	/// Relative to the border-edge side length
	BorderEdge,
	/// Relative to the background positioning area
	BackgroundPositioningArea,
	/// Relative to the reference box size
	ReferenceBox,
	/// Relative to the element's own dimensions
	SelfSize,
	/// Relative to the line box
	LineBox,
	/// Relative to the flex container
	FlexContainer,
	/// Relative to the border image area
	BorderImageArea,
	/// Map to a normalized range (e.g., `[0,1]`)
	NormalizedRange,
	/// Unknown or complex percentage resolution
	Unknown,
}

/// The type of element or container this style value applies to.
#[bitmask(u16)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum AppliesTo {
	/// Any element which is `display: block` or equivalent.
	Block,
	/// Any element which is `display: grid` or equivalent.
	Grid,
	/// Any element which is `display: flex` or equivalent.
	Flex,
	/// Any inline-level box.
	Inline,
	/// Any floated element.
	Float,
	/// Any Ruby container
	Ruby,
	/// Any absolutely positioned element.
	AbsPos,
	/// Any text node.
	Text,
	/// Any Pseudo Elements
	PseudoElements,
	/// Any Element
	Elements,
	/// What this applies to still needs to be established.
	Unknown,
}

pub enum AnimationType {
	/// This property is not animatable.
	None,
	/// This property animates between discrete values.
	Discrete,
	/// Animates by interpolating computed values
	ByComputedValue,
	/// Each item in a list animates independently
	RepeatableList,
	/// Animates as a transform list
	TransformList,
	/// Animates as a shadow list
	ShadowList,
	/// Animates as a length value
	Length,
	/// Animates as a number value
	Number,
	/// Unknown or complex animation behavior
	Unknown,
}

/// How the computed value is calculated from the specified value
pub enum ComputedValueType {
	/// The computed value is the same as the specified value
	AsSpecified,
	/// Computed to an absolute length
	AbsoluteLength,
	/// Computed to an absolute length or percentage
	AbsoluteLengthOrPercentage,
	/// Computed to an absolute length or 'none'
	AbsoluteLengthOrNone,
	/// A specified keyword plus an absolute length
	SpecifiedKeywordPlusAbsoluteLength,
	/// Two absolute lengths (e.g., for background-position)
	TwoAbsoluteLengths,
	/// A list of absolute lengths
	ListOfAbsoluteLengths,
	/// Computed as specified, but with relative lengths converted to absolute
	SpecifiedWithAbsoluteLengths,
	/// Computed as specified, but with relative URLs converted to absolute
	SpecifiedWithAbsoluteUrls,
	/// Special computation rules - see spec
	SeeIndividualProperties,
	/// Computed value calculation is complex or spec-specific
	Complex,
	/// Not yet categorized
	Unknown,
}

/// Which side(s) of the box a property applies to.
/// This is a bitmask so properties can apply to multiple sides.
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BoxSide {
	/// Applies to the physical top side
	Top,
	/// Applies to the physical bottom side
	Bottom,
	/// Applies to the physical left side
	Left,
	/// Applies to the physical right side
	Right,
	/// Applies to the logical block-start side
	BlockStart,
	/// Applies to the logical block-end side
	BlockEnd,
	/// Applies to the logical inline-start side
	InlineStart,
	/// Applies to the logical inline-end side
	InlineEnd,
}

/// Which portion(s) of the box model a property affects.
/// This is a bitmask so properties can affect multiple portions.
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum BoxPortion {
	/// Affects the content size (width/height)
	Size,
	/// Affects the margin area
	Margin,
	/// Affects the padding area
	Padding,
	/// Affects the border area
	Border,
	/// Affects the position/placement of the box
	Position,
}

pub trait DeclarationMetadata: PartialEq + Sized + Clone {
	/// Returns the initial value of this property, as a string
	fn initial() -> &'static str;

	/// Determines if this style value inherits from parent rules
	fn inherits() -> Inherits {
		// Most properties do not inherit, so this is a sensible default
		Inherits::False
	}

	/// Determines what types of frames this rule applies to
	fn applies_to() -> AppliesTo {
		AppliesTo::none()
	}

	/// Determines how this style value resolves percentages, if they are allowed as values
	fn percentages() -> Percentages {
		Percentages::None
	}

	/// Returns how this style value animates
	fn animation_type() -> AnimationType {
		// Most properties do not animate, so this is a sensible default
		AnimationType::None
	}

	/// Determines if this style value is a "shorthand" value, meaning it is comprised of other "longhand" style values.
	fn is_shorthand() -> bool {
		false
	}

	/// Determines if this style value is a "longhand" value, meaning a "shorthand" style value exists that could also
	/// express this.
	fn is_longhand() -> bool {
		Self::shorthand_group() == CssAtomSet::_None
	}

	/// Returns a list of direct "longhand" declaration IDs, or None if this is not a shorthand.
	/// For nested shorthands, this only returns the immediate children.
	/// Use `StyleValue::expanded_longhands_by_name()` to get all transitive longhands.
	fn longhands() -> Option<&'static [CssAtomSet]> {
		None
	}

	/// Returns the declaration ID of the shorthand that this property is part of.
	/// If this is not a longhand then it will be `CssAtomSet::_None`.
	fn shorthand_group() -> CssAtomSet {
		CssAtomSet::_None
	}

	/// Returns which CSS specification(s) this property belongs to.
	/// This allows tracking which CSS modules are used in a stylesheet.
	fn property_group() -> PropertyGroup {
		PropertyGroup::none()
	}

	/// Returns how the computed value is calculated from the specified value.
	fn computed_value_type() -> ComputedValueType {
		ComputedValueType::Unknown
	}

	/// Returns the canonical order for serialization (e.g., "per grammar", "unique").
	/// Returns None if not specified or not applicable.
	fn canonical_order() -> Option<&'static str> {
		None
	}

	/// Returns the logical property group this property belongs to (e.g., "Margin", "Border").
	/// This groups related logical/physical properties together.
	/// Returns None if this is not part of a logical property group.
	fn logical_property_group() -> Option<CssAtomSet> {
		None
	}

	/// Returns which side(s) of the box this property applies to.
	/// For example, `margin-top` returns BoxSide::Top, while `margin` returns all sides.
	/// Returns BoxSide::none() if the property doesn't apply to a specific side.
	fn box_side() -> BoxSide {
		BoxSide::none()
	}

	/// Returns which portion(s) of the box model this property affects.
	/// For example, `margin-top` returns BoxPortion::Margin, `border-width` returns BoxPortion::Border.
	/// Returns BoxPortion::none() if the property doesn't affect the box model.
	fn box_portion() -> BoxPortion {
		BoxPortion::none()
	}
}
