use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#common-lig-values>
///
/// ```text,ignore
/// <common-lig-values> = [ common-ligatures | no-common-ligatures ]
/// ```
#[node]
#[derive(
	Parse, Peek, ToCursors, IntoCursor, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum CommonLigValues {
	#[atom(CssAtomSet::CommonLigatures)]
	CommonLigatures(T![Ident]),
	#[atom(CssAtomSet::NoCommonLigatures)]
	NoCommonLigatures(T![Ident]),
}
