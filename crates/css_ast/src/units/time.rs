use super::prelude::*;

/// <https://drafts.csswg.org/css-values/#time>
///
/// ```text,ignore
/// <time> = <dimension-token>
/// ```
#[node]
#[derive(
	IntoCursor, ToSpan, SemanticEq, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Dimension, value_kinds = Time)]
pub enum Time {
	#[atom(CssAtomSet::Ms)]
	Ms(T![Dimension]),
	#[atom(CssAtomSet::S)]
	S(T![Dimension]),
}

impl Time {
	pub fn as_seconds(&self) -> f32 {
		match self {
			Self::Ms(f) => Into::<f32>::into(*f) / 1000.0,
			Self::S(f) => (*f).into(),
		}
	}
}

impl From<Time> for f32 {
	fn from(val: Time) -> Self {
		match val {
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

impl ToNormalisedValue for Time {
	fn to_normalised_value(&self) -> Option<f32> {
		Some(self.as_seconds())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Time, "0s");
		assert_parse!(CssAtomSet::ATOMS, Time, "0ms");
		assert_parse!(CssAtomSet::ATOMS, Time, "1s");
		assert_parse!(CssAtomSet::ATOMS, Time, "100ms");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, Time, "0");
		assert_peek_false!(CssAtomSet::ATOMS, Time, "1");
		assert_peek_false!(CssAtomSet::ATOMS, Time, "foo");
	}
}
