use super::prelude::*;

/// <https://drafts.csswg.org/css-content-3/#quote-values>
///
/// ```text,ignore
/// <quote> = open-quote | close-quote | no-open-quote | no-close-quote
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum Quote {
	#[atom(CssAtomSet::OpenQuote)]
	OpenQuote(T![Ident]),
	#[atom(CssAtomSet::CloseQuote)]
	CloseQuote(T![Ident]),
	#[atom(CssAtomSet::NoOpenQuote)]
	NoOpenQuote(T![Ident]),
	#[atom(CssAtomSet::NoCloseQuote)]
	NoCloseQuote(T![Ident]),
}
