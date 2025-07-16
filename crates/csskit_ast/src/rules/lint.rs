use crate::{NodeRule, UnknownAtRule, UnknownQualifiedRule, rules, values};
use css_ast::{Computed, Custom, Unknown};
use css_lexer::Cursor;
use css_parse::{
	AtRule, Build, DeclarationRuleList, DeclarationValue as DeclarationValueTrait, Parse, Parser, Peek,
	Result as ParserResult, RuleVariants, T, atkeyword_set, diagnostics, keyword_set,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

atkeyword_set!(struct AtLintKeyword "lint");

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct LintRule<'a>(AtRule<'a, AtLintKeyword, LintName, LintRuleBlock<'a>>);

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LintName(T![Ident]);

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LintRuleBlock<'a>(DeclarationRuleList<'a, LintRuleDeclarationValue<'a>, LintRuleRule<'a>>);

macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			Fix(FixRule<'a>): "fix",
			Pass(PassRule<'a>): "pass",
			With(WithRule<'a>): "with",
		}
	};
}

macro_rules! rule {
    ( $(
        $name: ident($ty: ident$(<$a: lifetime>)?): $str: pat,
    )+ ) => {
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		pub enum LintRuleRule<'a> {
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

impl<'a> RuleVariants<'a> for LintRuleRule<'a> {
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

	fn parse_unknown_qualified_rule(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<UnknownQualifiedRule>().map(Self::Unknown)
	}
}

macro_rules! apply_properties {
	($macro:ident) => {
		$macro! {
			Category(CategoryStyleValue): "category",
			Lang(LangStyleValue): "lang",
		}
	};
}

macro_rules! lint_value {
	( $( $name: ident($ty: ident$(<$a: lifetime>)?): $str: tt,)* ) => {
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
		#[visit(self)]
		pub enum LintRuleDeclarationValue<'a> {
			$(
				#[cfg_attr(feature = "serde", serde(untagged))]
				$name(values::$ty$(<$a>)?),
			)*
			#[cfg_attr(feature = "serde", serde(untagged))]
			Custom(Custom<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Computed(Computed<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Unknown(Unknown<'a>),
		}
	}
}

impl<'a> Parse<'a> for LintRuleRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_rule_variants(p)
	}
}

apply_properties!(lint_value);

macro_rules! define_property_id {
	( $( $name: ident($ty: ident$(<$a: lifetime>)?): $str: tt,)* ) => {
		keyword_set!(pub enum LintPropertyId {
			$($name: $str,)*
		});
	}
}
apply_properties!(define_property_id);

impl<'a> DeclarationValueTrait<'a> for LintRuleDeclarationValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name(p: &Parser<'a>, c: Cursor) -> bool {
		LintPropertyId::peek(p, c)
	}

	fn parse_custom_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<Custom>().map(Self::Custom)
	}

	fn parse_computed_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<Computed>().map(Self::Computed)
	}

	fn parse_unknown_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<Unknown>().map(Self::Unknown)
	}

	fn parse_specified_declaration_value(p: &mut Parser<'a>, name: Cursor) -> ParserResult<Self> {
		macro_rules! parse_declaration_value {
			( $( $name: ident($ty: ident$(<$a: lifetime>)?): $str: tt,)+ ) => {
				match LintPropertyId::build(p, name) {
					$(LintPropertyId::$name(_) => p.parse::<values::$ty>().map(Self::$name),)+
				}
			}
		}
		apply_properties!(parse_declaration_value)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn needs_computing(&self) -> bool {
		matches!(self, Self::Computed(_))
	}
}
