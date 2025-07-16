use crate::values;
use css_ast::{Computed, Custom, Unknown};
use css_lexer::Cursor;
use css_parse::{
	AtRule, Build, DeclarationList, DeclarationValue as DeclarationValueTrait, Parser, Peek, Result as ParserResult, T,
	atkeyword_set, keyword_set,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

atkeyword_set!(struct AtTestKeyword "test");
atkeyword_set!(struct AtPassKeyword "pass");

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct TestRule<'a>(AtRule<'a, AtTestKeyword, TestName, TestRuleBlock<'a>>);

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct PassRule<'a>(AtRule<'a, AtPassKeyword, TestName, TestRuleBlock<'a>>);

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct TestName(T![Ident]);

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct TestRuleBlock<'a>(DeclarationList<'a, TestRuleDeclarationValue<'a>>);

macro_rules! apply_properties {
	($macro:ident) => {
		$macro! {
			Content(ContentStyleValue<'a>): "content",
			Span(SpanStyleValue<'a>): "span",
		}
	};
}

macro_rules! fix_value {
	( $( $name: ident($ty: ident$(<$a: lifetime>)?): $str: tt,)* ) => {
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
		#[visit(self)]
		pub enum TestRuleDeclarationValue<'a> {
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

apply_properties!(fix_value);

macro_rules! define_property_id {
	( $( $name: ident($ty: ident$(<$a: lifetime>)?): $str: tt,)* ) => {
		keyword_set!(pub enum TestPropertyId {
			$($name: $str,)*
		});
	}
}
apply_properties!(define_property_id);

impl<'a> DeclarationValueTrait<'a> for TestRuleDeclarationValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name(p: &Parser<'a>, c: Cursor) -> bool {
		TestPropertyId::peek(p, c)
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
				match TestPropertyId::build(p, name) {
					$(TestPropertyId::$name(_) => p.parse::<values::$ty>().map(Self::$name),)+
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
