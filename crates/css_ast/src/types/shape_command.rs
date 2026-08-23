use super::prelude::*;
use crate::{Angle, CalcableValue, LengthPercentage};
use css_parse::Box;

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-command>
///
/// ```text,ignore
/// <shape-command> = <move-command> | <line-command> | close |
///                   <horizontal-line-command> | <vertical-line-command> |
///                   <curve-command> | <smooth-command> | <arc-command>
/// ```
#[syntax(
	" <move-command> | <line-command> | close | <horizontal-line-command> | <vertical-line-command> | <curve-command> | <smooth-command> | <arc-command> "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ShapeCommand<'a> {}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-coordinate-pair>
///
/// ```text,ignore
/// <coordinate-pair> = <length-percentage>{2}
/// ```
#[syntax(" <length-percentage>{2} ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CoordinatePair<'a>;

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-command-end-point>
///
/// ```text,ignore
/// <command-end-point> = [ to <position> | by <coordinate-pair> ]
/// ```
#[syntax(" to <position> | by <coordinate-pair> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CommandEndPoint<'a> {}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-control-point>
///
/// ```text,ignore
/// <control-point> = [ <position> | <relative-control-point> ]
/// ```
#[syntax(" <position> | <relative-control-point> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ControlPoint<'a> {}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-relative-control-point>
///
/// ```text,ignore
/// <relative-control-point> = <coordinate-pair> [ from [ start | end | origin ] ]?
/// ```
#[syntax(" <coordinate-pair> [ from [ start | end | origin ] ]? ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RelativeControlPoint<'a>;

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-arc-sweep>
///
/// ```text,ignore
/// <arc-sweep> = cw | ccw
/// ```
#[syntax(" cw | ccw ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ArcSweep<'a> {}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-arc-size>
///
/// ```text,ignore
/// <arc-size> = large | small
/// ```
#[syntax(" large | small ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ArcSize<'a> {}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-move-command>
///
/// ```text,ignore
/// <move-command> = move <command-end-point>
/// ```
///
/// Written by hand (rather than via `#[syntax]`) because a struct whose
/// entire body is a single leading literal keyword hits a bug in the
/// `#[syntax]` macro's generated `ToSpan`/`Parse` impls for its synthesized
/// keyword-marker type.
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct MoveCommand<'a> {
	#[atom(CssAtomSet::Move)]
	pub keyword: T![Ident],
	pub point: CommandEndPoint<'a>,
}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-line-command>
///
/// ```text,ignore
/// <line-command> = line <command-end-point>
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LineCommand<'a> {
	#[atom(CssAtomSet::Line)]
	pub keyword: T![Ident],
	pub point: CommandEndPoint<'a>,
}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-horizontal-line-command>
///
/// ```text,ignore
/// <horizontal-line-command> = hline
///         [ to [ <length-percentage> | left | center | right | x-start | x-end ]
///         | by <length-percentage> ]
/// ```
#[syntax(" <length-percentage> | left | center | right | x-start | x-end ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum HorizontalLineValue<'a> {}

#[syntax(" to <horizontal-line-value> | by <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum HorizontalLineClause<'a> {}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct HorizontalLineCommand<'a> {
	#[atom(CssAtomSet::Hline)]
	pub keyword: T![Ident],
	pub clause: HorizontalLineClause<'a>,
}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-vertical-line-command>
///
/// ```text,ignore
/// <vertical-line-command> = vline
///         [ to [ <length-percentage> | top | center | bottom | y-start | y-end ]
///         | by <length-percentage> ]
/// ```
///
#[syntax(" <length-percentage> | top | center | bottom | y-start | y-end ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum VerticalLineValue<'a> {}

#[syntax(" to <vertical-line-value> | by <length-percentage> ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum VerticalLineClause<'a> {}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct VerticalLineCommand<'a> {
	#[atom(CssAtomSet::Vline)]
	pub keyword: T![Ident],
	pub clause: VerticalLineClause<'a>,
}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-curve-command>
///
/// ```text,ignore
/// <curve-command> = curve
///         [ [ to <position> with <control-point> [ / <control-point> ]? ]
///         | [ by <coordinate-pair> with <relative-control-point> [ / <relative-control-point> ]? ] ]
/// ```
#[syntax(
	" to <position> with <control-point> [ / <control-point> ]? | by <coordinate-pair> with <relative-control-point> [ / <relative-control-point> ]? "
)]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CurveTarget<'a> {}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CurveCommand<'a> {
	#[atom(CssAtomSet::Curve)]
	pub keyword: T![Ident],
	pub target: Box<'a, CurveTarget<'a>>,
}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-smooth-command>
///
/// ```text,ignore
/// <smooth-command> = smooth
///         [ [ to <position> [ with <control-point> ]? ]
///         | [ by <coordinate-pair> [ with <relative-control-point> ]? ] ]
/// ```
#[syntax(" to <position> [ with <control-point> ]? | by <coordinate-pair> [ with <relative-control-point> ]? ")]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SmoothTarget<'a> {}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SmoothCommand<'a> {
	#[atom(CssAtomSet::Smooth)]
	pub keyword: T![Ident],
	pub target: Box<'a, SmoothTarget<'a>>,
}

/// <https://drafts.csswg.org/css-shapes/#typedef-shape-arc-command>
///
/// ```text,ignore
/// <arc-command> = arc <command-end-point>
///             [ [ of <length-percentage>{1,2} ]
///               && <arc-sweep>? && <arc-size>? && [rotate <angle>]? ]
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ArcRadii<'a> {
	#[atom(CssAtomSet::Of)]
	pub keyword: T![Ident],
	pub horizontal: CalcableValue<'a, LengthPercentage>,
	pub vertical: Option<CalcableValue<'a, LengthPercentage>>,
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ArcRotate<'a> {
	#[atom(CssAtomSet::Rotate)]
	pub keyword: T![Ident],
	pub angle: CalcableValue<'a, Angle>,
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[parse(all_must_occur)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ArcCommandParams<'a> {
	pub radii: ArcRadii<'a>,
	pub sweep: Option<ArcSweep<'a>>,
	pub size: Option<ArcSize<'a>>,
	pub rotate: Option<ArcRotate<'a>>,
}

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ArcCommand<'a> {
	#[atom(CssAtomSet::Arc)]
	pub keyword: T![Ident],
	pub point: CommandEndPoint<'a>,
	pub params: Box<'a, ArcCommandParams<'a>>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "close");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "move to 10px 10px");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "line by 10px 10px");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "hline to 50%");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "hline by 10px");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "vline to bottom");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "curve to 100% 5% with 100% 0%");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "curve by 10px 10px with 5px 5px / 8px 8px");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "smooth to 100% 5%");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "smooth by 10px 10px with 5px 5px");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "arc to 10px 10px of 5px 5px");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "arc to 10px 10px of 5px cw large rotate 10deg");
		assert_parse!(CssAtomSet::ATOMS, ShapeCommand, "arc to 10px 10px of calc(5px) rotate var(--a)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ShapeCommand, "foo");
		assert_parse_error!(CssAtomSet::ATOMS, ShapeCommand, "move");
		assert_parse_error!(CssAtomSet::ATOMS, ShapeCommand, "arc to 10px 10px");
	}
}
