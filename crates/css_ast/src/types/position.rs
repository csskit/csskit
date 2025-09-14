use crate::LengthPercentage;
use css_parse::{Cursor, Kind, Parse, Parser, Peek, Result as ParserResult, T, Token, diagnostics, keyword_set};
use csskit_derives::{IntoCursor, ToCursors, ToSpan, Visitable};

// https://drafts.csswg.org/css-values-4/#position
// <position> = [
//   [ left | center | right | top | bottom | <length-percentage> ]
// |
//   [ left | center | right ] && [ top | center | bottom ]
// |
//   [ left | center | right | <length-percentage> ]
//   [ top | center | bottom | <length-percentage> ]
// |
//   [ [ left | right ] <length-percentage> ] &&
//   [ [ top | bottom ] <length-percentage> ]
// ]
#[derive(ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum Position {
	SingleValue(PositionSingleValue),
	TwoValue(PositionHorizontal, PositionVertical),
	FourValue(PositionHorizontalKeyword, LengthPercentage, PositionVerticalKeyword, LengthPercentage),
}

impl<'a> Peek<'a> for Position {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		PositionSingleValue::peek(p, c)
	}
}

impl<'a> Parse<'a> for Position {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let first = p.parse::<PositionSingleValue>()?;
		// Single case
		if !p.peek::<PositionSingleValue>() {
			return Ok(Self::SingleValue(first));
		}
		let second = p.parse::<PositionSingleValue>()?;
		// Two value
		if !p.peek::<PositionSingleValue>() {
			if let Some(horizontal) = first.to_horizontal() {
				if let Some(vertical) = second.to_vertical() {
					return Ok(Self::TwoValue(horizontal, vertical));
				}
			} else if let Some(horizontal) = second.to_horizontal() {
				if let Some(vertical) = first.to_vertical() {
					return Ok(Self::TwoValue(horizontal, vertical));
				} else {
					Err(diagnostics::Unexpected(second.into()))?
				}
			}
		}
		// Four value
		if matches!(first, PositionSingleValue::Center(_) | PositionSingleValue::LengthPercentage(_))
			|| !matches!(&second, PositionSingleValue::LengthPercentage(_))
		{
			Err(diagnostics::Unexpected(second.into()))?
		}
		let third = p.parse::<PositionSingleValue>()?;
		if third.to_horizontal_keyword().is_none() && third.to_vertical_keyword().is_none() {
			let cursor: Cursor = third.into();
			Err(diagnostics::UnexpectedIdent(p.parse_str(cursor).into(), cursor))?
		}
		let fourth = p.parse::<LengthPercentage>()?;
		if let PositionSingleValue::LengthPercentage(second) = second {
			if let Some(horizontal) = first.to_horizontal_keyword() {
				if let Some(vertical) = third.to_vertical_keyword() {
					Ok(Self::FourValue(horizontal, second, vertical, fourth))
				} else {
					Err(diagnostics::Unexpected(third.into()))?
				}
			} else if let Some(horizontal) = third.to_horizontal_keyword() {
				if let Some(vertical) = first.to_vertical_keyword() {
					Ok(Self::FourValue(horizontal, fourth, vertical, second))
				} else {
					Err(diagnostics::Unexpected(third.into()))?
				}
			} else {
				Err(diagnostics::Unexpected(third.into()))?
			}
		} else {
			Err(diagnostics::Unexpected(second.into()))?
		}
	}
}

keyword_set!(pub enum PositionValueKeyword { Left: "left", Right: "right", Center: "center", Top: "top", Bottom: "bottom" });

#[derive(IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionSingleValue {
	Left(T![Ident]),
	Right(T![Ident]),
	Center(T![Ident]),
	Top(T![Ident]),
	Bottom(T![Ident]),
	LengthPercentage(LengthPercentage),
}

impl PositionSingleValue {
	#[inline]
	fn to_horizontal(self) -> Option<PositionHorizontal> {
		match self {
			Self::Left(t) => Some(PositionHorizontal::Left(t)),
			Self::Right(t) => Some(PositionHorizontal::Right(t)),
			Self::Center(t) => Some(PositionHorizontal::Center(t)),
			Self::LengthPercentage(l) => Some(PositionHorizontal::LengthPercentage(l)),
			_ => None,
		}
	}

	#[inline]
	fn to_vertical(self) -> Option<PositionVertical> {
		match self {
			Self::Top(t) => Some(PositionVertical::Top(t)),
			Self::Bottom(t) => Some(PositionVertical::Bottom(t)),
			Self::Center(t) => Some(PositionVertical::Center(t)),
			Self::LengthPercentage(l) => Some(PositionVertical::LengthPercentage(l)),
			_ => None,
		}
	}

	#[inline]
	fn to_horizontal_keyword(self) -> Option<PositionHorizontalKeyword> {
		match self {
			Self::Left(t) => Some(PositionHorizontalKeyword::Left(t)),
			Self::Right(t) => Some(PositionHorizontalKeyword::Right(t)),
			_ => None,
		}
	}

	#[inline]
	fn to_vertical_keyword(self) -> Option<PositionVerticalKeyword> {
		match self {
			Self::Top(t) => Some(PositionVerticalKeyword::Top(t)),
			Self::Bottom(t) => Some(PositionVerticalKeyword::Bottom(t)),
			_ => None,
		}
	}
}

impl From<PositionSingleValue> for Kind {
	fn from(value: PositionSingleValue) -> Self {
		let t: Token = value.into();
		t.into()
	}
}

impl<'a> Peek<'a> for PositionSingleValue {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		LengthPercentage::peek(p, c) || PositionValueKeyword::peek(p, c)
	}
}

