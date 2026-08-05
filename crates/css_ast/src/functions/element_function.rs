use super::prelude::*;
use crate::Id;

/// <https://drafts.csswg.org/css-images-4/#funcdef-element>
///
/// ```text,ignore
/// <element()> = element( <id-selector> )
/// ```
#[node]
#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(all))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ElementFunction {
	#[atom(CssAtomSet::Element)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub id: Id,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ElementFunction, "element(#foo)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, ElementFunction, "element()");
		assert_parse_error!(CssAtomSet::ATOMS, ElementFunction, "element(foo)");
		assert_parse_error!(CssAtomSet::ATOMS, ElementFunction, "element(.foo)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("element(#foo)", ElementFunction, Id);
	}
}
