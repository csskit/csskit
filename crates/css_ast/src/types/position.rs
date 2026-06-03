use super::prelude::*;
use crate::LengthPercentage;

/// <https://drafts.csswg.org/css-values-5/#typedef-position>
///
/// ```text,ignore
/// <position> = <position-one> | <position-two> | <position-four>
/// ```
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Position {
	One(PositionOne),
	Two(PositionTwo),
	Four(PositionFour),
}

impl<'a> Peek<'a> for Position {
	const PEEK_KINDSET: KindSet = PositionOne::PEEK_KINDSET;

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		PositionOne::peek(p, c)
	}
}

impl<'a> Parse<'a> for Position {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let first = p.parse::<PositionOne>()?;
		if !p.peek::<PositionOne>() {
			return Ok(Self::One(first));
		}
		let second = p.parse::<PositionOne>()?;
		if !p.peek::<PositionOne>() {
			return Ok(Self::Two(PositionTwo::from_two(p, first, second)?));
		}
		// Four-value: first two tokens must form a keyword + LP pair
		Ok(Self::Four(PositionFour::from_four(p, first, second)?))
	}
}

/// <https://drafts.csswg.org/css-values-5/#typedef-position-one>
///
/// ```text,ignore
/// <position-one> = [
///   left | center | right | top | bottom |
///   x-start | x-end | y-start | y-end |
///   block-start | block-end | inline-start | inline-end |
///   start | end |
///   <length-percentage>
/// ]
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionOne {
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::Top)]
	Top(T![Ident]),
	#[atom(CssAtomSet::Bottom)]
	Bottom(T![Ident]),
	#[atom(CssAtomSet::XStart)]
	XStart(T![Ident]),
	#[atom(CssAtomSet::XEnd)]
	XEnd(T![Ident]),
	#[atom(CssAtomSet::YStart)]
	YStart(T![Ident]),
	#[atom(CssAtomSet::YEnd)]
	YEnd(T![Ident]),
	#[atom(CssAtomSet::BlockStart)]
	BlockStart(T![Ident]),
	#[atom(CssAtomSet::BlockEnd)]
	BlockEnd(T![Ident]),
	#[atom(CssAtomSet::InlineStart)]
	InlineStart(T![Ident]),
	#[atom(CssAtomSet::InlineEnd)]
	InlineEnd(T![Ident]),
	#[atom(CssAtomSet::Start)]
	Start(T![Ident]),
	#[atom(CssAtomSet::End)]
	End(T![Ident]),
	LengthPercentage(LengthPercentage),
}

impl PositionOne {
	pub(crate) fn to_horizontal(self) -> Option<PositionHorizontal> {
		match self {
			Self::Left(t) => Some(PositionHorizontal::Left(t)),
			Self::Right(t) => Some(PositionHorizontal::Right(t)),
			Self::Center(t) => Some(PositionHorizontal::Center(t)),
			Self::XStart(t) => Some(PositionHorizontal::XStart(t)),
			Self::XEnd(t) => Some(PositionHorizontal::XEnd(t)),
			Self::LengthPercentage(l) => Some(PositionHorizontal::LengthPercentage(l)),
			_ => None,
		}
	}

	pub(crate) fn to_vertical(self) -> Option<PositionVertical> {
		match self {
			Self::Top(t) => Some(PositionVertical::Top(t)),
			Self::Bottom(t) => Some(PositionVertical::Bottom(t)),
			Self::Center(t) => Some(PositionVertical::Center(t)),
			Self::YStart(t) => Some(PositionVertical::YStart(t)),
			Self::YEnd(t) => Some(PositionVertical::YEnd(t)),
			Self::LengthPercentage(l) => Some(PositionVertical::LengthPercentage(l)),
			_ => None,
		}
	}

	pub(crate) fn to_horizontal_keyword(self) -> Option<PositionHorizontalKeyword> {
		match self {
			Self::Left(t) => Some(PositionHorizontalKeyword::Left(t)),
			Self::Right(t) => Some(PositionHorizontalKeyword::Right(t)),
			Self::XStart(t) => Some(PositionHorizontalKeyword::XStart(t)),
			Self::XEnd(t) => Some(PositionHorizontalKeyword::XEnd(t)),
			_ => None,
		}
	}

