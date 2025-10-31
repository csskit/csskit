use super::prelude::*;

// https://drafts.csswg.org/css-anchor-position-1/#typedef-position-area
// <position-area> = [
//   [ left | center | right | span-left | span-right
//   | x-start | x-end | span-x-start | span-x-end
//   | x-self-start | x-self-end | span-x-self-start | span-x-self-end
//   | span-all ]
//   ||
//   [ top | center | bottom | span-top | span-bottom
//   | y-start | y-end | span-y-start | span-y-end
//   | y-self-start | y-self-end | span-y-self-start | span-y-self-end
//   | span-all ]
// |
//   [ block-start | center | block-end | span-block-start | span-block-end | span-all ]
//   ||
//   [ inline-start | center | inline-end | span-inline-start | span-inline-end
//   | span-all ]
// |
//   [ self-block-start | center | self-block-end | span-self-block-start
//   | span-self-block-end | span-all ]
//   ||
//   [ self-inline-start | center | self-inline-end | span-self-inline-start
//   | span-self-inline-end | span-all ]
// |
//   [ start | center | end | span-start | span-end | span-all ]{1,2}
// |
//   [ self-start | center | self-end | span-self-start | span-self-end | span-all ]{1,2}
// ]
#[derive(ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum PositionArea {
	Physical(Option<PositionAreaPhsyicalHorizontal>, Option<PositionAreaPhsyicalVertical>),
	Logical(Option<PositionAreaBlock>, Option<PositionAreaInline>),
	SelfLogical(Option<PositionAreaSelfBlock>, Option<PositionAreaSelfInline>),
	Position(PositionAreaPosition, Option<PositionAreaPosition>),
	SelfPosition(PositionAreaSelfPosition, Option<PositionAreaSelfPosition>),
}

impl<'a> Peek<'a> for PositionArea {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		PositionAreaPhsyicalVertical::peek(p, c)
			|| PositionAreaPhsyicalHorizontal::peek(p, c)
			|| PositionAreaBlock::peek(p, c)
			|| PositionAreaInline::peek(p, c)
			|| PositionAreaSelfBlock::peek(p, c)
			|| PositionAreaSelfInline::peek(p, c)
			|| PositionAreaPosition::peek(p, c)
			|| PositionAreaSelfPosition::peek(p, c)
	}
}

