use crate::{CssAtomSet, CssMetadata, NodeKinds, StyleValue, rules, stylerule::StyleRule};
use bumpalo::collections::Vec;
use css_parse::{
	ComponentValues, Cursor, Diagnostic, NodeWithMetadata, Parse, Parser, QualifiedRule, Result as ParserResult,
	RuleVariants, StyleSheet as StyleSheetTrait, T, UnknownRuleBlock,
};
use csskit_derives::{Parse, Peek, SemanticEq, ToCursors, ToSpan};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit, metadata(skip))]
pub struct StyleSheet<'a> {
	pub rules: Vec<'a, Rule<'a>>,
	#[to_cursors(skip)]
	#[cfg_attr(feature = "serde", serde(skip))]
	#[cfg_attr(feature = "visitable", visit(skip))]
	meta: CssMetadata,
}

impl<'a> NodeWithMetadata<CssMetadata> for StyleSheet<'a> {
	fn metadata(&self) -> CssMetadata {
		self.meta
	}
}

// A StyleSheet represents the root node of a CSS-like language.
// The StyleSheet trait represents an abstraction of this, which allows for
// alternate implementations such as SCSS.
// AtRules vs QualifiedRules are differentiated by two different functions.
impl<'a> Parse<'a> for StyleSheet<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let (rules, meta) = Self::parse_stylesheet(p)?;
		Ok(Self { rules, meta })
	}
}

impl<'a> StyleSheetTrait<'a, CssMetadata> for StyleSheet<'a> {
	type Rule = Rule<'a>;
}

macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			Charset(CharsetRule): CssAtomSet::Charset,
			ColorProfile(ColorProfileRule): CssAtomSet::ColorProfile,
			Container(ContainerRule<'a>): CssAtomSet::Container,
			CounterStyle(CounterStyleRule<'a>): CssAtomSet::CounterStyle,
			FontFace(FontFaceRule<'a>): CssAtomSet::FontFace,
			FontFeatureValues(FontFeatureValuesRule): CssAtomSet::FontFeatureValues,
			FontPaletteValues(FontPaletteValuesRule): CssAtomSet::FontPaletteValues,
			Import(ImportRule): CssAtomSet::Import,
			Keyframes(KeyframesRule<'a>): CssAtomSet::Keyframes,
			Layer(LayerRule<'a>): CssAtomSet::Layer,
			Media(MediaRule<'a>): CssAtomSet::Media,
			Namespace(NamespaceRule): CssAtomSet::Namespace,
			Page(PageRule<'a>): CssAtomSet::Page,
			Property(PropertyRule<'a>): CssAtomSet::Property,
			Scope(ScopeRule): CssAtomSet::Scope,
			StartingStyle(StartingStyleRule<'a>): CssAtomSet::StartingStyle,
			Supports(SupportsRule<'a>): CssAtomSet::Supports,

			// Deprecated Rules
			Document(DocumentRule<'a>): CssAtomSet::Document,

			// Vendor Prefixed
			WebkitKeyframes(WebkitKeyframesRule<'a>): CssAtomSet::_WebkitKeyframes,

			// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#at-rules
			MozDocument(MozDocumentRule<'a>): CssAtomSet::_MozDocument,
		}
	};
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self), metadata(skip))]
pub struct UnknownAtRule<'a> {
	name: T![AtKeyword],
	prelude: ComponentValues<'a>,
	block: ComponentValues<'a>,
}

impl<'a> NodeWithMetadata<CssMetadata> for UnknownAtRule<'a> {
	fn metadata(&self) -> CssMetadata {
		CssMetadata { node_kinds: NodeKinds::Unknown, ..Default::default() }
	}
}

#[derive(Parse, Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self), metadata(skip))]
pub struct UnknownQualifiedRule<'a>(
	QualifiedRule<
		'a,
		UnknownRuleBlock<'a>,
		StyleValue<'a>,
		UnknownRuleBlock<'a, StyleValue<'a>, CssMetadata>,
		CssMetadata,
	>,
);

impl<'a> NodeWithMetadata<CssMetadata> for UnknownQualifiedRule<'a> {
	fn metadata(&self) -> CssMetadata {
		let mut meta = self.0.metadata();
		meta.node_kinds |= NodeKinds::Unknown;
		meta
	}
}

macro_rules! rule {
    ( $(
        $name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
    )+ ) => {
		#[allow(clippy::large_enum_variant)] // TODO: Box?
		// https://drafts.csswg.org/cssom-1/#the-cssrule-interface
		#[derive(ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		pub enum Rule<'a> {
			$(
				$name(rules::$ty$(<$a>)?),
			)+
			UnknownAt(UnknownAtRule<'a>),
			Style(StyleRule<'a>),
			Unknown(UnknownQualifiedRule<'a>)
		}
	}
}

apply_rules!(rule);

impl<'a> NodeWithMetadata<CssMetadata> for Rule<'a> {
	fn metadata(&self) -> CssMetadata {
		macro_rules! match_rule {
			( $(
				$name: ident($ty: ident$(<$a: lifetime>)?): $atoms: pat,
			)+ ) => {
				match self {
					$(Self::$name(r) => r.metadata(),)+
					Self::Style(r) => r.metadata(),
					Self::UnknownAt(r) => r.metadata(),
					Self::Unknown(r) => r.metadata(),
				}
			}
		}
		apply_rules!(match_rule)
	}
}

impl<'a> RuleVariants<'a> for Rule<'a> {
	type DeclarationValue = StyleValue<'a>;
	type Metadata = CssMetadata;

	fn parse_at_rule<I>(p: &mut Parser<'a, I>, c: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		macro_rules! parse_rule {
			( $(
				$name: ident($ty: ident$(<$a: lifetime>)?): $atoms: pat,
			)+ ) => {
				match p.to_atom::<CssAtomSet>(c) {
					$($atoms => p.parse::<rules::$ty>().map(Self::$name),)+
					_ => Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?,
				}
			}
		}
		apply_rules!(parse_rule)
	}

	fn parse_unknown_at_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<UnknownAtRule>().map(Self::UnknownAt)
	}

	fn parse_qualified_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<StyleRule>().map(Self::Style)
	}

	fn parse_unknown_qualified_rule<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<UnknownQualifiedRule>().map(Self::Unknown)
	}
}

impl<'a> Parse<'a> for Rule<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_rule_variants(p)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StyleSheet>(), 56);
		assert_eq!(std::mem::size_of::<Rule>(), 640);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "body{}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "body{color:red;}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "body,tr:nth-child(n-1){}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "body{width:1px;}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "body{width:1px;}.a{width:2px;}");
		assert_parse!(CssAtomSet::ATOMS, StyleSheet, "one:1;a{two:2}");
		assert_parse!(CssAtomSet::ATOMS, Rule, "@media screen{}", Rule::Media(_));
		assert_parse!(CssAtomSet::ATOMS, Rule, "@layer foo{}", Rule::Layer(_));
	}
}
