use css_parse::{DimensionUnit, Parse, Parser, Result, T, diagnostics};
use csskit_derives::{IntoCursor, Peek, ToCursors};

// https://drafts.csswg.org/css-values/#resolution
#[derive(Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Frequency {
	Hz(T![Dimension]),
	Khz(T![Dimension]),
}

impl From<Frequency> for f32 {
	fn from(frequency: Frequency) -> Self {
		match frequency {
			Frequency::Hz(f) => f.into(),
			Frequency::Khz(f) => f.into(),
		}
	}
}

impl<'a> Parse<'a> for Frequency {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let c = p.peek_n(1);
		match c.token().dimension_unit() {
			DimensionUnit::Hz => p.parse::<T![Dimension]>().map(Self::Hz),
			DimensionUnit::Khz => p.parse::<T![Dimension]>().map(Self::Khz),
			_ => Err(diagnostics::Unexpected(p.next()))?,
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