impl<'a> Parse<'a> for PositionArea {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if let Some(first) = p.parse_if_peek::<PositionAreaPosition>()? {
			Ok(Self::Position(first, p.parse_if_peek::<PositionAreaPosition>()?))
		} else if let Some(first) = p.parse_if_peek::<PositionAreaSelfPosition>()? {
			Ok(Self::SelfPosition(first, p.parse_if_peek::<PositionAreaSelfPosition>()?))
		} else if let Some(block) = p.parse_if_peek::<PositionAreaBlock>()? {
			Ok(Self::Logical(Some(block), p.parse_if_peek::<PositionAreaInline>()?))
		} else if let Some(inline) = p.parse_if_peek::<PositionAreaInline>()? {
			Ok(Self::Logical(p.parse_if_peek::<PositionAreaBlock>()?, Some(inline)))
		} else if let Some(block) = p.parse_if_peek::<PositionAreaSelfBlock>()? {
			Ok(Self::SelfLogical(Some(block), p.parse_if_peek::<PositionAreaSelfInline>()?))
		} else if let Some(inline) = p.parse_if_peek::<PositionAreaSelfInline>()? {
			Ok(Self::SelfLogical(p.parse_if_peek::<PositionAreaSelfBlock>()?, Some(inline)))
		} else if let Some(horizontal) = p.parse_if_peek::<PositionAreaPhsyicalHorizontal>()? {
			Ok(Self::Physical(Some(horizontal), p.parse_if_peek::<PositionAreaPhsyicalVertical>()?))
		} else if let Some(vertical) = p.parse_if_peek::<PositionAreaPhsyicalVertical>()? {
			Ok(Self::Physical(p.parse_if_peek::<PositionAreaPhsyicalHorizontal>()?, Some(vertical)))
		} else {
			Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?
		}
	}
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionAreaPhsyicalHorizontal {
	#[atom(CssAtomSet::Left)]
	Left(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::Right)]
	Right(T![Ident]),
	#[atom(CssAtomSet::SpanLeft)]
	SpanLeft(T![Ident]),
	#[atom(CssAtomSet::SpanRight)]
	SpanRight(T![Ident]),
	#[atom(CssAtomSet::XStart)]
	XStart(T![Ident]),
	#[atom(CssAtomSet::XEnd)]
	XEnd(T![Ident]),
	#[atom(CssAtomSet::SpanXStart)]
	SpanXStart(T![Ident]),
	#[atom(CssAtomSet::SpanXEnd)]
	SpanXEnd(T![Ident]),
	#[atom(CssAtomSet::XSelfStart)]
	XSelfStart(T![Ident]),
	#[atom(CssAtomSet::XSelfEnd)]
	XSelfEnd(T![Ident]),
	#[atom(CssAtomSet::SpanXSelfStart)]
	SpanXSelfStart(T![Ident]),
	#[atom(CssAtomSet::SpanXSelfEnd)]
	SpanXSelfEnd(T![Ident]),
	#[atom(CssAtomSet::SpanAll)]
	SpanAll(T![Ident]),
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionAreaPhsyicalVertical {
	#[atom(CssAtomSet::Top)]
	Top(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::Bottom)]
	Bottom(T![Ident]),
	#[atom(CssAtomSet::SpanTop)]
	SpanTop(T![Ident]),
	#[atom(CssAtomSet::SpanBottom)]
	SpanBottom(T![Ident]),
	#[atom(CssAtomSet::YStart)]
	YStart(T![Ident]),
	#[atom(CssAtomSet::YEnd)]
	YEnd(T![Ident]),
	#[atom(CssAtomSet::SpanYStart)]
	SpanYStart(T![Ident]),
	#[atom(CssAtomSet::SpanYEnd)]
	SpanYEnd(T![Ident]),
	#[atom(CssAtomSet::YSelfStart)]
	YSelfStart(T![Ident]),
	#[atom(CssAtomSet::YSelfEnd)]
	YSelfEnd(T![Ident]),
	#[atom(CssAtomSet::SpanYSelfStart)]
	SpanYSelfStart(T![Ident]),
	#[atom(CssAtomSet::SpanYSelfEnd)]
	SpanYSelfEnd(T![Ident]),
	#[atom(CssAtomSet::SpanAll)]
	SpanAll(T![Ident]),
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionAreaBlock {
	#[atom(CssAtomSet::BlockStart)]
	BlockStart(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::BlockEnd)]
	BlockEnd(T![Ident]),
	#[atom(CssAtomSet::SpanBlockStart)]
	SpanBlockStart(T![Ident]),
	#[atom(CssAtomSet::SpanBlockEnd)]
	SpanBlockEnd(T![Ident]),
	#[atom(CssAtomSet::SpanAll)]
	SpanAll(T![Ident]),
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionAreaInline {
	#[atom(CssAtomSet::InlineStart)]
	InlineStart(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::InlineEnd)]
	InlineEnd(T![Ident]),
	#[atom(CssAtomSet::SpanInlineStart)]
	SpanInlineStart(T![Ident]),
	#[atom(CssAtomSet::SpanInlineEnd)]
	SpanInlineEnd(T![Ident]),
	#[atom(CssAtomSet::SpanAll)]
	SpanAll(T![Ident]),
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionAreaSelfBlock {
	#[atom(CssAtomSet::SelfBlockStart)]
	SelfBlockStart(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::SelfBlockEnd)]
	SelfBlockEnd(T![Ident]),
	#[atom(CssAtomSet::SpanSelfBlockStart)]
	SpanSelfBlockStart(T![Ident]),
	#[atom(CssAtomSet::SpanSelfBlockEnd)]
	SpanSelfBlockEnd(T![Ident]),
	#[atom(CssAtomSet::SpanAll)]
	SpanAll(T![Ident]),
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionAreaSelfInline {
	#[atom(CssAtomSet::SelfInlineStart)]
	SelfInlineStart(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::SelfInlineEnd)]
	SelfInlineEnd(T![Ident]),
	#[atom(CssAtomSet::SpanSelfInlineStart)]
	SpanSelfInlineStart(T![Ident]),
	#[atom(CssAtomSet::SpanSelfInlineEnd)]
	SpanSelfInlineEnd(T![Ident]),
	#[atom(CssAtomSet::SpanAll)]
	SpanAll(T![Ident]),
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionAreaPosition {
	#[atom(CssAtomSet::Start)]
	Start(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::End)]
	End(T![Ident]),
	#[atom(CssAtomSet::SpanStart)]
	SpanStart(T![Ident]),
	#[atom(CssAtomSet::SpanEnd)]
	SpanEnd(T![Ident]),
	#[atom(CssAtomSet::SpanAll)]
	SpanAll(T![Ident]),
}

#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionAreaSelfPosition {
	#[atom(CssAtomSet::SelfStart)]
	SelfStart(T![Ident]),
	#[atom(CssAtomSet::Center)]
	Center(T![Ident]),
	#[atom(CssAtomSet::SelfEnd)]
	SelfEnd(T![Ident]),
	#[atom(CssAtomSet::SpanSelfStart)]
	SpanSelfStart(T![Ident]),
	#[atom(CssAtomSet::SpanSelfEnd)]
	SpanSelfEnd(T![Ident]),
	#[atom(CssAtomSet::SpanAll)]
	SpanAll(T![Ident]),
}
