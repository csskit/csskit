use super::{UnknownAtRule, UnknownQualifiedRule, rules, values};
use crate::selector::SelectorList;
use css_ast::{Computed, Custom, Unknown};
use css_lexer::Cursor;
use css_parse::{
	BadDeclaration, Build, DeclarationValue as DeclarationValueTrait, Parse, Parser, Peek, QualifiedRule,
	Result as ParserResult, State, T, keyword_set,
};
use csskit_derives::{Parse, ToCursors, ToSpan, Visitable};
use csskit_proc_macro::visit;

/// Represents a "Node Rule", such as `tag { message: "tag" }`.
///
/// The Node Rule is comprised of two child nodes: the [SelectorList] represents the selectors of the rule, and the
/// [NodeRuleBlock] represents the block, including the `{` and `}`.
///
/// ```md
/// <node-rule>
///  │├─ <selector-list> ─ <node-declaration> ─┤│
///
/// ```
#[derive(Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct NodeRule<'a>(pub QualifiedRule<'a, SelectorList<'a>, NodeRuleDeclarationValue<'a>, NestedGroupRule<'a>>);

macro_rules! apply_properties {
	($macro:ident) => {
		$macro! {
			Message(MessageStyleValue<'a>): "message",
			Help(HelpStyleValue<'a>): "help",
		}
	};
}

macro_rules! node_value {
	( $( $name: ident($ty: ident$(<$a: lifetime>)?): $str: tt,)* ) => {
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
		#[visit(self)]
		pub enum NodeRuleDeclarationValue<'a> {
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

apply_properties!(node_value);

macro_rules! define_property_id {
	( $( $name: ident($ty: ident$(<$a: lifetime>)?): $str: tt,)* ) => {
		keyword_set!(pub enum NodeRulePropertyId {
			$($name: $str,)*
		});
	}
}
apply_properties!(define_property_id);

impl<'a> DeclarationValueTrait<'a> for NodeRuleDeclarationValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name(p: &Parser<'a>, c: Cursor) -> bool {
		NodeRulePropertyId::peek(p, c)
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
				match NodeRulePropertyId::build(p, name) {
					$(NodeRulePropertyId::$name(_) => p.parse::<values::$ty>().map(Self::$name),)+
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

macro_rules! apply_rules {
	($macro: ident) => {
		$macro! {
			FixRule<'a>: "fix",
			WithRule<'a>: "with",
			TestRule<'a>: "test",
			PassRule<'a>: "pass",
		}
	};
}

macro_rules! nested_group_rule {
    ( $(
        $name: ident$(<$a: lifetime>)?: $str: pat,
    )* ) => {
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
		pub enum NestedGroupRule<'a> {
			$(
				$name(rules::$name$(<$a>)?),
			)*
			UnknownAt(UnknownAtRule<'a>),
			Node(NodeRule<'a>),
			Unknown(UnknownQualifiedRule<'a>),
			BadDeclaration(BadDeclaration<'a>),
		}
	}
}
apply_rules!(nested_group_rule);

impl<'a> Parse<'a> for NestedGroupRule<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		if p.peek::<T![AtKeyword]>() {
			let c: Cursor = p.peek_n(1);
			macro_rules! parse_rule {
				( $(
					$name: ident$(<$a: lifetime>)?: $str: pat,
				)* ) => {
					match p.parse_str_lower(c) {
						$($str => p.parse::<rules::$name>().map(Self::$name),)*
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
		} else if let Ok(rule) = p.parse::<NodeRule>() {
			Ok(Self::Node(rule))
		} else {
			p.rewind(checkpoint);
			if let Ok(rule) = p.parse::<UnknownQualifiedRule>() {
				Ok(Self::Unknown(rule))
			} else {
				p.rewind(checkpoint);
				let state = p.set_state(State::Nested);
				let declaration = p.parse::<BadDeclaration>();
				p.set_state(state);
				Ok(Self::BadDeclaration(declaration?))
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<NodeRule>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(NodeRule, "tag{warn:\"Don't use tags!\"}");
	}
}
