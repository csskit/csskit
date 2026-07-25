use super::prelude::*;
use crate::{LineNames, NonEmpty, PositiveNonZeroInt};

/// ```text,ignore
/// <track-repeat>           = repeat( [ <integer [1,∞]> ] , [ <line-names>? <track-size> ]+ <line-names>? )
/// <fixed-repeat>           = repeat( [ <integer [1,∞]> ] , [ <line-names>? <fixed-size> ]+ <line-names>? )
/// <auto-repeat>            = repeat( [ auto-fill | auto-fit ] , [ <line-names>? <fixed-size> ]+ <line-names>? )
/// <repeat-line-width>      = repeat( [ <integer [1,∞]> ] , [ <line-width> ]+ )
/// <auto-repeat-line-width> = repeat( auto , [ <line-width> ]+ )
/// ```
#[derive(Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct RepeatFunction<Items, Count = PositiveNonZeroInt> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Repeat)]
	pub name: css_parse::token_macros::Function,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub count: Count,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub comma: css_parse::token_macros::Comma,
	pub items: Items,
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[semantic_eq(skip)]
	pub close: css_parse::token_macros::RightParen,
}

impl<'a, Items, Count: Peek<'a>> Peek<'a> for RepeatFunction<Items, Count> {
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Function]);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<css_parse::token_macros::Function>::peek(p, c)
			&& p.equals_atom(c, &CssAtomSet::Repeat)
			&& Count::peek(p, p.peek_n(2))
	}
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct NamedRepeatItems<'a, Item> {
	pub leading_names: Option<LineNames<'a>>,
	pub items: NonEmpty<Vec<'a, (Item, Option<LineNames<'a>>)>>,
}

#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum AutoFillOrFit {
	#[atom(CssAtomSet::AutoFill)]
	AutoFill(T![Ident]),
	#[atom(CssAtomSet::AutoFit)]
	AutoFit(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{CssAtomSet, FixedSize};
	use css_parse::{assert_parse, assert_peek_false};

	type FixedRepeat<'a> = RepeatFunction<NamedRepeatItems<'a, FixedSize<'a>>>;
	type AutoRepeat<'a> = RepeatFunction<NamedRepeatItems<'a, FixedSize<'a>>, AutoFillOrFit>;

	#[test]
	fn test_count_disambiguates_same_named_function() {
		assert_parse!(CssAtomSet::ATOMS, FixedRepeat, "repeat(2,10px)");
		assert_peek_false!(CssAtomSet::ATOMS, FixedRepeat, "repeat(auto-fill,10px)");
		assert_parse!(CssAtomSet::ATOMS, AutoRepeat, "repeat(auto-fill,10px)");
		assert_peek_false!(CssAtomSet::ATOMS, AutoRepeat, "repeat(2,10px)");
	}
}
