use css_parse::{Cursor, Diagnostic, DimensionUnit, Parse, Parser, Peek, Result, T, ToNumberValue};
use csskit_derives::{IntoCursor, ToCursors, Visitable};

// https://drafts.csswg.org/css-values/#resolution
#[derive(IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum Time {
	Zero(T![Number]),
	Ms(T![Dimension]),
	S(T![Dimension]),
}

impl From<Time> for f32 {
	fn from(val: Time) -> Self {
		match val {
			Time::Zero(_) => 0.0,
			Time::Ms(f) => f.into(),
			Time::S(f) => f.into(),
		}
	}
}

impl ToNumberValue for Time {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl<'a> Peek<'a> for Time {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		(<T![Number]>::peek(p, c) && c.token().value() == 0.0)
			|| (<T![Dimension]>::peek(p, c) && matches!(p.parse_str_lower(c), "ms" | "s"))
	}
}

impl<'a> Parse<'a> for Time {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		if p.peek::<T![Number]>() {
			p.parse::<T![Number]>().and_then(|number| {
				if number.value() == 0.0 {
					Ok(Self::Zero(number))
				} else {
					Err(Diagnostic::new(number.into(), Diagnostic::unexpected))
				}
			})
		} else {
			let dimension = p.parse::<T![Dimension]>()?;
			match dimension.dimension_unit() {
				DimensionUnit::S => Ok(Self::S(dimension)),
				DimensionUnit::Ms => Ok(Self::Ms(dimension)),
				_ => Err(Diagnostic::new(dimension.into(), Diagnostic::unexpected))?,
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Time>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Time, "0");
		assert_parse!(Time, "0s");
		assert_parse!(Time, "0ms");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Time, "1");
		assert_parse_error!(Time, "foo");
	}
}
