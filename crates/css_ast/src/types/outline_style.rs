use super::prelude::*;

/// <https://drafts.csswg.org/css-ui-4/#typedef-outline-line-style>
///
/// `<outline-line-style>` accepts the same values as `<line-style>` (CSS Backgrounds 3 § 3.2 Line Patterns: the
/// border-style properties) with the same meaning, except that hidden is not a legal outline style. In addition, the
/// outline-style property accepts the value auto.
///
/// ```text,ignore
/// <line-style> = none | hidden | dotted | dashed | solid | double | groove | ridge | inset | outset
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum OutlineLineStyle {
	#[atom(CssAtomSet::None)]
	None(T![Ident]),
	#[atom(CssAtomSet::Hidden)]
	Hidden(T![Ident]),
	#[atom(CssAtomSet::Dotted)]
	Dotted(T![Ident]),
	#[atom(CssAtomSet::Dashed)]
	Dashed(T![Ident]),
	#[atom(CssAtomSet::Solid)]
	Solid(T![Ident]),
	#[atom(CssAtomSet::Double)]
	Double(T![Ident]),
	#[atom(CssAtomSet::Groove)]
	Groove(T![Ident]),
	#[atom(CssAtomSet::Ridge)]
	Ridge(T![Ident]),
	#[atom(CssAtomSet::Inset)]
	Inset(T![Ident]),
	#[atom(CssAtomSet::Outset)]
	Outset(T![Ident]),
}
