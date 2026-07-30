use super::prelude::*;
use crate::{CalcableValue, OpentypeTag};

/// Value for `font-variation-settings`: `<opentype-tag> <number>`
///
/// ```text,ignore
/// <opentype-tag> <number>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct VariationTagValue<'a>(pub OpentypeTag, pub CalcableValue<'a, T![Number]>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "\"wght\" 700");
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "\"wdth\" 100");
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "\"slnt\" -12");
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "'ital' 1");
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "\"opsz\" 48");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "\"wght\" var(--w)");
		assert_parse!(CssAtomSet::ATOMS, VariationTagValue, "\"wght\" calc(400 + 300)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, VariationTagValue, "\"wght\"");
		assert_parse_error!(CssAtomSet::ATOMS, VariationTagValue, "\"wg\" 700");
		assert_peek_false!(CssAtomSet::ATOMS, VariationTagValue, "wght 700");
	}
}
