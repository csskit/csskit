use super::prelude::*;
use crate::{
	LengthPercentage, Position, PositionHorizontal, PositionHorizontalKeyword, PositionSingleValue,
	PositionVerticalKeyword,
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
	/// Standard 1, 2, or 4-value `<position>`.
	Standard(Position),
	/// `[ left | center | right ] [ top | bottom ] <length-percentage>`
	/// e.g. `center bottom 10px`
	HorizontalVerticalOffset(PositionSingleValue, PositionVerticalKeyword, LengthPercentage),
	/// `[ left | right ] <length-percentage> [ top | center | bottom ]`
	/// e.g. `left 10px center`
	HorizontalOffsetVertical(PositionHorizontalKeyword, LengthPercentage, PositionSingleValue),
}

impl<'a> Peek<'a> for BgPosition {
	const PEEK_KINDSET: KindSet = PositionSingleValue::PEEK_KINDSET;

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		PositionSingleValue::peek(p, c)
	}
}

impl<'a> Parse<'a> for BgPosition {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let first = p.parse::<PositionSingleValue>()?;

		if !p.peek::<PositionSingleValue>() {
			return Ok(Self::Standard(Position::SingleValue(first)));
		}

		let second = p.parse::<PositionSingleValue>()?;
		if !p.peek::<PositionSingleValue>() {
			if let Some(h) = first.to_horizontal() {
				if let Some(v) = second.to_vertical() {
					return Ok(Self::Standard(Position::TwoValue(h, v)));
				}
			} else if let Some(h) = second.to_horizontal()
				&& let Some(v) = first.to_vertical()
			{
				return Ok(Self::Standard(Position::TwoValue(h, v)));
			}
			Err(Diagnostic::new(second.into(), Diagnostic::unexpected))?
		}

		if let Some(h_kw) = first.to_horizontal_keyword() {
			if let PositionSingleValue::LengthPercentage(lp) = second {
				if let Some(v) = p.parse_if_peek::<PositionSingleValue>()? {
					if let Some(_v_single) = v.to_vertical() {
						if !p.peek::<LengthPercentage>() {
							return Ok(Self::HorizontalOffsetVertical(h_kw, lp, v));
						}
						let fourth = p.parse::<LengthPercentage>()?;
						if let Some(v_kw) = v.to_vertical_keyword() {
							return Ok(Self::Standard(Position::FourValue(h_kw, lp, v_kw, fourth)));
						}
						Err(Diagnostic::new(v.into(), Diagnostic::unexpected))?
					} else {
						Err(Diagnostic::new(v.into(), Diagnostic::unexpected))?
					}
				} else {
					Err(Diagnostic::new(second.into(), Diagnostic::unexpected))?
				}
			} else {
				if let Some(v_kw) = second.to_vertical_keyword() {
					let third = p.parse::<LengthPercentage>()?;
					if !p.peek::<PositionSingleValue>() {
						return Ok(Self::HorizontalVerticalOffset(first, v_kw, third));
					}
					Err(Diagnostic::new(third.into(), Diagnostic::unexpected))?
				} else {
					Err(Diagnostic::new(second.into(), Diagnostic::unexpected))?
				}
			}
		} else if matches!(first, PositionSingleValue::Center(_)) {
			if let Some(v_kw) = second.to_vertical_keyword() {
				let third = p.parse::<LengthPercentage>()?;
				return Ok(Self::HorizontalVerticalOffset(first, v_kw, third));
			}
			if let Some(v) = second.to_vertical() {
				return Ok(Self::Standard(Position::TwoValue(
					PositionHorizontal::Center(match first {
						PositionSingleValue::Center(t) => t,
						_ => unreachable!(),
					}),
					v,
				)));
			}
			Err(Diagnostic::new(second.into(), Diagnostic::unexpected))?
		} else {
			Err(Diagnostic::new(second.into(), Diagnostic::unexpected))?
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BgPosition>(), 64);
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
		assert_parse_error!(CssAtomSet::ATOMS, BgPosition, "");
		assert_parse_error!(CssAtomSet::ATOMS, BgPosition, "left left");
		assert_parse_error!(CssAtomSet::ATOMS, BgPosition, "top top");
	}
}
