use crate::CssTypes;
use css_parse::{ComponentValues, Cursor, Parse, Parser, Result};
use csskit_derives::*;
use csskit_proc_macro::node;

/// ComponentValues, but with additional type information.
///
/// Produced when a substitution function (e.g. `var()`, `env()`) appears at a position where:
/// - The full grammar type is known but the literal could not be parsed (`Value<T>::Unresolved`).
/// - The entire value is a single substitution (`StyleValue`-level escape).
///
/// `tokens` preserves the original source tokens verbatim.
/// `expected` records the set of CSS value types the position accepts, enabling
/// downstream inference and diagnostics.
#[node]
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Unresolved<'a> {
	pub tokens: ComponentValues<'a>,
	#[peek(skip)]
	#[semantic_eq(skip)]
	#[metadata(skip)]
	pub expected: CssTypes,
}

impl<'a> Parse<'a> for Unresolved<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let tokens = p.parse::<ComponentValues>()?;
		Ok(Self { tokens, expected: CssTypes::ANY })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_expected_types_any() {
		assert_eq!(CssTypes::ANY.bits(), !0u32);
	}

	#[test]
	fn test_expected_types_compose() {
		let lp = CssTypes::Length | CssTypes::Percentage;
		assert!(lp.contains(CssTypes::Length));
		assert!(lp.contains(CssTypes::Percentage));
		assert!(!lp.contains(CssTypes::Color));
	}

	#[test]
	fn test_parse_roundtrip() {
		assert_parse!(CssAtomSet::ATOMS, Unresolved, "var(--x)");
		assert_parse!(CssAtomSet::ATOMS, Unresolved, "1px solid red");
	}
}
