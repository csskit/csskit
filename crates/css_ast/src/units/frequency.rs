use super::prelude::*;

// https://drafts.csswg.org/css-values/#resolution
#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum Frequency {
	#[atom(CssAtomSet::Hz)]
	Hz(T![Dimension]),
	#[atom(CssAtomSet::Khz)]
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

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Frequency>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Frequency, "40hz");
		assert_parse!(CssAtomSet::ATOMS, Frequency, "40khz");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, Frequency, "40w");
		assert_parse_error!(CssAtomSet::ATOMS, Frequency, "40kw");
	}
}
