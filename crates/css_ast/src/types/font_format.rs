use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#font-format-values>
///
/// ```text,ignore
/// <font-format> = [ <string> | collection | embedded-opentype | opentype | svg | truetype | woff | woff2 ]
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum FontFormat {
	#[atom(CssAtomSet::Collection)]
	Collection(T![Ident]),
	#[atom(CssAtomSet::EmbeddedOpentype)]
	EmbeddedOpentype(T![Ident]),
	#[atom(CssAtomSet::Opentype)]
	Opentype(T![Ident]),
	#[atom(CssAtomSet::Svg)]
	Svg(T![Ident]),
	#[atom(CssAtomSet::Truetype)]
	Truetype(T![Ident]),
	#[atom(CssAtomSet::Woff)]
	Woff(T![Ident]),
	#[atom(CssAtomSet::Woff2)]
	Woff2(T![Ident]),
	String(T![String]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FontFormat, "collection");
		assert_parse!(CssAtomSet::ATOMS, FontFormat, "embedded-opentype");
		assert_parse!(CssAtomSet::ATOMS, FontFormat, "opentype");
		assert_parse!(CssAtomSet::ATOMS, FontFormat, "woff2");
		assert_parse!(CssAtomSet::ATOMS, FontFormat, "\"woff2-variations\"");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, FontFormat, "1px");
	}
}
