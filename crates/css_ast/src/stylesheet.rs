use bumpalo::collections::Vec;
use css_parse::{
	AtRule, ComponentValues, Cursor, Diagnostic, Parse, Parser, QualifiedRule, Result as ParserResult, RuleVariants,
	StyleSheet as StyleSheetTrait, T, atkeyword_set,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

use crate::{StyleValue, rules, stylerule::StyleRule};

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
			Charset(CharsetRule): "charset",
			ColorProfile(ColorProfileRule): "color-profile",
			Container(ContainerRule<'a>): "container",
			CounterStyle(CounterStyleRule): "counter-style",
			FontFace(FontFaceRule<'a>): "font-face",
			FontFeatureValues(FontFeatureValuesRule): "font-feature-values",
			FontPaletteValues(FontPaletteValuesRule): "font-palette-values",
			Import(ImportRule): "import",
			Keyframes(KeyframesRule<'a>): "keyframes",
			Layer(LayerRule<'a>): "layer",
			Media(MediaRule<'a>): "media",
			Namespace(NamespaceRule): "namespace",
			Page(PageRule<'a>): "page",
			Property(PropertyRule<'a>): "property",
			Scope(ScopeRule): "scope",
			StartingStyle(StartingStyleRule): "starting-style",
			Supports(SupportsRule<'a>): "supports",

			// Deprecated Rules
			Document(DocumentRule<'a>): "document",

			// Vendor Prefixed
			WebkitKeyframes(WebkitKeyframesRule<'a>): "-webkit-keyframes",

			// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#at-rules
			MozDocument(MozDocumentRule<'a>): "-moz-document",
		}
	};
}

#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct UnknownAtRule<'a>(AtRule<T![AtKeyword], ComponentValues<'a>, ComponentValues<'a>>);

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

macro_rules! define_atkeyword_set {
	( $(
		$name:ident($ty:ty): $str:tt,
	)+ ) => {
		atkeyword_set!(
			enum AtRuleKeywords {
				$($name: $str),+
			}
		);
	}
}

apply_rules!(define_atkeyword_set);

impl<'a> RuleVariants<'a> for Rule<'a> {
	fn parse_at_rule(p: &mut Parser<'a>, c: Cursor) -> ParserResult<Self> {
		macro_rules! parse_rule {
			( $(
				$name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
			)+ ) => {
				match AtRuleKeywords::from_cursor(p, c) {
					$(Some(AtRuleKeywords::$name(_)) => p.parse::<rules::$ty>().map(Self::$name),)+
					None => Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?,
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
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<StyleSheet>(), 32);
		assert_eq!(std::mem::size_of::<Rule>(), 512);
	}

	#[test]
	fn test_writes() {
		assert_parse!(StyleSheet, "body{}");
		assert_parse!(StyleSheet, "body{color:red;}");
		assert_parse!(StyleSheet, "body,tr:nth-child(n-1){}");
		assert_parse!(StyleSheet, "body{width:1px;}");
		assert_parse!(StyleSheet, "body{width:1px;}.a{width:2px;}");
		assert_parse!(StyleSheet, "one:1;a{two:2}");
		assert_parse!(Rule, "@media screen{}", Rule::Media(_));
		assert_parse!(Rule, "@layer foo{}", Rule::Layer(_));
	}
}
