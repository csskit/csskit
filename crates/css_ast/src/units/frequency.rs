use super::prelude::*;

/// <https://drafts.csswg.org/css-values/#frequency>
///
/// ```text,ignore
/// <frequency> = <dimension-token>
/// ```
#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Dimension)]
pub enum Frequency {
	#[atom(CssAtomSet::Hz)]
	Hz(T![Dimension]),
	#[atom(CssAtomSet::Khz)]
	Khz(T![Dimension]),
}

impl Frequency {
	pub fn as_hertz(&self) -> f32 {
		match self {
			Self::Hz(f) => (*f).into(),
			Self::Khz(f) => Into::<f32>::into(*f) * 1000.0,
		}
	}
}

impl From<Frequency> for f32 {
	fn from(frequency: Frequency) -> Self {
		match frequency {
			Frequency::Hz(f) => f.into(),
			Frequency::Khz(f) => f.into(),
		}
	}
}

impl ToNumberValue for Frequency {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl ToNormalisedValue for Frequency {
	fn to_normalised_value(&self) -> Option<f32> {
		Some(self.as_hertz())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Frequency, "40hz");
		assert_parse!(CssAtomSet::ATOMS, Frequency, "40khz");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, Frequency, "40w");
		assert_peek_false!(CssAtomSet::ATOMS, Frequency, "40kw");
	}
}
