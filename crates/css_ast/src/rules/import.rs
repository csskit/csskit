use super::prelude::*;
use crate::{LayerName, MediaQueryList, SupportsCondition, UrlOrString};

/// <https://drafts.csswg.org/css-cascade-5/#at-ruledef-import>
///
/// ```text
/// @import [ <url> | <string> ]
///  [ layer | layer(<layer-name>) ]?
///  <import-conditions> ;
///
/// <import-conditions>  = [ supports( [ <supports-condition> | <declaration> ] ) ]?
///                      <media-query-list>?
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ImportRule<'a> {
	#[cfg_attr(feature = "visitable", visit(skip))]
	#[atom(CssAtomSet::Import)]
	pub name: T![AtKeyword],
	pub url: UrlOrString,
	pub layer: Option<ImportLayer<'a>>,
	pub supports_condition: Option<ImportSupportsFunction<'a>>,
	pub media_condition: Option<MediaQueryList<'a>>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub semicolon: Option<T![;]>,
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum ImportLayer<'a> {
	#[atom(CssAtomSet::Layer)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	Layer(T![Ident]),
	#[atom(CssAtomSet::Layer)]
	LayerFunction(ImportLayerFunction<'a>),
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ImportLayerFunction<'a> {
	#[atom(CssAtomSet::Layer)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub layer: LayerName<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct ImportSupportsFunction<'a> {
	#[atom(CssAtomSet::Supports)]
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub name: T![Function],
	pub condition: SupportsCondition<'a>,
	#[cfg_attr(feature = "visitable", visit(skip))]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<ImportRule>(), 744);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, ImportRule, "@import \"foo.css\";");
		assert_parse!(CssAtomSet::ATOMS, ImportRule, "@import url(\"foo.css\");");
		assert_parse!(CssAtomSet::ATOMS, ImportRule, "@import url(\"foo.css\") print;");
		assert_parse!(CssAtomSet::ATOMS, ImportRule, "@import url('foo.css') projection, tv;");
		assert_parse!(CssAtomSet::ATOMS, ImportRule, "@import url('foo.css') handheld and (max-width: 400px);");
		assert_parse!(CssAtomSet::ATOMS, ImportRule, "@import url('foo.css') supports(not (display: flex));");
		assert_parse!(
			CssAtomSet::ATOMS,
			ImportRule,
			"@import url('foo.css') layer supports(not (display: flex)) media, print;"
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			ImportRule,
			"@import url('foo.css') layer(main) supports(not (display: flex)) media, print;"
		);
	}
}
