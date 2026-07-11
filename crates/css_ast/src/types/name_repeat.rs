use super::prelude::*;
use crate::{LineNames, NonEmpty};

/// <https://drafts.csswg.org/css-grid-2/#typedef-name-repeat>
///
/// ```text,ignore
/// <name-repeat> = repeat( [ <integer [1,∞]> | auto-fill ] , <line-names>+ )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NameRepeat<'a> {
	#[atom(CssAtomSet::Repeat)]
	pub name: T![Function],
	pub params: NameRepeatParams<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// `<integer [1,∞]> | auto-fill`
#[syntax(" <integer [1,∞]> | auto-fill ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum NameRepeatCount {}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NameRepeatParams<'a> {
	pub count: NameRepeatCount,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma: T![,],
	pub names: NonEmpty<Vec<'a, LineNames<'a>>>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NameRepeat>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, NameRepeat, "repeat(2,[a])");
		assert_parse!(CssAtomSet::ATOMS, NameRepeat, "repeat(auto-fill,[a] [b])");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, NameRepeat, "repeat(2,)");
		assert_parse_error!(CssAtomSet::ATOMS, NameRepeat, "repeat(2,10px)");
	}
}
