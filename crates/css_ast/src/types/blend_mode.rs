use super::prelude::*;

/// <https://drafts.csswg.org/css2/#value-def-absolute-size>
///
/// ```text,ignore
/// <blend-mode> = darken | multiply | color-burn | lighten | screen | color-dodge | overlay | soft-light | hard-light | difference | exclusion | hue | saturation | color | luminosity
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum BlendMode {
	#[atom(CssAtomSet::Normal)]
	Normal(T![Ident]),
	#[atom(CssAtomSet::Darken)]
	Darken(T![Ident]),
	#[atom(CssAtomSet::Multiply)]
	Multiply(T![Ident]),
	#[atom(CssAtomSet::ColorBurn)]
	ColorBurn(T![Ident]),
	#[atom(CssAtomSet::Lighten)]
	Lighten(T![Ident]),
	#[atom(CssAtomSet::Screen)]
	Screen(T![Ident]),
	#[atom(CssAtomSet::ColorDodge)]
	ColorDodge(T![Ident]),
	#[atom(CssAtomSet::Overlay)]
	Overlay(T![Ident]),
	#[atom(CssAtomSet::SoftLift)]
	SoftLift(T![Ident]),
	#[atom(CssAtomSet::HardLight)]
	HardLight(T![Ident]),
	#[atom(CssAtomSet::Difference)]
	Difference(T![Ident]),
	#[atom(CssAtomSet::Exclusion)]
	Exclusion(T![Ident]),
	#[atom(CssAtomSet::Hue)]
	Hue(T![Ident]),
}
