use super::prelude::*;
use css_parse::{Function, function_set};

function_set!(pub struct GenericScriptSpecificFunctionName "generic");

/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
///
/// ```text,ignore
/// <generic-family> = <generic-script-specific> | <generic-complete> | <generic-incomplete>
/// <generic-script-specific> = generic(fangsong) | generic(kai) | generic(khmer-mul) |  generic(nastaliq)
/// <generic-complete> = serif | sans-serif | system-ui | cursive | fantasy | math | monospace
/// <generic-incomplete> = ui-serif | ui-sans-serif | ui-monospace | ui-rounded
/// ```
#[syntax(" <generic-script-specific> | <generic-complete> | <generic-incomplete> ")]
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit(self)]
pub enum GenericFamily {}

/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
///
/// ```text,ignore
/// <generic-script-specific> = generic(fangsong) | generic(kai) | generic(khmer-mul) |  generic(nastaliq)
/// ```
#[derive(Peek, Parse, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
pub struct GenericScriptSpecific(Function<GenericScriptSpecificFunctionName, GenericScriptSpecificKeyword>);

keyword_set!(
	/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
	///
	/// ```text,ignore
	/// <generic-script-specific> = generic(fangsong) | generic(kai) | generic(khmer-mul) |  generic(nastaliq)
	/// ```
	pub enum GenericScriptSpecificKeyword {
		Fangsong: "fangsong",
		Kai: "kai",
		KhmerMul: "khmer-mul",
		Nastaliq: "nastaliq",
	}
);

keyword_set!(
	/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
	///
	/// ```text,ignore
	/// <generic-complete> = serif | sans-serif | system-ui | cursive | fantasy | math | monospace
	/// ```
	pub enum GenericComplete {
		Serif: "serif"
		SansSerif: "sans-serif",
		SystemUi: "system-ui",
		Cursive: "cursive",
		Fantasy: "fantasy",
		Math: "math",
		Monospace: "monospace",
	}
);

keyword_set!(
	/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
	///
	/// ```text,ignore
	/// <generic-incomplete> = ui-serif | ui-sans-serif | ui-monospace | ui-rounded
	/// ```
	pub enum GenericIncomplete {
		UiSerif: "ui-serif",
		UiSansSerif: "ui-sans-serif",
		UiMonospace: "ui-monospace",
		UiRounded: "ui-rounded",
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<GenericFamily>(), 44);
	}

	#[test]
	fn test_writes() {
		assert_parse!(GenericFamily, "sans-serif");
		assert_parse!(GenericFamily, "ui-serif");
		assert_parse!(GenericFamily, "generic(fangsong)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(GenericFamily, "");
		assert_parse_error!(GenericFamily, "'foo' bar");
	}
}
