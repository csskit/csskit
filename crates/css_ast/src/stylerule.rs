use crate::{StyleValue, selector::SelectorList};
use css_parse::{
	Cursor, Parse, Parser, QualifiedRule, Result as ParserResult, RuleVariants, atkeyword_set, syntax::BadDeclaration,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};
use csskit_proc_macro::visit;

use super::{UnknownAtRule, UnknownQualifiedRule, rules};

/// Represents a "Style Rule", such as `body { width: 100% }`. See also the CSS-OM [CSSStyleRule][1] interface.
///
/// The Style Rule is comprised of two child nodes: the [SelectorList] represents the selectors of the rule.
/// Each [Declaration][css_parse::Declaration] will have a [StyleValue], and each rule will be a [NestedGroupRule].
///
/// [1]: https://drafts.csswg.org/cssom-1/#the-cssstylerule-interface
#[derive(Parse, Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct StyleRule<'a>(pub QualifiedRule<'a, SelectorList<'a>, StyleValue<'a>, NestedGroupRule<'a>>);

// https://drafts.csswg.org/css-nesting/#conditionals
macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			Container(ContainerRule<'a>): "container",
			Layer(LayerRule<'a>): "layer",
			Media(MediaRule<'a>): "media",
			Scope(ScopeRule): "scope",
			Supports(SupportsRule<'a>): "supports",
		}
	};
}

macro_rules! nested_group_rule {
    ( $(
        $name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
    )+ ) => {
		#[allow(clippy::large_enum_variant)] // TODO: Box?
		// https://drafts.csswg.org/cssom-1/#the-cssrule-interface
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		pub enum NestedGroupRule<'a> {
			$(
				$name(rules::$ty$(<$a>)?),
			)+
			UnknownAt(UnknownAtRule<'a>),
			Style(StyleRule<'a>),
			Unknown(UnknownQualifiedRule<'a>),
			BadDeclaration(BadDeclaration<'a>),
		}
	}
}
apply_rules!(nested_group_rule);

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

impl<'a> RuleVariants<'a> for NestedGroupRule<'a> {
	fn parse_at_rule(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		let kw = p.parse::<AtRuleKeywords>()?;
		macro_rules! parse_rule {
			( $(
				$name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
			)+ ) => {
				match kw {
					$(AtRuleKeywords::$name(_) => p.parse::<rules::$ty>().map(Self::$name),)+
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

	fn bad_declaration(b: BadDeclaration<'a>) -> Option<Self> {
		Some(Self::BadDeclaration(b))
	}
}

impl<'a> Parse<'a> for NestedGroupRule<'a> {
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
		assert_eq!(std::mem::size_of::<StyleRule>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(StyleRule, "body{}");
		assert_parse!(StyleRule, "body,body{}");
		assert_parse!(StyleRule, "body{width:1px;}");
		assert_parse!(StyleRule, "body{opacity:0;}");
		assert_parse!(StyleRule, ".foo *{}", ".foo *{}");
		assert_parse!(StyleRule, ":nth-child(1){opacity:0;}");
		assert_parse!(StyleRule, ".foo{--bar:(baz);}");
		assert_parse!(StyleRule, ".foo{width: calc(1px + (var(--foo)) + 1px);}");
		assert_parse!(StyleRule, ".foo{--bar:1}");
		assert_parse!(StyleRule, ":root{--custom:{width:0;height:0;};}");
		// Semicolons are "allowed" in geneirc preludes
		assert_parse!(StyleRule, ":root{a;b{}}");
		// Bad Declarations should be parsable.
		assert_parse!(StyleRule, ":root{$(var)-size: 100%;}");
	}
}
