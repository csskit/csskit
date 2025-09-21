use super::prelude::*;

// https://drafts.csswg.org/css-values/#resolution
#[derive(IntoCursor, Parse, Peek, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum Time {
	Zero(#[in_range(0.0..0.0)] T![Number]),
	#[atom(CssAtomSet::Ms)]
	Ms(T![Dimension]),
	#[atom(CssAtomSet::S)]
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Time>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Time, "0");
		assert_parse!(CssAtomSet::ATOMS, Time, "0s");
		assert_parse!(CssAtomSet::ATOMS, Time, "0ms");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Time, "1");
		assert_parse_error!(CssAtomSet::ATOMS, Time, "foo");
	}
}
