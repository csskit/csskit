use super::prelude::*;
use css_parse::ComponentValues;

/// <https://drafts.csswg.org/css-link-params-1/#funcdef-param>
///
/// ```text,ignore
/// <param()> = param( <dashed-ident> , <declaration-value>? )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ParamFunction<'a> {
	#[atom(CssAtomSet::Param)]
	pub name: T![Function],
	pub params: ParamFunctionParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ParamFunctionParams<'a> {
	pub ident: T![DashedIdent],
	#[semantic_eq(skip)]
	pub comma: T![,],
	pub value: Option<ComponentValues<'a>>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ParamFunction, "param(--foo,12px)");
		assert_parse!(CssAtomSet::ATOMS, ParamFunction, "param(--foo,var(--bar))");
	}
}
