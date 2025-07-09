use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	Parse, Parser, Result as ParserResult, StyleSheet as StyleSheetTrait, T,
	syntax::{AtRule, QualifiedRule},
};
use csskit_derives::{IntoSpan, ToCursors};
use csskit_proc_macro::visit;

use crate::{Visit, Visitable, rules, stylerule::StyleRule};

// https://drafts.csswg.org/cssom-1/#the-cssstylesheet-interface
#[derive(IntoSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "stylesheet"))]
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

impl<'a> Visitable<'a> for StyleSheet<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_style_sheet(self);
		for rule in &self.rules {
			Visitable::accept(rule, v);
		}
	}
}

macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			CharsetRule: "charset",
			ColorProfileRule: "color-profile",
			ContainerRule<'a>: "container",
			CounterStyleRule: "counter-style",
			FontFaceRule<'a>: "font-face",
			FontFeatureValuesRule: "font-feature-values",
			FontPaletteValuesRule: "font-palette-values",
			ImportRule: "import",
			KeyframesRule<'a>: "keyframes",
			LayerRule<'a>: "layer",
			MediaRule<'a>: "media",
			NamespaceRule: "namespace",
			PageRule<'a>: "page",
			PropertyRule<'a>: "property",
			ScopeRule: "scope",
			StartingStyleRule: "starting-style",
			SupportsRule<'a>: "supports",

			// Deprecated Rules
			DocumentRule<'a>: "document",

			// Vendor Prefixed
			WebkitKeyframesRule<'a>: "-webkit-keyframes",

			// https://developer.mozilla.org/en-US/docs/Web/CSS/Mozilla_Extensions#at-rules
			MozDocumentRule<'a>: "-moz-document",
		}
	};
}

#[derive(IntoSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct UnknownAtRule<'a>(AtRule<'a>);

impl<'a> Parse<'a> for UnknownAtRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(p.parse::<AtRule>()?))
	}
}

impl<'a> Visitable<'a> for UnknownAtRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_unknown_at_rule(self);
	}
}

#[derive(IntoSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct UnknownQualifiedRule<'a>(QualifiedRule<'a>);

impl<'a> Parse<'a> for UnknownQualifiedRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(p.parse::<QualifiedRule>()?))
	}
}

impl<'a> Visitable<'a> for UnknownQualifiedRule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		v.visit_unknown_qualified_rule(self);
	}
}

macro_rules! rule {
    ( $(
        $name: ident$(<$a: lifetime>)?: $str: pat,
    )+ ) => {
		// https://drafts.csswg.org/cssom-1/#the-cssrule-interface
		#[derive(IntoSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		pub enum Rule<'a> {
			$(
				$name(rules::$name$(<$a>)?),
			)+
			UnknownAt(UnknownAtRule<'a>),
			Style(StyleRule<'a>),
			Unknown(UnknownQualifiedRule<'a>)
		}
	}
}

apply_rules!(rule);

impl<'a> Parse<'a> for Rule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		if p.peek::<T![AtKeyword]>() {
			let c: Cursor = p.peek_n(1);
			macro_rules! parse_rule {
				( $(
					$name: ident$(<$a: lifetime>)?: $str: pat,
				)+ ) => {
					match p.parse_str_lower(c) {
						$($str => p.parse::<rules::$name>().map(Self::$name),)+
						_ => {
							let rule = p.parse::<UnknownAtRule>()?;
							Ok(Self::UnknownAt(rule))
						}
					}
				}
			}
			if let Ok(rule) = apply_rules!(parse_rule) {
				Ok(rule)
			} else {
				p.rewind(checkpoint);
				p.parse::<UnknownAtRule>().map(Self::UnknownAt)
			}
		} else if let Ok(rule) = p.parse::<StyleRule>() {
			Ok(Self::Style(rule))
		} else {
			p.rewind(checkpoint);
			p.parse::<UnknownQualifiedRule>().map(Self::Unknown)
		}
	}
}

impl<'a> Visitable<'a> for Rule<'a> {
	fn accept<V: Visit<'a>>(&self, v: &mut V) {
		macro_rules! match_rule {
				( $(
					$name: ident$(<$a: lifetime>)?: $str: pat,
				)+ ) => {
					match self {
						$(Self::$name(r) => Visitable::accept(r, v),)+
						Self::UnknownAt(r) => Visitable::accept(r, v),
						Self::Style(r) => Visitable::accept(r, v),
						Self::Unknown(r) => Visitable::accept(r, v),
					};
				}
			}
		apply_rules!(match_rule);
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
		assert_parse!(StyleSheet, "body,tr:nth-child(n-1){}");
		assert_parse!(StyleSheet, "body{width:1px;}");
		assert_parse!(StyleSheet, "body{width:1px;}.a{width:2px;}");
		assert_parse!(StyleSheet, "one:1;a{two:2}");
	}
}
