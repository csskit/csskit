use super::prelude::*;

/// first-valid() function: returns the first syntactically valid value from a comma list.
///
/// <https://drafts.csswg.org/css-values-5/#first-valid>
///
/// ```text,ignore
/// first-valid() = first-valid( <declaration-value># )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub struct FirstValidFunction<'a, V> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	#[atom(CssAtomSet::FirstValid)]
	pub name: Function,
	pub body: CommaSeparated<'a, V>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: RightParen,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CalcableValue, CssAtomSet, Length, Value};
	use css_parse::assert_parse;

	type FirstValidLength<'a> = FirstValidFunction<'a, Value<'a, Length>>;
	type CalcableFirstValidLength<'a> = FirstValidFunction<'a, CalcableValue<'a, Length>>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FirstValidLength>(), 48);
		assert_eq!(std::mem::size_of::<CalcableFirstValidLength>(), 48);
	}

	#[test]
	fn test_first_valid_function() {
		assert_parse!(CssAtomSet::ATOMS, FirstValidLength, "first-valid(1px, 2em, 3rem)");
		assert_parse!(CssAtomSet::ATOMS, FirstValidLength, "first-valid(var(--x), 10px)");
	}

	#[test]
	fn test_calcable_first_valid_function() {
		assert_parse!(CssAtomSet::ATOMS, CalcableFirstValidLength, "first-valid(calc(1px + 2px), 10px)");
	}
}
