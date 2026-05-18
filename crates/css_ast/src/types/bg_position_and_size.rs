use super::prelude::*;
use crate::{BgPosition, BgSize};

/// Represents the `<bg-position> [ / <bg-size> ]?` component of `<bg-layer>`.
///
/// ```text,ignore
/// <bg-position> [ / <bg-size> ]?
/// ```
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct BgPositionAndSize {
	pub position: BgPosition,
	pub size: Option<(T![/], BgSize)>,
}

impl<'a> Peek<'a> for BgPositionAndSize {
	const PEEK_KINDSET: KindSet = BgPosition::PEEK_KINDSET;

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		BgPosition::peek(p, c)
	}
}

impl<'a> Parse<'a> for BgPositionAndSize {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let position = p.parse::<BgPosition>()?;
		let size = if p.peek::<T![/]>() {
			let slash = p.parse::<T![/]>()?;
			let size = p.parse::<BgSize>()?;
			Some((slash, size))
		} else {
			None
		};
		Ok(Self { position, size })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<BgPositionAndSize>(), 108);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BgPositionAndSize, "center");
		assert_parse!(CssAtomSet::ATOMS, BgPositionAndSize, "50% 50%");
		assert_parse!(CssAtomSet::ATOMS, BgPositionAndSize, "center / cover");
		assert_parse!(CssAtomSet::ATOMS, BgPositionAndSize, "0 0 / auto");
		assert_parse!(CssAtomSet::ATOMS, BgPositionAndSize, "left top / 100px 200px");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, BgPositionAndSize, "");
	}
}