	pub(crate) fn to_vertical_keyword(self) -> Option<PositionVerticalKeyword> {
		match self {
			Self::Top(t) => Some(PositionVerticalKeyword::Top(t)),
			Self::Bottom(t) => Some(PositionVerticalKeyword::Bottom(t)),
			Self::YStart(t) => Some(PositionVerticalKeyword::YStart(t)),
			Self::YEnd(t) => Some(PositionVerticalKeyword::YEnd(t)),
			_ => None,
		}
	}

	pub(crate) fn to_block_axis(self) -> Option<PositionBlockAxis> {
		match self {
			Self::BlockStart(t) => Some(PositionBlockAxis::BlockStart(t)),
			Self::BlockEnd(t) => Some(PositionBlockAxis::BlockEnd(t)),
			Self::Center(t) => Some(PositionBlockAxis::Center(t)),
			_ => None,
		}
	}

	pub(crate) fn to_inline_axis(self) -> Option<PositionInlineAxis> {
		match self {
			Self::InlineStart(t) => Some(PositionInlineAxis::InlineStart(t)),
			Self::InlineEnd(t) => Some(PositionInlineAxis::InlineEnd(t)),
			Self::Center(t) => Some(PositionInlineAxis::Center(t)),
			_ => None,
		}
	}

	pub(crate) fn to_block_axis_keyword(self) -> Option<PositionBlockAxisKeyword> {
		match self {
			Self::BlockStart(t) => Some(PositionBlockAxisKeyword::BlockStart(t)),
			Self::BlockEnd(t) => Some(PositionBlockAxisKeyword::BlockEnd(t)),
			_ => None,
		}
	}

	pub(crate) fn to_inline_axis_keyword(self) -> Option<PositionInlineAxisKeyword> {
		match self {
			Self::InlineStart(t) => Some(PositionInlineAxisKeyword::InlineStart(t)),
			Self::InlineEnd(t) => Some(PositionInlineAxisKeyword::InlineEnd(t)),
			_ => None,
		}
	}

	pub(crate) fn to_logical(self) -> Option<PositionLogical> {
		match self {
			Self::Start(t) => Some(PositionLogical::Start(t)),
			Self::End(t) => Some(PositionLogical::End(t)),
			_ => None,
		}
	}
}

/// <https://drafts.csswg.org/css-values-5/#typedef-position-two>
///
/// ```text,ignore
/// <position-two> = [
///   [ left | center | right | x-start | x-end ] &&
///   [ top | center | bottom | y-start | y-end ]
/// |
///   [ left | center | right | x-start | x-end | <lp> ]
///   [ top | center | bottom | y-start | y-end | <lp> ]
/// |
///   [ block-start | center | block-end ] &&
///   [ inline-start | center | inline-end ]
/// |
///   [ start | center | end ]{2}
/// ]
/// ```
///
/// All forms normalise to (primary-axis, secondary-axis) order in the AST.
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionTwo {
	/// Physical horizontal/vertical axes, with optional `<length-percentage>`.
	/// Stored horizontal-first regardless of source order.
	Physical(PositionHorizontal, PositionVertical),
	/// Flow-relative block/inline axes.
	/// Stored block-first regardless of source order.
	FlowRelative(PositionBlockAxis, PositionInlineAxis),
	/// Axis-ambiguous `start`/`end` pair; first = block axis, second = inline axis.
	Logical(PositionLogical, PositionLogical),
}

impl PositionTwo {
	pub(crate) fn from_two<'a, I>(_p: &mut Parser<'a, I>, first: PositionOne, second: PositionOne) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// Try physical form: one must be horizontal, the other vertical
		let h = first.to_horizontal();
		let v = second.to_vertical();
		if let (Some(h), Some(v)) = (h, v) {
			return Ok(Self::Physical(h, v));
		}
		// Reversed physical (vertical first, horizontal second)
		let v = first.to_vertical();
		let h = second.to_horizontal();
		if let (Some(v), Some(h)) = (v, h) {
			return Ok(Self::Physical(h, v));
		}
		// Flow-relative: block && inline (reorderable)
		let block = first.to_block_axis();
		let inline = second.to_inline_axis();
		if let (Some(block), Some(inline)) = (block, inline) {
			return Ok(Self::FlowRelative(block, inline));
		}
		let inline = first.to_inline_axis();
		let block = second.to_block_axis();
		if let (Some(inline), Some(block)) = (inline, block) {
			return Ok(Self::FlowRelative(block, inline));
		}
		// Logical: start|end pair
		let a = first.to_logical();
		let b = second.to_logical();
		if let (Some(a), Some(b)) = (a, b) {
			return Ok(Self::Logical(a, b));
		}
		Err(Diagnostic::new(second.into(), Diagnostic::unexpected))?
	}
}

