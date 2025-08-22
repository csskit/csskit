use crate::values;
use css_lexer::{Cursor, KindSet};
use css_parse::{
	Build, ComponentValues, DeclarationValue, Parser, Peek, Result as ParserResult, State, T, keyword_set,
};
use csskit_derives::{Parse, ToCursors, ToSpan, Visitable};
use std::{fmt::Debug, hash::Hash};

// The build.rs generates a list of CSS properties from the value mods
include!(concat!(env!("OUT_DIR"), "/css_apply_properties.rs"));

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
pub struct Custom<'a>(pub ComponentValues<'a>);

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
pub struct Computed<'a>(pub ComponentValues<'a>);

impl<'a> Peek<'a> for Computed<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Function]>::peek(p, c)
			&& matches!(
				p.parse_str_lower(c),
				"var"
					| "calc" | "min"
					| "max" | "clamp"
					| "round" | "mod"
					| "rem" | "sin" | "cos"
					| "tan" | "asin"
					| "atan" | "atan2"
					| "pow" | "sqrt"
					| "hypot" | "log"
					| "exp" | "abs" | "sign"
			)
	}
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
pub struct Unknown<'a>(pub ComponentValues<'a>);

macro_rules! style_value {
	( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
		#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename_all = "kebab-case"))]
		#[visit(self)]
		pub enum StyleValue<'a> {
			Initial(T![Ident]),
			Inherit(T![Ident]),
			Unset(T![Ident]),
			Revert(T![Ident]),
			RevertLayer(T![Ident]),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Custom(Custom<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Computed(Computed<'a>),
			#[cfg_attr(feature = "serde", serde(untagged))]
			Unknown(Unknown<'a>),
			$(
				#[cfg_attr(feature = "serde", serde(untagged))]
				$name(values::$ty$(<$a>)?),
			)+
		}
	}
}

apply_properties!(style_value);

keyword_set!(pub enum CSSWideKeyword {
	Initial: "initial",
	Inherit: "inherit",
	Unset: "unset",
	Revert: "revert",
	RevertLayer: "revert-layer",
});

macro_rules! define_property_id {
	( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
		keyword_set!(pub enum PropertyId {
			$($name: $str,)+
		});
	}
}
apply_properties!(define_property_id);

impl<'a> DeclarationValue<'a> for StyleValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name(p: &Parser<'a>, c: Cursor) -> bool {
		PropertyId::peek(p, c)
	}

	fn is_unknown(&self) -> bool {
		matches!(self, Self::Unknown(_))
	}

	fn is_initial(&self) -> bool {
		matches!(self, Self::Initial(_))
	}

	fn is_inherit(&self) -> bool {
		matches!(self, Self::Inherit(_))
	}

	fn is_unset(&self) -> bool {
		matches!(self, Self::Unset(_))
	}

	fn is_revert(&self) -> bool {
		matches!(self, Self::Revert(_))
	}

	fn is_revert_layer(&self) -> bool {
		matches!(self, Self::RevertLayer(_))
	}

	fn needs_computing(&self) -> bool {
		matches!(self, Self::Computed(_))
	}

	fn parse_custom_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<Custom>().map(Self::Custom)
	}

	fn parse_computed_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<Computed>().map(Self::Computed)
	}

	fn parse_specified_declaration_value(p: &mut Parser<'a>, name: Cursor) -> ParserResult<Self> {
		match p.parse_if_peek::<CSSWideKeyword>()? {
			Some(CSSWideKeyword::Initial(ident)) => return Ok(Self::Initial(ident)),
			Some(CSSWideKeyword::Inherit(ident)) => return Ok(Self::Inherit(ident)),
			Some(CSSWideKeyword::Unset(ident)) => return Ok(Self::Unset(ident)),
			Some(CSSWideKeyword::Revert(ident)) => return Ok(Self::Revert(ident)),
			Some(CSSWideKeyword::RevertLayer(ident)) => return Ok(Self::RevertLayer(ident)),
			None => {}
		}
		macro_rules! parse_declaration_value {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match PropertyId::build(p, name) {
					$(PropertyId::$name(_) => p.parse::<values::$ty>().map(Self::$name),)+
				}
			}
		}
		apply_properties!(parse_declaration_value)
	}

	fn parse_unknown_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> ParserResult<Self> {
		p.parse::<Unknown>().map(Self::Unknown)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{Declaration, assert_parse};

	type Property<'a> = Declaration<'a, StyleValue<'a>>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Property>(), 400);
		assert_eq!(std::mem::size_of::<StyleValue>(), 328);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Property, "width:inherit", Property { value: StyleValue::Inherit(_), .. });
		assert_parse!(
			Property,
			"width:inherit!important",
			Property { value: StyleValue::Inherit(_), important: Some(_), .. }
		);
		assert_parse!(Property, "width:revert;", Property { value: StyleValue::Revert(_), semicolon: Some(_), .. });
		assert_parse!(Property, "width:var(--a)", Property { value: StyleValue::Computed(_), .. });

		assert_parse!(Property, "float:none!important");
		assert_parse!(Property, "width:1px");
		assert_parse!(Property, "width:min(1px, 2px)");
		assert_parse!(Property, "border:1px solid var(--red)");
		// Should still parse unknown properties
		assert_parse!(Property, "dunno:like whatever");
		assert_parse!(Property, "rotate:1.21gw");
		assert_parse!(Property, "_background:black");
		assert_parse!(Property, "--custom:{foo:{bar};baz:(bing);}");
	}
}
