use super::prelude::*;
use crate::{Image, SvgPaint};

/// <https://drafts.csswg.org/fill-stroke-3/#typedef-paint>
///
/// ```text,ignore
/// <paint> = none | <image> | <svg-paint>
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Paint<'a> {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	Image(Image<'a>),
	SvgPaint(SvgPaint<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Paint, "none");
		assert_parse!(CssAtomSet::ATOMS, Paint, "url(foo.svg)");
		assert_parse!(CssAtomSet::ATOMS, Paint, "linear-gradient(red, blue)");
		assert_parse!(CssAtomSet::ATOMS, Paint, "child");
		assert_parse!(CssAtomSet::ATOMS, Paint, "child(2)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, Paint, "");
		// `<paint>` doesn't include `<color>` directly (that's `fill-color`'s job).
		assert_peek_false!(CssAtomSet::ATOMS, Paint, "red");
	}
}
