use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-5/#system-font-family-name>
///
/// ```text,ignore
/// <system-font-family-name> = caption | icon | menu | message-box | small-caption | status-bar
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum SystemFontFamilyName {
	#[atom(CssAtomSet::Caption)]
	Caption(T![Ident]),
	#[atom(CssAtomSet::Icon)]
	Icon(T![Ident]),
	#[atom(CssAtomSet::Menu)]
	Menu(T![Ident]),
	#[atom(CssAtomSet::MessageBox)]
	MessageBox(T![Ident]),
	#[atom(CssAtomSet::SmallCaption)]
	SmallCaption(T![Ident]),
	#[atom(CssAtomSet::StatusBar)]
	StatusBar(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SystemFontFamilyName, "caption");
		assert_parse!(CssAtomSet::ATOMS, SystemFontFamilyName, "message-box");
		assert_parse!(CssAtomSet::ATOMS, SystemFontFamilyName, "status-bar");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, SystemFontFamilyName, "serif");
	}
}
