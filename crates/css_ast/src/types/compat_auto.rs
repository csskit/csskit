use super::prelude::*;

/// <https://drafts.csswg.org/css-ui-4/#typedef-appearance-compat-auto>
///
/// These values exist for compatibility of content developed for earlier non-standard versions of this property.
/// They all have the same effect as auto.
///
/// ```text,ignore
/// <compat-auto> = searchfield | textarea | checkbox | radio | menulist | listbox | meter | progress-bar | button
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum CompatAuto {
	#[atom(CssAtomSet::Search)]
	Searchfield(T![Ident]),
	#[atom(CssAtomSet::Textarea)]
	Textarea(T![Ident]),
	#[atom(CssAtomSet::Checkbox)]
	Checkbox(T![Ident]),
	#[atom(CssAtomSet::Radio)]
	Radio(T![Ident]),
	#[atom(CssAtomSet::Menulist)]
	Menulist(T![Ident]),
	#[atom(CssAtomSet::Listbox)]
	Listbox(T![Ident]),
	#[atom(CssAtomSet::Meter)]
	Meter(T![Ident]),
	#[atom(CssAtomSet::ProgressBar)]
	ProgressBar(T![Ident]),
	#[atom(CssAtomSet::Button)]
	Button(T![Ident]),
}
