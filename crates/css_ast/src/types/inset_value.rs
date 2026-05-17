use super::prelude::*;
use crate::LengthPercentage;

/// <https://drafts.csswg.org/css-gaps-1/#typedef-inset-value>
///
/// ```text,ignore
/// <inset-value> = <length-percentage> | overlap-join
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum InsetValue {
	LengthPercentage(LengthPercentage),
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::OverlapJoin)]
	OverlapJoin(T![Ident]),
}
