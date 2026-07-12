use super::prelude::*;

#[derive(
	Peek, Parse, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Dimension)]
pub struct Percentage(#[atom(CssAtomSet::Percentage)] T![Dimension]);

impl Percentage {
	pub fn value(&self) -> f32 {
		self.0.into()
	}
}

impl From<Percentage> for f32 {
	fn from(percentage: Percentage) -> Self {
		percentage.0.into()
	}
}

impl From<Percentage> for i32 {
	fn from(percentage: Percentage) -> Self {
		f32::from(percentage) as i32
	}
}

impl ToNumberValue for Percentage {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl ToNormalisedValue for Percentage {
	fn to_normalised_value(&self) -> Option<f32> {
		self.to_number_value()
	}
}

#[derive(
	Peek, Parse, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Dimension)]
pub enum NumberPercentage {
	Number(T![Number]),
	Percentage(Percentage),
}

impl From<NumberPercentage> for f32 {
	fn from(val: NumberPercentage) -> Self {
		match val {
			NumberPercentage::Number(n) => n.into(),
			NumberPercentage::Percentage(n) => n.into(),
		}
	}
}

impl ToNumberValue for NumberPercentage {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

impl ToNormalisedValue for NumberPercentage {
	fn to_normalised_value(&self) -> Option<f32> {
		self.to_number_value()
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Percentage>(), 12);
		assert_eq!(std::mem::size_of::<NumberPercentage>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Percentage, "1%");
	}
}
