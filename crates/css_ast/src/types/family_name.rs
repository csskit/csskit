use super::prelude::*;

/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
///
/// ```text,ignore
/// <family-name> = <string> | <custom-ident>+
/// ```
#[syntax(" <string> | <custom-ident>+ ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub enum FamilyName<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FamilyName>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FamilyName, "New Century Schoolbook");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FamilyName, "'foo' bar");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("'foo'", FamilyName);
		assert_visits!("foo bar", FamilyName, CustomIdent, CustomIdent);
	}
}
