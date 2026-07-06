use super::prelude::*;
use crate::{AutoOr, Length};

/// <https://drafts.csswg.org/css-masking-1/#funcdef-rect>
///
/// ```text
/// rect() = rect( <top>, <right>, <bottom>, <left> )
/// <top>, <right>, <bottom>, <left> = <length> | auto
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(all))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RectFunction {
	#[atom(CssAtomSet::Rect)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub top: AutoOr<Length>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma1: Option<T![,]>,
	pub right: AutoOr<Length>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma2: Option<T![,]>,
	pub bottom: AutoOr<Length>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma3: Option<T![,]>,
	pub left: AutoOr<Length>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<RectFunction>(), 136);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, RectFunction, "rect(0 0 0 0)");
		assert_parse!(CssAtomSet::ATOMS, RectFunction, "rect(10px,20px,30px,40px)");
		assert_parse!(CssAtomSet::ATOMS, RectFunction, "rect(auto,auto,auto,auto)");
		assert_parse!(CssAtomSet::ATOMS, RectFunction, "rect(10px,auto,30px,auto)");
		assert_parse!(CssAtomSet::ATOMS, RectFunction, "rect(-10px,20px,-5px,0px)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, RectFunction, "rect(10px)");
		assert_parse_error!(CssAtomSet::ATOMS, RectFunction, "rect(10px,20px)");
		assert_parse_error!(CssAtomSet::ATOMS, RectFunction, "rect(10%,20%,30%,40%)");
		assert_parse_error!(CssAtomSet::ATOMS, RectFunction, "rect()");
	}
}
