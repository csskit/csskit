use crate::ExpectedTypes;
use css_parse::{ComponentValues, Cursor, Parse, Parser, Result};
use csskit_derives::*;

/// ComponentValues, but with additional type information.
///
/// Produced when a substitution function (e.g. `var()`, `env()`) appears at a position where:
/// - The full grammar type is known but the literal could not be parsed (`Value<T>::Unresolved`).
/// - The entire value is a single substitution (`StyleValue`-level escape).
///
/// `tokens` preserves the original source tokens verbatim.
/// `expected` records the set of CSS value types the position accepts, enabling
/// downstream inference and diagnostics.
#[derive(Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Unresolved<'a> {
	pub tokens: ComponentValues<'a>,
	#[peek(skip)]
	#[semantic_eq(skip)]
	pub expected: ExpectedTypes,
}

impl<'a> Parse<'a> for Unresolved<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let tokens = p.parse::<ComponentValues>()?;
		Ok(Self { tokens, expected: ExpectedTypes::ANY })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Unresolved>(), 32);
	}

	#[test]
	fn test_expected_types_any() {
		assert_eq!(ExpectedTypes::ANY.bits(), !0u32);
	}

	#[test]
	fn test_expected_types_compose() {
		let lp = ExpectedTypes::Length | ExpectedTypes::Percentage;
		assert!(lp.contains(ExpectedTypes::Length));
		assert!(lp.contains(ExpectedTypes::Percentage));
		assert!(!lp.contains(ExpectedTypes::Color));
	}

	#[test]
	fn test_parse_roundtrip() {
		assert_parse!(CssAtomSet::ATOMS, Unresolved, "var(--x)");
		assert_parse!(CssAtomSet::ATOMS, Unresolved, "1px solid red");
	}
}