impl<'a> Parse<'a> for PositionSingleValue {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		if p.peek::<LengthPercentage>() {
			p.parse::<LengthPercentage>().map(Self::LengthPercentage)
		} else {
			match p.parse::<PositionValueKeyword>()? {
				PositionValueKeyword::Center(ident) => Ok(Self::Center(ident)),
				PositionValueKeyword::Left(ident) => Ok(Self::Left(ident)),
				PositionValueKeyword::Right(ident) => Ok(Self::Right(ident)),
				PositionValueKeyword::Top(ident) => Ok(Self::Top(ident)),
				PositionValueKeyword::Bottom(ident) => Ok(Self::Bottom(ident)),
			}
		}
	}
}

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionHorizontal {
	Left(T![Ident]),
	Right(T![Ident]),
	Center(T![Ident]),
	LengthPercentage(LengthPercentage),
}

#[derive(ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PositionVertical {
	Top(T![Ident]),
	Bottom(T![Ident]),
	Center(T![Ident]),
	LengthPercentage(LengthPercentage),
}

keyword_set!(pub enum PositionHorizontalKeyword { Left: "left", Right: "right" });

keyword_set!(pub enum PositionVerticalKeyword { Top: "top", Bottom: "bottom" });

#[cfg(test)]
mod tests {
	use crate::Length;

	use super::*;
	use css_parse::{assert_parse, assert_parse_error, assert_parse_span};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Position>(), 64);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Position, "left", Position::SingleValue(PositionSingleValue::Left(_)));
		assert_parse!(Position, "right", Position::SingleValue(PositionSingleValue::Right(_)));
		assert_parse!(Position, "top", Position::SingleValue(PositionSingleValue::Top(_)));
		assert_parse!(Position, "bottom", Position::SingleValue(PositionSingleValue::Bottom(_)));
		assert_parse!(Position, "center", Position::SingleValue(PositionSingleValue::Center(_)));
		assert_parse!(
			Position,
			"center center",
			Position::TwoValue(PositionHorizontal::Center(_), PositionVertical::Center(_))
		);
		assert_parse!(
			Position,
			"center top",
			Position::TwoValue(PositionHorizontal::Center(_), PositionVertical::Top(_))
		);
		assert_parse!(
			Position,
			"50% 50%",
			Position::TwoValue(
				PositionHorizontal::LengthPercentage(LengthPercentage::Percent(_)),
				PositionVertical::LengthPercentage(LengthPercentage::Percent(_))
			)
		);
		assert_parse!(
			Position,
			"50%",
			Position::SingleValue(PositionSingleValue::LengthPercentage(LengthPercentage::Percent(_)))
		);
		assert_parse!(
			Position,
			"20px 30px",
			Position::TwoValue(
				PositionHorizontal::LengthPercentage(LengthPercentage::Length(Length::Px(_))),
				PositionVertical::LengthPercentage(LengthPercentage::Length(Length::Px(_)))
			)
		);
		assert_parse!(
			Position,
			"2% bottom",
			Position::TwoValue(
				PositionHorizontal::LengthPercentage(LengthPercentage::Percent(_)),
				PositionVertical::Bottom(_)
			)
		);
		assert_parse!(
			Position,
			"-70% -180%",
			Position::TwoValue(
				PositionHorizontal::LengthPercentage(LengthPercentage::Percent(_)),
				PositionVertical::LengthPercentage(LengthPercentage::Percent(_))
			)
		);
		assert_parse!(
			Position,
			"right 8.5%",
			Position::TwoValue(
				PositionHorizontal::Right(_),
				PositionVertical::LengthPercentage(LengthPercentage::Percent(_))
			)
		);
		assert_parse!(
			Position,
			"right -6px bottom 12vmin",
			Position::FourValue(
				PositionHorizontalKeyword::Right(_),
				LengthPercentage::Length(Length::Px(_)),
				PositionVerticalKeyword::Bottom(_),
				LengthPercentage::Length(Length::Vmin(_))
			)
		);
		assert_parse!(
			Position,
			"bottom 12vmin right -6px",
			"right -6px bottom 12vmin",
			Position::FourValue(
				PositionHorizontalKeyword::Right(_),
				LengthPercentage::Length(Length::Px(_)),
				PositionVerticalKeyword::Bottom(_),
				LengthPercentage::Length(Length::Vmin(_))
			)
		);
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Position, "left left");
		assert_parse_error!(Position, "bottom top");
		assert_parse_error!(Position, "10px 15px 20px 15px");
		// 3 value syntax is not allowed
		assert_parse_error!(Position, "right -6px bottom");
	}

	#[test]
	fn test_spans() {
		// Parsing should stop at var()
		assert_parse_span!(
			Position,
			r#"
			right var(--foo)
			^^^^^
		"#
		);
		// Parsing should stop at four values:
		assert_parse_span!(
			Position,
			r#"
			right -6px bottom 12rem 8px 20%
			^^^^^^^^^^^^^^^^^^^^^^^
		"#
		);
	}

	// #[cfg(feature = "serde")]
	// #[test]
	// fn test_serializes() {
	// 	assert_json!(Position, "center center", {
	// 		"node": [
	// 			{"type": "center"},
	// 			{"type": "center"},
	// 		],
	// 		"start": 0,
	// 		"end": 13
	// 	});
	// 	assert_json!(Position, "left bottom", {
	// 		"node": [
	// 			{"type": "left", "value": null},
	// 			{"type": "bottom", "value": null},
	// 		],
	// 		"start": 0,
	// 		"end": 11
	// 	});
	// }
}