/// <https://drafts.csswg.org/css-values-5/#typedef-position-four>
///
/// ```text,ignore
/// <position-four> = [
///   [ [ left | right | x-start | x-end ] <lp> ] &&
///   [ [ top | bottom | y-start | y-end ] <lp> ]
/// |
///   [ [ block-start | block-end ] <lp> ] &&
///   [ [ inline-start | inline-end ] <lp> ]
/// |
///   [ [ start | end ] <lp> ]{2}
/// ]
/// ```
///
/// All forms stored with the first keyword-axis pair first.
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionFour {
	/// `[left|right|x-start|x-end] <lp>` && `[top|bottom|y-start|y-end] <lp>`
	/// Stored horizontal-first.
	Physical(PositionHorizontalKeyword, LengthPercentage, PositionVerticalKeyword, LengthPercentage),
	/// `[block-start|block-end] <lp>` && `[inline-start|inline-end] <lp>`
	/// Stored block-first.
	FlowRelative(PositionBlockAxisKeyword, LengthPercentage, PositionInlineAxisKeyword, LengthPercentage),
	/// `[start|end] <lp>` × 2; first = block axis, second = inline axis.
	Logical(PositionLogical, LengthPercentage, PositionLogical, LengthPercentage),
}

impl PositionFour {
	pub(crate) fn from_four<'a, I>(p: &mut Parser<'a, I>, first: PositionOne, second: PositionOne) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// second must be a <length-percentage> in all four-value forms except
		// when the pair is reversed (vertical/block/logical first).
		// Pattern: <keyword> <lp> <keyword> <lp>  (or reversed)
		let second_lp = if let PositionOne::LengthPercentage(lp) = second { Some(lp) } else { None };

		if let Some(lp1) = second_lp {
			let third = p.parse::<PositionOne>()?;
			let fourth = p.parse::<LengthPercentage>()?;
			// Physical: first=H keyword, third=V keyword
			if let Some(h_kw) = first.to_horizontal_keyword()
				&& let Some(v_kw) = third.to_vertical_keyword()
			{
				return Ok(Self::Physical(h_kw, lp1, v_kw, fourth));
			}
			// Physical reversed: first=V keyword, third=H keyword
			if let Some(v_kw) = first.to_vertical_keyword()
				&& let Some(h_kw) = third.to_horizontal_keyword()
			{
				return Ok(Self::Physical(h_kw, fourth, v_kw, lp1));
			}
			// Flow-relative: first=block keyword, third=inline keyword
			if let Some(b_kw) = first.to_block_axis_keyword()
				&& let Some(i_kw) = third.to_inline_axis_keyword()
			{
				return Ok(Self::FlowRelative(b_kw, lp1, i_kw, fourth));
			}
			// Flow-relative reversed: first=inline keyword, third=block keyword
			if let Some(i_kw) = first.to_inline_axis_keyword()
				&& let Some(b_kw) = third.to_block_axis_keyword()
			{
				return Ok(Self::FlowRelative(b_kw, fourth, i_kw, lp1));
			}
			// Logical: first=start|end, third=start|end
			if let Some(a) = first.to_logical()
				&& let Some(b) = third.to_logical()
			{
				return Ok(Self::Logical(a, lp1, b, fourth));
			}
			Err(Diagnostic::new(third.into(), Diagnostic::unexpected))?
		} else {
			// second is not LP — must be a keyword; first is the offset LP
			// Pattern: <keyword> <keyword> <lp>  is invalid for four-value
			Err(Diagnostic::new(second.into(), Diagnostic::unexpected))?
		}
	}
}

