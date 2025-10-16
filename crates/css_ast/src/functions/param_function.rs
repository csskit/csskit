use super::prelude::*;
use css_parse::ComponentValues;

/// <https://drafts.csswg.org/css-link-params-1/#funcdef-param>
///
/// ```text,ignore
/// <param()> = param( <dashed-ident> , <declaration-value>? )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct ParamFunction<'a> {
	#[atom(CssAtomSet::Param)]
	pub name: T![Function],
	pub params: ParamFunctionParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct ParamFunctionParams<'a> {
	pub ident: T![DashedIdent],
	pub comma: T![,],
	pub value: Option<ComponentValues<'a>>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ParamFunction>(), 80);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ParamFunction, "param(--foo,12px)");
		assert_parse!(CssAtomSet::ATOMS, ParamFunction, "param(--foo,var(--bar))");
	}
}
