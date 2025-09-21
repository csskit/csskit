use super::prelude::*;

/// <https://drafts.csswg.org/css-gaps-1/#typedef-line-width-list>
///
/// ```text,ignore
/// <line-width-list> = [ <line-width-or-repeat> ]+
/// ```
#[syntax(" [ <line-width-or-repeat> ]+ ")]
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct LineWidthList<'a>;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LineWidthList>(), 32);
	}
}
