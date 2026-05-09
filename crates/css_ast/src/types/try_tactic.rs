use super::prelude::*;

/// <https://drafts.csswg.org/css-anchor-position-1/#typedef-position-try-fallbacks-try-tactic>
///
/// ```text,ignore
/// <try-tactic> = flip-block || flip-inline || flip-start || flip-x || flip-y
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[parse(one_must_occur)]
pub struct TryTactic {
	#[atom(CssAtomSet::FlipBlock)]
	pub flip_block: Option<T![Ident]>,
	#[atom(CssAtomSet::FlipInline)]
	pub flip_inline: Option<T![Ident]>,
	#[atom(CssAtomSet::FlipStart)]
	pub flip_start: Option<T![Ident]>,
	#[atom(CssAtomSet::FlipX)]
	pub flip_x: Option<T![Ident]>,
	#[atom(CssAtomSet::FlipY)]
	pub flip_y: Option<T![Ident]>,
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<TryTactic>(), 80);
	}

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, TryTactic, "flip-block");
		assert_parse!(CssAtomSet::ATOMS, TryTactic, "flip-inline");
		assert_parse!(CssAtomSet::ATOMS, TryTactic, "flip-x");
		assert_parse!(CssAtomSet::ATOMS, TryTactic, "flip-y");
		assert_parse!(CssAtomSet::ATOMS, TryTactic, "flip-block flip-inline");
		assert_parse!(CssAtomSet::ATOMS, TryTactic, "flip-x flip-y flip-start");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, TryTactic, "");
		assert_parse_error!(CssAtomSet::ATOMS, TryTactic, "auto");
		assert_parse_error!(CssAtomSet::ATOMS, TryTactic, "none");
	}
}
