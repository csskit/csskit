use css_parse::{Build, Cursor, DimensionUnit, Parser, T};
use csskit_derives::{IntoCursor, Peek, ToCursors};

// https://drafts.csswg.org/css-values/#resolution
#[derive(Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Frequency {
	Hz(T![Dimension::Hz]),
	Khz(T![Dimension::Khz]),
}

impl From<Frequency> for f32 {
	fn from(frequency: Frequency) -> Self {
		match frequency {
			Frequency::Hz(f) => f.into(),
			Frequency::Khz(f) => f.into(),
		}
	}
}

impl<'a> Build<'a> for Frequency {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		match c.token().dimension_unit() {
			DimensionUnit::Hz => Self::Hz(<T![Dimension::Hz]>::build(p, c)),
			DimensionUnit::Khz => Self::Khz(<T![Dimension::Khz]>::build(p, c)),
			_ => unreachable!(),
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Frequency>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Frequency, "40hz");
		assert_parse!(Frequency, "40khz");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(Frequency, "40w");
		assert_parse_error!(Frequency, "40kw");
	}
}
