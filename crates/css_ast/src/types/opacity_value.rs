use css_parse::{Cursor, Parser, Peek, T};
use csskit_derives::{IntoCursor, Parse, ToCursors, Visitable};

use crate::Percentage;

#[derive(IntoCursor, Parse, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[visit(self)]
pub enum OpacityValue {
	Number(#[parse(in_range=0.0..=1.0)] T![Number]),
	Percent(#[parse(in_range=0.0..=100.0)] Percentage),
}

impl OpacityValue {
	#[allow(non_upper_case_globals)]
	pub const Zero: OpacityValue = OpacityValue::Number(<T![Number]>::NUMBER_ZERO);
}

impl From<OpacityValue> for i32 {
	fn from(value: OpacityValue) -> Self {
		match value {
			OpacityValue::Number(t) => t.into(),
			OpacityValue::Percent(t) => t.into(),
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

impl<'a> Peek<'a> for OpacityValue {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		(<T![Number]>::peek(p, c) && (0.0..=1.0).contains(&c.token().value()))
			|| (<Percentage>::peek(p, c) && (0.0..=100.0).contains(&c.token().value()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<OpacityValue>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(OpacityValue, "0.1");
		assert_parse!(OpacityValue, "1");
		assert_parse!(OpacityValue, "50%");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(OpacityValue, "20");
		assert_parse_error!(OpacityValue, "1000%");
	}

	// #[cfg(feature = "serde")]
	// #[test]
	// fn test_serializes() {
	// 	assert_json!(OpacityValue, "0.1", {
	// 		"node": [0, 3],
	// 		"start": 0,
	// 		"end": 5
	// 	});
	// }
}
