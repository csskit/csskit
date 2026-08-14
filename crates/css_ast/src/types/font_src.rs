use super::prelude::*;
use crate::{FormatFunction, LocalFunction, TechFunction, Url};
use css_parse::CommaSeparated;

/// <https://drafts.csswg.org/css-fonts-4/#src-desc>
///
/// ```text,ignore
/// <font-src-list> = <font-src>#
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FontSrcList<'a>(pub CommaSeparated<'a, FontSrc<'a>>);

/// <https://drafts.csswg.org/css-fonts-4/#src-desc>
///
/// ```text,ignore
/// <font-src> = <url> [ format( <font-format> ) ]? [ tech( <font-tech># ) ]? | local( <font-family-name> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FontSrc<'a> {
	Local(LocalFunction<'a>),
	Url(Url, Option<FormatFunction<'a>>, Option<TechFunction<'a>>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FontSrc, "url(font.woff2)");
		assert_parse!(CssAtomSet::ATOMS, FontSrc, "url(\"font.woff2\")format(woff2)");
		assert_parse!(CssAtomSet::ATOMS, FontSrc, "url(font.woff2)format(woff2)tech(variations)");
		assert_parse!(CssAtomSet::ATOMS, FontSrc, "local(Gentium)");
		assert_parse!(CssAtomSet::ATOMS, FontSrcList, "local(Gentium),url(font.woff2)format(woff2)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FontSrc, "Gentium");
	}
}
