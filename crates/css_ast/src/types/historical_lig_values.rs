use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#historical-lig-values>
///
/// ```text,ignore
/// <historical-lig-values> = [ historical-ligatures | no-historical-ligatures ]
/// ```
#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum HistoricalLigValues {
	#[atom(CssAtomSet::HistoricalLigatures)]
	HistoricalLigatures(T![Ident]),
	#[atom(CssAtomSet::NoHistoricalLigatures)]
	NoHistoricalLigatures(T![Ident]),
}
