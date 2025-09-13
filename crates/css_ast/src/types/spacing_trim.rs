use super::prelude::*;

/// <https://drafts.csswg.org/css-text-4/#typedef-spacing-trim>
///
/// ```text,ignore
/// <spacing-trim> = space-all | normal | space-first | trim-start | trim-both | trim-all
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum SpacingTrim {
	#[atom(CssAtomSet::SpaceAll)]
	SpaceAll(T![Ident]),
	#[atom(CssAtomSet::Normal)]
	Normal(T![Ident]),
	#[atom(CssAtomSet::SpaceFirst)]
	SpaceFirst(T![Ident]),
	#[atom(CssAtomSet::TrimStart)]
	TrimStart(T![Ident]),
	#[atom(CssAtomSet::TrimBoth)]
	TrimBoth(T![Ident]),
	#[atom(CssAtomSet::TrimAll)]
	TrimAll(T![Ident]),
}
