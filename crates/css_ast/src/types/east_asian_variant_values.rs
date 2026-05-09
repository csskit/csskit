use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#east-asian-variant-values>
///
/// ```text,ignore
/// <east-asian-variant-values> = [ jis78 | jis83 | jis90 | jis04 | simplified | traditional ]
/// ```
#[derive(Parse, Peek, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum EastAsianVariantValues {
	#[atom(CssAtomSet::Jis78)]
	Jis78(T![Ident]),
	#[atom(CssAtomSet::Jis83)]
	Jis83(T![Ident]),
	#[atom(CssAtomSet::Jis90)]
	Jis90(T![Ident]),
	#[atom(CssAtomSet::Jis04)]
	Jis04(T![Ident]),
	#[atom(CssAtomSet::Simplified)]
	Simplified(T![Ident]),
	#[atom(CssAtomSet::Traditional)]
	Traditional(T![Ident]),
}
