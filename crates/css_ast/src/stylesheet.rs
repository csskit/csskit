use crate::{CssAtomSet, StyleValue, rules, stylerule::StyleRule};
use bumpalo::collections::Vec;
use css_parse::{
	ComponentValues, Cursor, Diagnostic, Parse, Parser, QualifiedRule, Result as ParserResult, RuleVariants,
	StyleSheet as StyleSheetTrait, T,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct StyleSheet<'a> {
	pub rules: Vec<'a, Rule<'a>>,
}

// A StyleSheet represents the root node of a CSS-like language.
// The StyleSheet trait represents an abstraction of this, which allows for
// alternate implementations such as SCSS.
// AtRules vs QualifiedRules are differentiated by two different functions.
impl<'a> Parse<'a> for StyleSheet<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self { rules: Self::parse_stylesheet(p)? })
	}
}

impl<'a> StyleSheetTrait<'a> for StyleSheet<'a> {
	type Rule = Rule<'a>;
}

macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			Charset(CharsetRule): CssAtomSet::Charset,
			ColorProfile(ColorProfileRule): CssAtomSet::ColorProfile,
			Container(ContainerRule<'a>): CssAtomSet::Container,
			CounterStyle(CounterStyleRule): CssAtomSet::CounterStyle,
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
			StartingStyle(StartingStyleRule): CssAtomSet::StartingStyle,
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

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct UnknownAtRule<'a> {
	name: T![AtKeyword],
	prelude: ComponentValues<'a>,
	block: ComponentValues<'a>,
}

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct UnknownQualifiedRule<'a>(QualifiedRule<'a, ComponentValues<'a>, StyleValue<'a>, ComponentValues<'a>>);

macro_rules! rule {
    ( $(
        $name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
    )+ ) => {
		#[allow(clippy::large_enum_variant)] // TODO: Box?
		// https://drafts.csswg.org/cssom-1/#the-cssrule-interface
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
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

impl<'a> RuleVariants<'a> for Rule<'a> {
	fn parse_at_rule(p: &mut Parser<'a>, c: Cursor) -> ParserResult<Self> {
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

	fn parse_unknown_at_rule(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<UnknownAtRule>().map(Self::UnknownAt)
	}

	fn parse_qualified_rule(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<StyleRule>().map(Self::Style)
	}

	fn parse_unknown_qualified_rule(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<UnknownQualifiedRule>().map(Self::Unknown)
	}
}

impl<'a> Parse<'a> for Rule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
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
		assert_eq!(std::mem::size_of::<StyleSheet>(), 32);
		assert_eq!(std::mem::size_of::<Rule>(), 496);
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
