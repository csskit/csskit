use bumpalo::collections::Vec;
use css_parse::T;
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

/// <https://drafts.csswg.org/css-fonts-4/#family-name-syntax>
///
/// ```text,ignore
/// <family-name> = <string> | <custom-ident>+
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit]
pub enum FamilyName<'a> {
	String(T![String]),
	#[visit(skip)]
	CustomIdents(Vec<'a, T![Ident]>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FamilyName>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FamilyName, "New Century Schoolbook");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(FamilyName, "'foo' bar");
	}
}
