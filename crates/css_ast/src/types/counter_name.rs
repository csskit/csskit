use super::prelude::*;

/// <https://drafts.csswg.org/css-lists-3/#typedef-counter-name>
///
/// ```text,ignore
/// <counter-name> = <custom-ident>
/// ```
#[derive(IntoCursor, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CounterName(T![Ident]);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CounterName>(), 12);
	}

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, CounterName, "my-counter");
		assert_parse!(CssAtomSet::ATOMS, CounterName, "foo");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, CounterName, "");
	}
}
