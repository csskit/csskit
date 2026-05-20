use super::prelude::*;
use crate::FadeFunction;

/// <https://drafts.csswg.org/css-overflow-4/#text-overflow>
///
/// ```text,ignore
/// [ clip | ellipsis | <string> | fade | <fade()> ]
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum TextOverflowSingleAxis {
	#[atom(CssAtomSet::Clip)]
	Clip(T![Ident]),
	#[atom(CssAtomSet::Ellipsis)]
	Ellipsis(T![Ident]),
	#[atom(CssAtomSet::Fade)]
	Fade(T![Ident]),
	String(T![String]),
	#[atom(CssAtomSet::Fade)]
	FadeFunction(FadeFunction),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TextOverflowSingleAxis>(), 16);
	}

	#[test]
	fn test_parse() {
		assert_parse!(CssAtomSet::ATOMS, TextOverflowSingleAxis, "clip");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowSingleAxis, "ellipsis");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowSingleAxis, "fade");
		assert_parse!(CssAtomSet::ATOMS, TextOverflowSingleAxis, "'foo'");
		// assert_parse!(CssAtomSet::ATOMS, TextOverflowSingleAxis, "fade()");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TextOverflowSingleAxis, "");
		assert_parse_error!(CssAtomSet::ATOMS, TextOverflowSingleAxis, "clip clip");
	}
}
