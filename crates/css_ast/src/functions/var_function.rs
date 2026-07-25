use super::prelude::*;

/// <https://drafts.csswg.org/css-values-5/#var-notation>
///
/// ```text,ignore
/// var() = var( <custom-property-name> , <declaration-value>? )
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(uses_substitution, declaration_kinds = Computed)]
pub struct VarFunction<'a, V> {
	#[semantic_eq(skip)]
	#[atom(CssAtomSet::Var)]
	pub function: Function,
	pub name: DashedIdent,
	#[semantic_eq(skip)]
	pub comma: Option<Comma>,
	pub fallback: Option<Box<'a, V>>,
	#[semantic_eq(skip)]
	pub close: RightParen,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CalcableValue, CssAtomSet, Length, Value};
	use css_parse::assert_parse;

	type VarLength<'a> = VarFunction<'a, Value<'a, Length>>;
	type CalcableVarLength<'a> = VarFunction<'a, CalcableValue<'a, Length>>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<VarLength>(), 72);
		assert_eq!(std::mem::size_of::<CalcableVarLength>(), 72);
	}

	#[test]
	fn test_var_function() {
		assert_parse!(CssAtomSet::ATOMS, VarLength, "var(--x)");
		assert_parse!(CssAtomSet::ATOMS, VarLength, "var(--x, 10px)");
	}

	#[test]
	fn test_calcable_var_function() {
		assert_parse!(CssAtomSet::ATOMS, CalcableVarLength, "var(--x, calc(1px + 2px))");
	}
}
