use crate::{CssAtomSet, CssMetadata, CssTypes, DeclarationKind, SubstitutionKinds};
use css_parse::{AtomSet, ComponentValue, ComponentValues, Cursor, NodeWithMetadata, Parse, Parser, Result};
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
pub struct Unresolved<'a> {
	pub tokens: ComponentValues<'a>,
	#[peek(skip)]
	#[semantic_eq(skip)]
	pub expected: CssTypes,
	#[peek(skip)]
	#[semantic_eq(skip)]
	pub substitutions: SubstitutionKinds,
}

fn substitutions_in(values: &ComponentValues) -> SubstitutionKinds {
	values.values.iter().fold(SubstitutionKinds::none(), |kinds, value| match value {
		ComponentValue::Function(function) => {
			let cursor: Cursor = function.name.into();
			let own = SubstitutionKinds::try_from(CssAtomSet::from_bits(cursor.token().atom_bits()))
				.unwrap_or(SubstitutionKinds::none());
			kinds | own | substitutions_in(&function.params)
		}
		ComponentValue::SimpleBlock(block) => kinds | substitutions_in(&block.values),
		_ => kinds,
	})
}

impl<'a> Parse<'a> for Unresolved<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let tokens = p.parse::<ComponentValues>()?;
		let substitutions = substitutions_in(&tokens);
		Ok(Self { tokens, expected: CssTypes::ANY, substitutions })
	}
}

impl<'a> NodeWithMetadata<CssMetadata> for Unresolved<'a> {
	fn metadata(&self) -> CssMetadata {
		CssMetadata {
			declaration_kinds: DeclarationKind::Computed,
			substitutions: self.substitutions,
			..Default::default()
		}
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
