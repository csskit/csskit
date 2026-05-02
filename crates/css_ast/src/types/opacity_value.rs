use super::prelude::*;
use crate::Percentage;

#[derive(IntoCursor, Peek, Parse, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum OpacityValue {
	Number(T![Number]),
	Percent(Percentage),
}

impl OpacityValue {
	#[allow(non_upper_case_globals)]
	pub const Zero: OpacityValue = OpacityValue::Number(<T![Number]>::NUMBER_ZERO);
}

impl From<OpacityValue> for i32 {
	fn from(value: OpacityValue) -> Self {
		match value {
			OpacityValue::Number(t) => t.into(),
			OpacityValue::Percent(t) => {
				let f: f32 = t.into();
				f as i32
			}
		}
	}
}

impl From<OpacityValue> for f32 {
	fn from(value: OpacityValue) -> Self {
		match value {
			OpacityValue::Number(t) => t.into(),
			OpacityValue::Percent(t) => t.into(),
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
		assert_eq!(std::mem::size_of::<OpacityValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, OpacityValue, "0.1");
		assert_parse!(CssAtomSet::ATOMS, OpacityValue, "1");
		assert_parse!(CssAtomSet::ATOMS, OpacityValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, OpacityValue, "20");
		assert_parse!(CssAtomSet::ATOMS, OpacityValue, "1000%");
		assert_parse!(CssAtomSet::ATOMS, OpacityValue, "-2");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, OpacityValue, "red");
		assert_parse_error!(CssAtomSet::ATOMS, OpacityValue, "10px");
	}
}
