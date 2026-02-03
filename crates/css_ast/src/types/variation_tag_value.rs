use super::prelude::*;
use crate::OpentypeTag;

/// Value for `font-variation-settings`: `<opentype-tag> <number>`
///
/// ```text,ignore
/// <opentype-tag> <number>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct VariationTagValue(pub OpentypeTag, pub T![Number]);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<VariationTagValue>(), 24);
	}

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "\"wght\" 700");
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "\"wdth\" 100");
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "\"slnt\" -12");
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "'ital' 1");
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "\"opsz\" 48");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, VariationTagValue, "\"wght\"");
		assert_parse_error!(CssAtomSet::ATOMS, VariationTagValue, "\"wg\" 700");
		assert_parse_error!(CssAtomSet::ATOMS, VariationTagValue, "wght 700");
	}
}
