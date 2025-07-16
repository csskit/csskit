use bumpalo::collections::Vec;
use css_lexer::Cursor;
use css_parse::{
	AtRule, Build, ComponentValues, Parse, Parser, Peek, QualifiedRule, Result as ParserResult, RuleVariants,
	StyleSheet as StyleSheetTrait, T, atkeyword_set, diagnostics,
};
use csskit_derives::{Parse, ToCursors, ToSpan, Visitable};

use crate::{NodeRule, rules};

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
#[visit]
pub struct RuleSheet<'a> {
	pub rules: Vec<'a, Rule<'a>>,
}

impl<'a> Parse<'a> for RuleSheet<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self { rules: Self::parse_stylesheet(p)? })
	}
}

impl<'a> StyleSheetTrait<'a> for RuleSheet<'a> {
	type Rule = Rule<'a>;
}

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct UnknownAtRule<'a>(AtRule<'a, T![AtKeyword], ComponentValues<'a>, ComponentValues<'a>>);

#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct UnknownQualifiedRule<'a>(QualifiedRule<'a, ComponentValues<'a>, ComponentValues<'a>, ComponentValues<'a>>);

macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			Import(ImportRule<'a>): "import",
			Lint(LintRule<'a>): "lint",
		}
	};
}

macro_rules! rule {
    ( $(
        $name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
    )+ ) => {
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		pub enum Rule<'a> {
			$(
				$name(rules::$ty$(<$a>)?),
			)+
			Node(NodeRule<'a>),
			UnknownAt(UnknownAtRule<'a>),
			Unknown(UnknownQualifiedRule<'a>)
		}
	}
}

apply_rules!(rule);

macro_rules! define_atkeyword_set {
	( $(
		$at_name:ident($ty:ty): $str:tt,
	)+ ) => {
		atkeyword_set!(
			enum AtRuleKeywords {
				$($at_name: $str,)+
			}
		);
	}
}

apply_rules!(define_atkeyword_set);

impl<'a> RuleVariants<'a> for Rule<'a> {
	fn parse_at_rule(p: &mut Parser<'a>, c: Cursor) -> ParserResult<Self> {
		if !AtRuleKeywords::peek(p, c) {
			Err(diagnostics::Unexpected(c.into(), c.into()))?;
		}
		let kw = AtRuleKeywords::build(p, c);
		macro_rules! parse_rule {
			( $(
				$name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
			)+ ) => {
				match kw {
					$(AtRuleKeywords::$name(_) => dbg!(p.parse::<rules::$ty>().map(Self::$name)),)+
				}
			}
		}
		apply_rules!(parse_rule)
	}

	fn parse_unknown_at_rule(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<UnknownAtRule>().map(Self::UnknownAt)
	}

	fn parse_qualified_rule(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<NodeRule>().map(Self::Node)
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
		assert_eq!(std::mem::size_of::<RuleSheet>(), 32);
		assert_eq!(std::mem::size_of::<Rule>(), 136);
	}

	#[test]
	fn test_writes() {
		assert_parse!(RuleSheet, "tag{}");
		assert_parse!(RuleSheet, "tag,tr:nth-child(n-1){}");
		assert_parse!(RuleSheet, "tag{width:1px;}");
		assert_parse!(RuleSheet, "tag{width:1px;}#a{width:2px;}");
		assert_parse!(RuleSheet, "tag:1;tag{two:2}");
	}
}
