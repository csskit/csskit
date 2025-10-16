use super::prelude::*;

/// <https://drafts.csswg.org/css-gaps-1/#typedef-auto-line-width-list>
///
/// ```text,ignore
/// <auto-line-width-list> = [ <line-width-or-repeat> ]* <auto-repeat-line-width> [ <line-width-or-repeat> ]*
/// ```
#[syntax(" [ <line-width-or-repeat> ]* <repeat()> [ <line-width-or-repeat> ]* ")]
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct AutoLineWidthList<'a>;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AutoLineWidthList>(), 152);
	}
}
