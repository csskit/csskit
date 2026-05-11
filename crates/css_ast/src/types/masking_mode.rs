use super::prelude::*;

/// <https://drafts.csswg.org/css-masking-1/#typedef-masking-mode>
///
/// ```text,ignore
/// <masking-mode> = alpha | luminance | match-source
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum MaskingMode {
	#[atom(CssAtomSet::Alpha)]
	Alpha(T![Ident]),
	#[atom(CssAtomSet::Luminance)]
	Luminance(T![Ident]),
	#[atom(CssAtomSet::MatchSource)]
	MatchSource(T![Ident]),
}
