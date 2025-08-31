use css_parse::{Build, Cursor, Parser, Peek, T, ToNumberValue};
use csskit_derives::{IntoCursor, ToCursors, Visitable};

// https://drafts.csswg.org/css-values/#resolution
#[derive(IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum Time {
	Zero(T![Number]),
	Ms(T![Dimension::Ms]),
	S(T![Dimension::S]),
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
			|| <T![Dimension::Ms]>::peek(p, c)
			|| <T![Dimension::S]>::peek(p, c)
	}
}

impl<'a> Build<'a> for Time {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		if <T![Number]>::peek(p, c) && c.token().value() == 0.0 {
			Self::Zero(<T![Number]>::build(p, c))
		} else if <T![Dimension::S]>::peek(p, c) {
			Self::S(<T![Dimension::S]>::build(p, c))
		} else {
			Self::Ms(<T![Dimension::Ms]>::build(p, c))
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
