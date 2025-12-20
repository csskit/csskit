#![deny(warnings)]

//! CSS AST query utilities using selector-like syntax.

pub mod selector;
#[cfg(test)]
mod test_helpers;

use css_parse::AtomSet;
use derive_atom_set::AtomSet;

pub use selector::{
	MatchOutput, NthValue, QueryAttribute, QueryCombinator, QueryPseudo, QuerySelector, QuerySelectorList,
	QuerySelectorPart, QuerySimpleSelector, SelectorMatcher, SelectorParseError,
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
