use super::prelude::*;

/// <https://drafts.csswg.org/css-inline-3/#typedef-baseline-metric>
///
/// ```text,ignore
/// <baseline-metric> = text-bottom | alphabetic | ideographic | middle | central | mathematical | hanging | text-top
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum BaselineMetric {
	#[atom(CssAtomSet::TextBottom)]
	TextBottom(T![Ident]),
	#[atom(CssAtomSet::Alphabetic)]
	Alphabetic(T![Ident]),
	#[atom(CssAtomSet::Ideographic)]
	Ideographic(T![Ident]),
	#[atom(CssAtomSet::Middle)]
	Middle(T![Ident]),
	#[atom(CssAtomSet::Central)]
	Central(T![Ident]),
	#[atom(CssAtomSet::Mathematical)]
	Mathematical(T![Ident]),
	#[atom(CssAtomSet::Hanging)]
	Hanging(T![Ident]),
	#[atom(CssAtomSet::TextTop)]
	TextTop(T![Ident]),
}
