use super::prelude::*;

/// Vendor-prefixed legacy display values.
///
/// These are non-standard values used for backwards compatibility with older
/// browsers' flex implementations.
///
/// ```text,ignore
/// <display-legacy-vendor> = box | -webkit-box | -webkit-inline-box | -webkit-flex | -webkit-inline-flex
///                         | -moz-box | -moz-inline-box | -moz-flex | -moz-flexbox | -moz-inline-stack
///                         | -ms-flex | -ms-inline-flex | -ms-flexbox | -ms-inline-flexbox | -ms-grid
///                         | -o-flex
/// ```
#[node]
#[derive(
	Parse, Peek, IntoCursor, ToSpan, SemanticEq, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum DisplayLegacyVendor {
	#[atom(CssAtomSet::Box)]
	LegacyBox(T![Ident]),
	#[atom(CssAtomSet::_WebkitBox)]
	WebkitBox(T![Ident]),
	#[atom(CssAtomSet::_WebkitInlineBox)]
	WebkitInlineBox(T![Ident]),
	#[atom(CssAtomSet::_WebkitFlex)]
	WebkitFlex(T![Ident]),
	#[atom(CssAtomSet::_WebkitInlineFlex)]
	WebkitInlineFlex(T![Ident]),
	#[atom(CssAtomSet::_MozBox)]
	MozBox(T![Ident]),
	#[atom(CssAtomSet::_MozInlineBox)]
	MozInlineBox(T![Ident]),
	#[atom(CssAtomSet::_MozFlex)]
	MozFlex(T![Ident]),
	#[atom(CssAtomSet::_MozFlexbox)]
	MozFlexbox(T![Ident]),
	#[atom(CssAtomSet::_MozInlineStack)]
	MozInlineStack(T![Ident]),
	#[atom(CssAtomSet::_MsFlex)]
	MsFlex(T![Ident]),
	#[atom(CssAtomSet::_MsInlineFlex)]
	MsInlineFlex(T![Ident]),
	#[atom(CssAtomSet::_MsFlexbox)]
	MsFlexbox(T![Ident]),
	#[atom(CssAtomSet::_MsInlineFlexbox)]
	MsInlineFlexbox(T![Ident]),
	#[atom(CssAtomSet::_MsGrid)]
	MsGrid(T![Ident]),
	#[atom(CssAtomSet::_OFlex)]
	OFlex(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "box");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-webkit-box");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-webkit-inline-box");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-webkit-flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-webkit-inline-flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-moz-box");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-moz-inline-box");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-moz-flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-moz-flexbox");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-moz-inline-stack");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-ms-flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-ms-inline-flex");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-ms-flexbox");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-ms-inline-flexbox");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-ms-grid");
		assert_parse!(CssAtomSet::ATOMS, DisplayLegacyVendor, "-o-flex");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, DisplayLegacyVendor, "flex");
		assert_peek_false!(CssAtomSet::ATOMS, DisplayLegacyVendor, "foo");
	}
}
