use super::prelude::*;

/// <https://drafts.csswg.org/css-text-4/#typedef-autospace>
///
/// ```text,ignore
/// <autospace> = no-autospace | [ ideograph-alpha || ideograph-numeric || punctuation ] || [ insert | replace ]
/// ```
#[syntax(" no-autospace | [ ideograph-alpha || ideograph-numeric || punctuation ] || [ insert | replace ] ")]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum Autospace<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Autospace, "no-autospace");
		assert_parse!(CssAtomSet::ATOMS, Autospace, "ideograph-alpha");
		assert_parse!(CssAtomSet::ATOMS, Autospace, "ideograph-numeric");
		assert_parse!(CssAtomSet::ATOMS, Autospace, "punctuation");
		assert_parse!(CssAtomSet::ATOMS, Autospace, "ideograph-alpha ideograph-numeric punctuation");
		assert_parse!(CssAtomSet::ATOMS, Autospace, "insert");
		assert_parse!(CssAtomSet::ATOMS, Autospace, "replace");
		assert_parse!(CssAtomSet::ATOMS, Autospace, "ideograph-alpha insert");
		assert_parse!(CssAtomSet::ATOMS, Autospace, "punctuation replace");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, Autospace, "");
		assert_parse_error!(CssAtomSet::ATOMS, Autospace, "no-autospace insert");
		assert_parse_error!(CssAtomSet::ATOMS, Autospace, "insert replace");
		assert_parse_error!(CssAtomSet::ATOMS, Autospace, "ideograph-alpha ideograph-alpha");
	}
}
