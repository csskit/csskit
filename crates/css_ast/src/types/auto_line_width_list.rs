use super::prelude::*;
use crate::{LineWidthOrRepeat, RepeatLineWidth};

/// <https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-width-list>
///
/// ```text,ignore
/// <auto-line-width-list> = [ <line-width-or-repeat> ]* <auto-repeat-line-width> [ <line-width-or-repeat> ]*
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct AutoLineWidthList<'a> {
	pub start_items: Vec<'a, LineWidthOrRepeat<'a>>,
	pub auto_repeat: RepeatLineWidth<'a>,
	pub end_items: Vec<'a, LineWidthOrRepeat<'a>>,
}

impl<'a> Peek<'a> for AutoLineWidthList<'a> {
	const PEEK_KINDSET: KindSet = LineWidthOrRepeat::PEEK_KINDSET.combine(RepeatLineWidth::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		LineWidthOrRepeat::peek(p, c) || RepeatLineWidth::peek(p, c)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AutoLineWidthList>(), 152);
	}
}
