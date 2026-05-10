use super::prelude::*;

/// Vendor-prefixed legacy display values.
///
/// These are non-standard values used for backwards compatibility with older
/// browsers' flex implementations.
///
/// ```text,ignore
/// <display-legacy-vendor> = -webkit-box | -webkit-flex | -ms-flexbox | -ms-inline-flexbox
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
pub enum DisplayLegacyVendor {
	#[atom(CssAtomSet::_WebkitBox)]
	WebkitBox(T![Ident]),
	#[atom(CssAtomSet::_WebkitFlex)]
	WebkitFlex(T![Ident]),
	#[atom(CssAtomSet::_MsFlexbox)]
	MsFlexbox(T![Ident]),
	#[atom(CssAtomSet::_MsInlineFlexbox)]
	MsInlineFlexbox(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<DisplayLegacyVendor>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-webkit-box");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-webkit-flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-ms-flexbox");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-ms-inline-flexbox");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, DisplayLegacyVendor, "flex");
		assert_parse_error!(CssAtomSet::ATOMS, DisplayLegacyVendor, "foo");
	}
}
