use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
///
/// ```text,ignore
/// <generic-family> = <generic-script-specific> | <generic-complete> | <generic-incomplete>
/// <generic-script-specific> = generic(fangsong) | generic(kai) | generic(khmer-mul) |  generic(nastaliq)
/// <generic-complete> = serif | sans-serif | system-ui | cursive | fantasy | math | monospace
/// <generic-incomplete> = ui-serif | ui-sans-serif | ui-monospace | ui-rounded
/// ```
#[syntax(" <generic-script-specific> | <generic-complete> | <generic-incomplete> ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum GenericFamily {}

/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
///
/// ```text,ignore
/// <generic-script-specific> = generic(fangsong) | generic(kai) | generic(khmer-mul) |  generic(nastaliq)
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct GenericScriptSpecific {
	#[atom(CssAtomSet::Generic)]
	pub name: T![Function],
	pub params: GenericScriptSpecificKeyword,
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
///
/// ```text,ignore
/// <generic-script-specific> = generic(fangsong) | generic(kai) | generic(khmer-mul) |  generic(nastaliq)
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum GenericScriptSpecificKeyword {
	#[atom(CssAtomSet::Fangsong)]
	Fangsong(T![Ident]),
	#[atom(CssAtomSet::Kai)]
	Kai(T![Ident]),
	#[atom(CssAtomSet::KhmerMul)]
	KhmerMul(T![Ident]),
	#[atom(CssAtomSet::Nastaliq)]
	Nastaliq(T![Ident]),
}

/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
///
/// ```text,ignore
/// <generic-complete> = serif | sans-serif | system-ui | cursive | fantasy | math | monospace
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum GenericComplete {
	#[atom(CssAtomSet::Serif)]
	Serif(T![Ident]),
	#[atom(CssAtomSet::SansSerif)]
	SansSerif(T![Ident]),
	#[atom(CssAtomSet::SystemUi)]
	SystemUi(T![Ident]),
	#[atom(CssAtomSet::Cursive)]
	Cursive(T![Ident]),
	#[atom(CssAtomSet::Fantasy)]
	Fantasy(T![Ident]),
	#[atom(CssAtomSet::Math)]
	Math(T![Ident]),
	#[atom(CssAtomSet::Monospace)]
	Monospace(T![Ident]),
}

/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
///
/// ```text,ignore
/// <generic-incomplete> = ui-serif | ui-sans-serif | ui-monospace | ui-rounded
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum GenericIncomplete {
	#[atom(CssAtomSet::UiSerif)]
	UiSerif(T![Ident]),
	#[atom(CssAtomSet::UiSansSerif)]
	UiSansSerif(T![Ident]),
	#[atom(CssAtomSet::UiMonospace)]
	UiMonospace(T![Ident]),
	#[atom(CssAtomSet::UiRounded)]
	UiRounded(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<GenericFamily>(), 40);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, GenericFamily, "sans-serif");
		assert_parse!(CssAtomSet::ATOMS, GenericFamily, "ui-serif");
		assert_parse!(CssAtomSet::ATOMS, GenericFamily, "generic(fangsong)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, GenericFamily, "");
		assert_parse_error!(CssAtomSet::ATOMS, GenericFamily, "'foo' bar");
	}
}
