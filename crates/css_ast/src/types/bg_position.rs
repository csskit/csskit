use super::prelude::*;
use crate::{
	LengthPercentage, PositionFour, PositionHorizontal, PositionHorizontalKeyword, PositionOne, PositionTwo,
	PositionVertical, PositionVerticalKeyword,
};

/// <https://drafts.csswg.org/css-backgrounds-4/#typedef-bg-position>
///
/// ```text,ignore
/// <bg-position> = <position> | <position-three>
/// <position-three> = [
///   [ left | center | right ] && [ [ top | bottom ] <length-percentage> ]
/// |
///   [ [ left | right ] <length-percentage> ] && [ top | center | bottom ]
/// ]
/// ```
///
/// Extends `<position>` with 3-value forms only valid in `background-position`.
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum BgPosition {
	/// Single value syntax
	One(PositionOne),
	/// Two value syntax
	Two(PositionTwo),
	/// Three value syntax where the Horizontal axis may be "center"
	ThreeHorizontal(PositionHorizontal, PositionVerticalKeyword, LengthPercentage),
	/// Three value syntax where the Vertical axis may be "center"
	ThreeVertical(PositionHorizontalKeyword, LengthPercentage, PositionVertical),
	/// Four value syntax
	Four(PositionFour),
}

impl<'a> Peek<'a> for BgPosition {
	const PEEK_KINDSET: KindSet = PositionOne::PEEK_KINDSET;

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		PositionOne::peek(p, c)
	}
}

impl<'a> Parse<'a> for BgPosition {
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

		if let Some(h_kw) = first.to_horizontal_keyword() {
			if let PositionOne::LengthPercentage(lp) = second {
				// `H L ? ...` — three-value or four-value
				let third = p.parse::<PositionOne>()?;
				if let Some(v) = third.to_vertical() {
					let fourth = if third.to_vertical_keyword().is_some() {
						p.parse_if_peek::<LengthPercentage>()?
					} else {
						None
					};
					if let Some(fourth) = fourth
						&& let Some(v_kw) = third.to_vertical_keyword()
					{
						// Four-value physical: `left 10px bottom 20px`
						return Ok(Self::Four(PositionFour::Physical(h_kw, lp, v_kw, fourth)));
					} else if fourth.is_none() {
						// Three-value: `left 10px center`
						return Ok(Self::ThreeVertical(h_kw, lp, v));
					}
				}
				Err(Diagnostic::new(third.into(), Diagnostic::unexpected))?
			} else if let Some(v_kw) = second.to_vertical_keyword() {
				// `H V L` — three-value: `left bottom 10px`
				let third = p.parse::<LengthPercentage>()?;
				return Ok(Self::ThreeHorizontal(PositionHorizontal::from_keyword(h_kw), v_kw, third));
			}
			Err(Diagnostic::new(second.into(), Diagnostic::unexpected))?
		} else if matches!(first, PositionOne::Center(_)) {
			if let Some(v_kw) = second.to_vertical_keyword() {
				// Three-value: `center bottom 10px`
				let third = p.parse::<LengthPercentage>()?;
				return Ok(Self::ThreeHorizontal(
					PositionHorizontal::Center(match first {
						PositionOne::Center(t) => t,
						_ => unreachable!(),
					}),
					v_kw,
					third,
				));
			} else if let Some(v) = second.to_vertical() {
				return Ok(Self::Two(PositionTwo::Physical(
					PositionHorizontal::Center(match first {
						PositionOne::Center(t) => t,
						_ => unreachable!(),
					}),
					v,
				)));
			}
			Err(Diagnostic::new(second.into(), Diagnostic::unexpected))?
		}
		// Vertical keyword first — could be reversed four-value handled by Position::Four
		// e.g. `bottom 20px left 10px`
		Ok(Self::Four(PositionFour::from_four(p, first, second)?))
	}
}

impl PositionHorizontal {
	pub(crate) fn from_keyword(kw: PositionHorizontalKeyword) -> Self {
		match kw {
			PositionHorizontalKeyword::Left(t) => Self::Left(t),
			PositionHorizontalKeyword::Right(t) => Self::Right(t),
			PositionHorizontalKeyword::XStart(t) => Self::XStart(t),
			PositionHorizontalKeyword::XEnd(t) => Self::XEnd(t),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BgPosition>(), 68);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "center");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "left");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "50%");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "center center");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "left top");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "0 0");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "-80px 0");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "right 8px bottom 20px");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "left 10px top");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "left 10px bottom");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "right 10px center");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "center bottom 10px");
		assert_parse!(CssAtomSet::ATOMS, BgPosition, "left bottom 10px");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, BgPosition, "");
		assert_parse_error!(CssAtomSet::ATOMS, BgPosition, "left left");
		assert_parse_error!(CssAtomSet::ATOMS, BgPosition, "top top");
	}
}
