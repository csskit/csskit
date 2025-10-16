use super::prelude::*;

/// <https://drafts.csswg.org/css-values-5/#calc-size>
///
/// ```text,ignore
/// <calc-size()> = calc-size( <calc-size-basis>, <calc-sum> )
/// <calc-size-basis> = [ <size-keyword> | <calc-size()> | any | <calc-sum> ]
/// ```
///
/// The `<size-keyword>` production matches any sizing keywords allowed in the context.
/// For example, in width, it matches auto, min-content, stretch, etc.
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub struct CalcSizeFunction {
	#[atom(CssAtomSet::CalcSize)]
	pub name: T![Function],
	pub params: Todo,
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CalcSizeFunction>(), 24);
	}
}
