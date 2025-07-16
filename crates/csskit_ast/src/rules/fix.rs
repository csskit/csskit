use crate::{NodeRule, values};
use css_ast::{Computed, Custom, Unknown};
use css_lexer::Cursor;
use css_parse::{
	AtRule, Block, Build, DeclarationValue as DeclarationValueTrait, NoPreludeAllowed, Parser, Peek,
	Result as ParserResult, atkeyword_set, keyword_set,
};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

atkeyword_set!(struct AtFixKeyword "fix");

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct FixRule<'a>(AtRule<'a, AtFixKeyword, NoPreludeAllowed, FixRuleBlock<'a>>);

#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct FixRuleBlock<'a>(Block<'a, FixRuleDeclarationValue<'a>, NodeRule<'a>>);

macro_rules! apply_properties {
	($macro:ident) => {
		$macro! {
			Content(ContentStyleValue<'a>): "content",
			Action(ActionStyleValue): "action",
		}
	};
}

macro_rules! fix_value {
	( $( $name: ident($ty: ident$(<$a: lifetime>)?): $str: tt,)+ ) => {
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
		#[visit(self)]
		pub enum FixRuleDeclarationValue<'a> {
			$(
				#[cfg_attr(feature = "serde", serde(untagged))]
				$name(values::$ty$(<$a>)?),
			)+
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
		keyword_set!(pub enum FixPropertyId {
			$($name: $str,)*
		});
	}
}
apply_properties!(define_property_id);

impl<'a> DeclarationValueTrait<'a> for FixRuleDeclarationValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name(p: &Parser<'a>, c: Cursor) -> bool {
		FixPropertyId::peek(p, c)
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
				match FixPropertyId::build(p, name) {
					$(FixPropertyId::$name(_) => p.parse::<values::$ty>().map(Self::$name),)+
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

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<FixRule>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(FixRule, r#"@fix{replace:""}"#);
	}
}
