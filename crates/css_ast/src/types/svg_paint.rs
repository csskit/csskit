use super::prelude::*;
use crate::{Integer, Positive};

/// <https://drafts.csswg.org/fill-stroke-3/#typedef-svg-paint>
///
/// ```text,ignore
/// <svg-paint> = child | child( <integer> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SvgPaint {
	#[atom(CssAtomSet::Child)]
	Child(T![Ident]),
	ChildFunction(SvgPaintChildFunction),
}

/// `child( <integer> )` — refers to the nth child paint server element
/// (1-indexed). Arguments less than 1 are invalid.
///
/// <https://drafts.csswg.org/fill-stroke-3/#typedef-svg-paint>
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SvgPaintChildFunction {
	#[atom(CssAtomSet::Child)]
	pub function: T![Function],
	pub index: Positive<Integer>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SvgPaint>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SvgPaint, "child");
		assert_parse!(CssAtomSet::ATOMS, SvgPaint, "child(1)");
		assert_parse!(CssAtomSet::ATOMS, SvgPaint, "child(3)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, SvgPaint, "");
		assert_peek_false!(CssAtomSet::ATOMS, SvgPaint, "parent");
		assert_parse_error!(CssAtomSet::ATOMS, SvgPaint, "child(0)");
		assert_parse_error!(CssAtomSet::ATOMS, SvgPaint, "child(-1)");
	}
}
