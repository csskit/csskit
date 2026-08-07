use super::prelude::*;

/// <https://drafts.csswg.org/css-values-5/#tree-counting>
///
/// ```text,ignore
/// <sibling-count()> = sibling-count()
/// <sibling-index()> = sibling-index()
/// ```
///
/// Both notations represent an `<integer>` that can only be resolved against a live tree, so they
/// are valid wherever an `<integer>` is, and as a `<calc-value>` inside any math function.
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(declaration_kinds = Computed)]
pub enum TreeCountingFunction {
	SiblingCount(#[atom(CssAtomSet::SiblingCount)] Function, #[semantic_eq(skip)] RightParen),
	SiblingIndex(#[atom(CssAtomSet::SiblingIndex)] Function, #[semantic_eq(skip)] RightParen),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn test_tree_counting_function() {
		assert_parse!(CssAtomSet::ATOMS, TreeCountingFunction, "sibling-count()");
		assert_parse!(CssAtomSet::ATOMS, TreeCountingFunction, "sibling-index()");
	}
}
