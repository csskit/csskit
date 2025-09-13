use super::prelude::*;

/// <https://drafts.csswg.org/css-ui-4/#typedef-appearance-compat-special>
///
/// These values exist for compatibility of content developed for earlier non-standard versions of this property.
/// For the purpose of this specification, they all have the same effect as auto.
/// However, the host language may also take these values into account when defining the native appearance of the element.
///
/// ```text,ignore
/// <compat-special> = textfield | menulist-button
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum CompatSpecial {
	#[atom(CssAtomSet::Textfield)]
	Textfield(T![Ident]),
	#[atom(CssAtomSet::MenulistButton)]
	MenulistButton(T![Ident]),
}
