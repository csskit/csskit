use super::prelude::*;

/// <https://drafts.csswg.org/css-color-adjust-1/#typedef-color-scheme-name>
///
/// ```text,ignore
/// <color-scheme-name> = light | dark | <custom-ident>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ColorSchemeName<'a> {
	#[atom(CssAtomSet::Light)]
	Light(T![Ident]),
	#[atom(CssAtomSet::Dark)]
	Dark(T![Ident]),
	Other(crate::Value<'a, crate::CustomIdent>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ColorSchemeName>(), 24);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ColorSchemeName, "light");
		assert_parse!(CssAtomSet::ATOMS, ColorSchemeName, "dark");
		assert_parse!(CssAtomSet::ATOMS, ColorSchemeName, "foo");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, ColorSchemeName, "123");
	}
}
