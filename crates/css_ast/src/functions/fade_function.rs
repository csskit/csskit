use super::prelude::*;
use crate::{CalcableValue, LengthPercentage};

/// <https://drafts.csswg.org/css-overflow-4/#funcdef-text-overflow-fade>
///
/// ```text
/// <fade()> = fade( <length-percentage> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FadeFunction<'a> {
	#[atom(CssAtomSet::Fade)]
	pub name: T![Function],
	pub params: CalcableValue<'a, LengthPercentage>,
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
		assert_parse!(CssAtomSet::ATOMS, FadeFunction, "fade(1px)");
		assert_parse!(CssAtomSet::ATOMS, FadeFunction, "fade(10%)");
		assert_parse!(CssAtomSet::ATOMS, FadeFunction, "fade(var(--x))");
		assert_parse!(CssAtomSet::ATOMS, FadeFunction, "fade(calc(100% - 20px))");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FadeFunction, "fade()");
		assert_parse_error!(CssAtomSet::ATOMS, FadeFunction, "fade(red)");
	}
}
