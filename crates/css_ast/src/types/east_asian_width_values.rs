use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#east-asian-width-values>
///
/// ```text,ignore
/// <east-asian-width-values> = [ full-width | proportional-width ]
/// ```
#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum EastAsianWidthValues {
	#[atom(CssAtomSet::FullWidth)]
	FullWidth(T![Ident]),
	#[atom(CssAtomSet::ProportionalWidth)]
	ProportionalWidth(T![Ident]),
}
