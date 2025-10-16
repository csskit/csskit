use super::prelude::*;

/// <https://drafts.csswg.org/css-anchor-position-1/#typedef-anchor-name>
///
/// ```text,ignore
/// <anchor-name> = <dashed-ident>
/// ```
#[syntax("<dashed-ident>")]
#[derive(IntoCursor, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct AnchorName;

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<AnchorName>(), 12);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, AnchorName, "--foo");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, AnchorName, "foo");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
		assert_visits!("--foo", AnchorName, DashedIdent);
	}
}