/// Horizontal axis keywords and `<length-percentage>`.
///
/// `left | center | right | x-start | x-end | <length-percentage>`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionHorizontal {
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::XStart)]
	XStart(T![Ident]),
	#[atom(CssAtomSet::XEnd)]
	XEnd(T![Ident]),
	LengthPercentage(LengthPercentage),
}

/// Vertical axis keywords and `<length-percentage>`.
///
/// `top | center | bottom | y-start | y-end | <length-percentage>`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionVertical {
	#[atom(CssAtomSet::Top)]
	Top(T![Ident]),
	#[atom(CssAtomSet::Bottom)]
	Bottom(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::YStart)]
	YStart(T![Ident]),
	#[atom(CssAtomSet::YEnd)]
	YEnd(T![Ident]),
	LengthPercentage(LengthPercentage),
}

/// Block axis keywords (flow-relative).
///
/// `block-start | block-end | center`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionBlockAxis {
	#[atom(CssAtomSet::BlockStart)]
	BlockStart(T![Ident]),
	#[atom(CssAtomSet::BlockEnd)]
	BlockEnd(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
}

/// Inline axis keywords (flow-relative).
///
/// `inline-start | inline-end | center`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionInlineAxis {
	#[atom(CssAtomSet::InlineStart)]
	InlineStart(T![Ident]),
	#[atom(CssAtomSet::InlineEnd)]
	InlineEnd(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
}

/// Axis-ambiguous logical keywords.
///
/// `start | end`
///
/// When used in a two-value position, the first represents the block axis and
/// the second the inline axis.
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionLogical {
	#[atom(CssAtomSet::Start)]
	Start(T![Ident]),
	#[atom(CssAtomSet::End)]
	End(T![Ident]),
}

/// Horizontal edge keywords without `<length-percentage>` (for four-value syntax).
///
/// `left | right | x-start | x-end`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionHorizontalKeyword {
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
	#[atom(CssAtomSet::XStart)]
	XStart(T![Ident]),
	#[atom(CssAtomSet::XEnd)]
	XEnd(T![Ident]),
}

/// Vertical edge keywords without `<length-percentage>` (for four-value syntax).
///
/// `top | bottom | y-start | y-end`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionVerticalKeyword {
	#[atom(CssAtomSet::Top)]
	Top(T![Ident]),
	#[atom(CssAtomSet::Bottom)]
	Bottom(T![Ident]),
	#[atom(CssAtomSet::YStart)]
	YStart(T![Ident]),
	#[atom(CssAtomSet::YEnd)]
	YEnd(T![Ident]),
}

/// Block axis edge keywords without `center` (for four-value syntax).
///
/// `block-start | block-end`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionBlockAxisKeyword {
	#[atom(CssAtomSet::BlockStart)]
	BlockStart(T![Ident]),
	#[atom(CssAtomSet::BlockEnd)]
	BlockEnd(T![Ident]),
}

