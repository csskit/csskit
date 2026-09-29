use super::prelude::*;

/// <https://drafts.csswg.org/css-sizing-4/#typedef-box-size>
///
/// ```text,ignore
/// <box-size> = <length-percentage [0,∞]> | stretch | contain | min-content | max-content | fit-content | fit-content(<length-percentage [0,∞]>) | <calc-size()>
/// ```
///
/// The `-webkit-`/`-moz-` prefixed values are legacy aliases of `min-content`/`max-content`.
#[syntax(
	" <length-percentage [0,∞]> | stretch | contain | min-content | max-content | fit-content(<length-percentage [0,∞]>) | fit-content | <calc-size()> | -webkit-max-content | -moz-max-content | -webkit-min-content | -moz-min-content "
)]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum BoxSize<'a> {}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, BoxSize, "10px", BoxSize::LengthPercentage(_));
		assert_parse!(CssAtomSet::ATOMS, BoxSize, "stretch", BoxSize::Stretch(_));
		assert_parse!(CssAtomSet::ATOMS, BoxSize, "contain", BoxSize::Contain(_));
		assert_parse!(CssAtomSet::ATOMS, BoxSize, "min-content", BoxSize::MinContent(_));
		assert_parse!(CssAtomSet::ATOMS, BoxSize, "fit-content", BoxSize::FitContent(_));
		assert_parse!(CssAtomSet::ATOMS, BoxSize, "fit-content(20rem)");
		assert_parse!(CssAtomSet::ATOMS, BoxSize, "calc-size(auto, size/2)");
	}
}
