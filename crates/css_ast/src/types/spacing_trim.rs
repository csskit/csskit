use super::prelude::*;

/// <https://drafts.csswg.org/css-text-4/#typedef-spacing-trim>
///
/// ```text,ignore
/// <spacing-trim> = space-all | normal | space-first | trim-start | trim-both | trim-all
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
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