/// Inline axis edge keywords without `center` (for four-value syntax).
///
/// `inline-start | inline-end`
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum PositionInlineAxisKeyword {
	#[atom(CssAtomSet::InlineStart)]
	InlineStart(T![Ident]),
	#[atom(CssAtomSet::InlineEnd)]
	InlineEnd(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_parse_span};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Position>(), 68);
	}

	#[test]
	fn test_writes() {
		// One-value
		assert_parse!(CssAtomSet::ATOMS, Position, "left", Position::One(PositionOne::Left(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "right", Position::One(PositionOne::Right(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "top", Position::One(PositionOne::Top(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "bottom", Position::One(PositionOne::Bottom(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "center", Position::One(PositionOne::Center(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "x-start", Position::One(PositionOne::XStart(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "x-end", Position::One(PositionOne::XEnd(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "y-start", Position::One(PositionOne::YStart(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "y-end", Position::One(PositionOne::YEnd(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "block-start", Position::One(PositionOne::BlockStart(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "block-end", Position::One(PositionOne::BlockEnd(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "inline-start", Position::One(PositionOne::InlineStart(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "inline-end", Position::One(PositionOne::InlineEnd(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "start", Position::One(PositionOne::Start(_)));
		assert_parse!(CssAtomSet::ATOMS, Position, "end", Position::One(PositionOne::End(_)));
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"50%",
			Position::One(PositionOne::LengthPercentage(LengthPercentage::Percent(_)))
		);
		// Two-value physical
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"center center",
			Position::Two(PositionTwo::Physical(PositionHorizontal::Center(_), PositionVertical::Center(_)))
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"center top",
			Position::Two(PositionTwo::Physical(PositionHorizontal::Center(_), PositionVertical::Top(_)))
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"50% 50%",
			Position::Two(PositionTwo::Physical(
				PositionHorizontal::LengthPercentage(LengthPercentage::Percent(_)),
				PositionVertical::LengthPercentage(LengthPercentage::Percent(_))
			))
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"20px 30px",
			Position::Two(PositionTwo::Physical(
				PositionHorizontal::LengthPercentage(_),
				PositionVertical::LengthPercentage(_)
			))
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"2% bottom",
			Position::Two(PositionTwo::Physical(
				PositionHorizontal::LengthPercentage(LengthPercentage::Percent(_)),
				PositionVertical::Bottom(_)
			))
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"-70% -180%",
			Position::Two(PositionTwo::Physical(
				PositionHorizontal::LengthPercentage(LengthPercentage::Percent(_)),
				PositionVertical::LengthPercentage(LengthPercentage::Percent(_))
			))
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"right 8.5%",
			Position::Two(PositionTwo::Physical(
				PositionHorizontal::Right(_),
				PositionVertical::LengthPercentage(LengthPercentage::Percent(_))
			))
		);
		// Two-value physical with new keywords
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"x-start y-end",
			Position::Two(PositionTwo::Physical(PositionHorizontal::XStart(_), PositionVertical::YEnd(_)))
		);
		// Two-value flow-relative
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"block-start inline-end",
			Position::Two(PositionTwo::FlowRelative(
				PositionBlockAxis::BlockStart(_),
				PositionInlineAxis::InlineEnd(_)
			))
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"inline-end block-start",
			Position::Two(PositionTwo::FlowRelative(
				PositionBlockAxis::BlockStart(_),
				PositionInlineAxis::InlineEnd(_)
			))
		);
		// Two-value logical
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"start end",
			Position::Two(PositionTwo::Logical(PositionLogical::Start(_), PositionLogical::End(_)))
		);
		// Four-value physical
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"right -6px bottom 12vmin",
			Position::Four(PositionFour::Physical(
				PositionHorizontalKeyword::Right(_),
				LengthPercentage::Length(_),
				PositionVerticalKeyword::Bottom(_),
				LengthPercentage::Length(_)
			))
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"bottom 12vmin right -6px",
			Position::Four(PositionFour::Physical(
				PositionHorizontalKeyword::Right(_),
				LengthPercentage::Length(_),
				PositionVerticalKeyword::Bottom(_),
				LengthPercentage::Length(_)
			))
		);
		// Four-value flow-relative
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"block-start 10px inline-end 20px",
			Position::Four(PositionFour::FlowRelative(
				PositionBlockAxisKeyword::BlockStart(_),
				LengthPercentage::Length(_),
				PositionInlineAxisKeyword::InlineEnd(_),
				LengthPercentage::Length(_)
			))
		);
		// Four-value logical
		assert_parse!(
			CssAtomSet::ATOMS,
			Position,
			"start 10px end 20px",
			Position::Four(PositionFour::Logical(
				PositionLogical::Start(_),
				LengthPercentage::Length(_),
				PositionLogical::End(_),
				LengthPercentage::Length(_)
			))
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Position, "left left");
		assert_parse_error!(CssAtomSet::ATOMS, Position, "bottom top");
		assert_parse_error!(CssAtomSet::ATOMS, Position, "10px 15px 20px 15px");
		// 3 value syntax is not allowed
		assert_parse_error!(CssAtomSet::ATOMS, Position, "right -6px bottom");
	}

	#[test]
	fn test_spans() {
		// Parsing should stop at var()
		assert_parse_span!(
			CssAtomSet::ATOMS,
			Position,
			r#"
			right var(--foo)
			^^^^^
		"#
		);
		// Parsing should stop at four values:
		assert_parse_span!(
			CssAtomSet::ATOMS,
			Position,
			r#"
			right -6px bottom 12rem 8px 20%
			^^^^^^^^^^^^^^^^^^^^^^^
		"#
		);
	}
}
