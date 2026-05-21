use super::prelude::*;

/// <https://drafts.csswg.org/css-color-5/#light-dark>
///
/// ```text,ignore
/// light-dark() = light-dark( <color>, <color> )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(all))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LightDarkFunction<'a> {
	#[atom(CssAtomSet::LightDark)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub light: Color<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub comma: T![,],
	pub dark: Color<'a>,
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LightDarkFunction>(), 88);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LightDarkFunction, "light-dark(red,blue)");
		assert_parse!(CssAtomSet::ATOMS, LightDarkFunction, "light-dark(#fff,#000)");
		assert_parse!(CssAtomSet::ATOMS, LightDarkFunction, "light-dark(rgb(255,255,255),rgb(0,0,0))");
		assert_parse!(CssAtomSet::ATOMS, LightDarkFunction, "light-dark(white,black)");
	}
}
