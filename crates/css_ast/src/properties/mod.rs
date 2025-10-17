use crate::{CssAtomSet, values};
use css_lexer::Kind;
use css_parse::{
	ComponentValues, Cursor, DeclarationValue, Diagnostic, KindSet, Parser, Peek, Result as ParserResult, State, T,
};
use csskit_derives::{Parse, ToCursors, ToSpan};
use std::{fmt::Debug, hash::Hash};

// The build.rs generates a list of CSS properties from the value mods
include!(concat!(env!("OUT_DIR"), "/css_apply_properties.rs"));

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
pub struct Custom<'a>(pub ComponentValues<'a>);

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
pub struct Computed<'a>(pub ComponentValues<'a>);

impl<'a> Peek<'a> for Computed<'a> {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		<T![Function]>::peek(p, c)
			&& matches!(
				p.to_atom::<CssAtomSet>(c),
				CssAtomSet::Var
					| CssAtomSet::Calc
					| CssAtomSet::Min
					| CssAtomSet::Max
					| CssAtomSet::Clamp
					| CssAtomSet::Round
					| CssAtomSet::Mod
					| CssAtomSet::Rem
					| CssAtomSet::Sin
					| CssAtomSet::Cos
					| CssAtomSet::Tan
					| CssAtomSet::Asin
					| CssAtomSet::Atan
					| CssAtomSet::Atan2
					| CssAtomSet::Pow
					| CssAtomSet::Sqrt
					| CssAtomSet::Hypot
					| CssAtomSet::Log
					| CssAtomSet::Exp
					| CssAtomSet::Abs
					| CssAtomSet::Sign
			)
	}
}

#[derive(Parse, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable))]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[parse(state = State::Nested, stop = KindSet::RIGHT_CURLY_OR_SEMICOLON)]
pub struct Unknown<'a>(pub ComponentValues<'a>);

macro_rules! style_value {
	( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
		#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
		#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
		#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
		pub enum StyleValue<'a> {
			#[cfg_attr(feature = "visitable", visit(skip))]
			Initial(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			Inherit(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			Unset(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
			Revert(T![Ident]),
			#[cfg_attr(feature = "visitable", visit(skip))]
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

impl<'a> DeclarationValue<'a> for StyleValue<'a> {
	type ComputedValue = Computed<'a>;

	fn valid_declaration_name<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		macro_rules! match_name {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $str: tt,)+ ) => {
				match p.to_atom::<CssAtomSet>(c) {
					$(CssAtomSet::$name => true,)+
					_ => false,
				}
			}
		}
		apply_properties!(match_name)
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

	fn parse_custom_declaration_value<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Custom>().map(Self::Custom)
	}

	fn parse_computed_declaration_value<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Computed>().map(Self::Computed)
	}

	fn parse_specified_declaration_value<I>(p: &mut Parser<'a, I>, name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let c = p.peek_n(1);
		if c == Kind::Ident {
			match p.to_atom::<CssAtomSet>(c) {
				CssAtomSet::Initial => return Ok(Self::Initial(p.parse::<T![Ident]>()?)),
				CssAtomSet::Inherit => return Ok(Self::Inherit(p.parse::<T![Ident]>()?)),
				CssAtomSet::Unset => return Ok(Self::Unset(p.parse::<T![Ident]>()?)),
				CssAtomSet::Revert => return Ok(Self::Revert(p.parse::<T![Ident]>()?)),
				CssAtomSet::RevertLayer => return Ok(Self::RevertLayer(p.parse::<T![Ident]>()?)),
				_ => {}
			}
		}
		macro_rules! parse_declaration_value {
			( $( $name: ident: $ty: ident$(<$a: lifetime>)? = $atom: ident,)+ ) => {
				match p.to_atom::<CssAtomSet>(name) {
					$(CssAtomSet::$atom => p.parse::<values::$ty>().map(Self::$name),)+
					_ => Err(Diagnostic::new(name, Diagnostic::unexpected))?,
				}
			}
		}
		apply_properties!(parse_declaration_value)
	}

	fn parse_unknown_declaration_value<I>(p: &mut Parser<'a, I>, _name: Cursor) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		p.parse::<Unknown>().map(Self::Unknown)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{Declaration, assert_parse};

	type Property<'a> = Declaration<'a, StyleValue<'a>>;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Property>(), 368);
		assert_eq!(std::mem::size_of::<StyleValue>(), 296);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Property, "width:inherit", Property { value: StyleValue::Inherit(_), .. });
		assert_parse!(
			CssAtomSet::ATOMS,
			Property,
			"width:inherit!important",
			Property { value: StyleValue::Inherit(_), important: Some(_), .. }
		);
		assert_parse!(
			CssAtomSet::ATOMS,
			Property,
			"width:revert;",
			Property { value: StyleValue::Revert(_), semicolon: Some(_), .. }
		);
		assert_parse!(CssAtomSet::ATOMS, Property, "width:var(--a)", Property { value: StyleValue::Computed(_), .. });

		assert_parse!(CssAtomSet::ATOMS, Property, "float:none!important");
		assert_parse!(CssAtomSet::ATOMS, Property, "width:1px");
		assert_parse!(CssAtomSet::ATOMS, Property, "width:min(1px, 2px)");
		assert_parse!(CssAtomSet::ATOMS, Property, "border:1px solid var(--red)");
		// Should still parse unknown properties
		assert_parse!(CssAtomSet::ATOMS, Property, "dunno:like whatever");
		assert_parse!(CssAtomSet::ATOMS, Property, "rotate:1.21gw");
		assert_parse!(CssAtomSet::ATOMS, Property, "_background:black");
		assert_parse!(CssAtomSet::ATOMS, Property, "--custom:{foo:{bar};baz:(bing);}");
	}
}
