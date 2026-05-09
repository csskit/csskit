use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#discretionary-lig-values>
///
/// ```text,ignore
/// <discretionary-lig-values> = [ discretionary-ligatures | no-discretionary-ligatures ]
/// ```
#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum DiscretionaryLigValues {
	#[atom(CssAtomSet::DiscretionaryLigatures)]
	DiscretionaryLigatures(T![Ident]),
	#[atom(CssAtomSet::NoDiscretionaryLigatures)]
	NoDiscretionaryLigatures(T![Ident]),
}
