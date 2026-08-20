use super::prelude::*;
use crate::{CSSInt, Positive};

#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct PositiveNonZeroInt(pub Positive<CSSInt>);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, PositiveNonZeroInt, "1");
		assert_parse!(CssAtomSet::ATOMS, PositiveNonZeroInt, "100");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, PositiveNonZeroInt, "0");
		assert_peek_false!(CssAtomSet::ATOMS, PositiveNonZeroInt, "0.0");
		assert_parse_error!(CssAtomSet::ATOMS, PositiveNonZeroInt, "-1");
		assert_peek_false!(CssAtomSet::ATOMS, PositiveNonZeroInt, "1.2");
		assert_peek_false!(CssAtomSet::ATOMS, PositiveNonZeroInt, "-1.2");
	}
}
