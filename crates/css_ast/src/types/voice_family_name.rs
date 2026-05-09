use super::prelude::*;

/// <https://drafts.csswg.org/css-speech-1/#typedef-voice-family-voice-family-name>
///
/// ```text,ignore
/// <voice-family-name> = <string> | <custom-ident>+
/// ```
#[syntax(" <string> | <custom-ident>+ ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum VoiceFamilyName<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<VoiceFamilyName>(), 32);
	}

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyName, "\"Alice\"");
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyName, "Alice");
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyName, "Deep Voice");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, VoiceFamilyName, "");
		assert_parse_error!(CssAtomSet::ATOMS, VoiceFamilyName, "\"foo\" bar");
	}
}
