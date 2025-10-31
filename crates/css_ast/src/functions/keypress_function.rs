use super::prelude::*;

/// <https://drafts.csswg.org/css-grid-2/#funcdef-grid-template-columns-fit-content>
///
/// ```text
/// keypress( <string> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct KeypressFunction {
	#[atom(CssAtomSet::Keypress)]
	pub name: T![Function],
	pub params: T![String],
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<KeypressFunction>(), 36);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, KeypressFunction, "keypress('a')");
	}
}
