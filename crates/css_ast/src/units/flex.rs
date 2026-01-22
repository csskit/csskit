use super::prelude::*;

// https://www.w3.org/TR/css-grid-2/#typedef-flex
#[derive(IntoCursor, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Dimension)]
pub struct Flex(#[atom(CssAtomSet::Fr)] T![Dimension]);

impl ToNumberValue for Flex {
	fn to_number_value(&self) -> Option<f32> {
		Some(self.0.into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Flex>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Flex, "1fr");
	}
}
