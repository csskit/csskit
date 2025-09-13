use super::prelude::*;

/// <https://drafts.csswg.org/css-animations-2/#typedef-single-animation-timeline>
///
/// ```text,ignore
/// <single-animation-timeline> = auto | none | <dashed-ident> | <scroll()> | <view()>
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum SingleAnimationTimeline {
	#[atom(CssAtomSet::Auto)]
	Auto(T![Ident]),
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
}
