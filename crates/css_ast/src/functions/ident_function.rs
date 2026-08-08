use super::prelude::*;
use crate::{NumericValue, Value};

/// <https://drafts.csswg.org/css-values-5/#ident>
///
/// The `ident()` function constructs a `<custom-ident>` from multiple parts.
///
/// ```text,ignore
/// <ident()> = ident( <ident-arg>+ )
/// <ident-arg> = <string> | <integer> | <ident>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct IdentFunction<'a> {
	#[atom(CssAtomSet::Ident)]
	pub name: T![Function],
	pub args: Vec<'a, IdentArg<'a>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// A single `<ident-arg>` inside an [`IdentFunction`].
///
/// The `<integer>` slot is wrapped in [`NumericValue`] so it also admits the tree-counting
/// functions (`sibling-index()`/`sibling-count()`) and math functions, and the `<ident>` slot in
/// [`Value`] so it admits arbitrary substitution functions, matching the spec examples such as
/// `ident("vtl-" sibling-index())` and `ident(var(--id) "-title")`.
///
/// ```text,ignore
/// <ident-arg> = <string> | <integer> | <ident>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum IdentArg<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	String(T![String]),
	Integer(NumericValue<'a, CSSInt>),
	Keyword(Value<'a, T![Ident]>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_ident_function() {
		assert_parse!(CssAtomSet::ATOMS, IdentFunction, "ident(foo)");
		assert_parse!(CssAtomSet::ATOMS, IdentFunction, "ident('vtl-'sibling-index())");
		assert_parse!(CssAtomSet::ATOMS, IdentFunction, "ident(var(--id))");
		assert_parse!(CssAtomSet::ATOMS, IdentFunction, "ident(var(--id)'-title')");
	}
}
