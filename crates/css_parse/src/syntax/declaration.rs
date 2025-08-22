use std::marker::PhantomData;

use crate::{BangImportant, CursorSink, DeclarationValue, Parse, Parser, Peek, Result, T, ToCursors, token_macros};
use css_lexer::{Cursor, Kind};
use csskit_derives::ToSpan;

/// This is a generic type that can be used for AST nodes representing a [Declaration][1], aka "property". This is
/// defined as:
///
/// ```md
/// <property-id>
///  │├─ <ident> ─┤│
///
/// <declaration>
///  │├─ <property-id> ─ ":" ─ <V> ──╮─────────────────────────────╭──╮───────╭┤│
///                                  ╰─ "!" ─ <ident "important"> ─╯  ╰─ ";" ─╯
/// ```
///
/// An ident is parsed first, as the property name, followed by a `:`. After this the given `<V>` will be parsed as the
/// style value. Parsing may continue to a `!important`, or the optional trailing semi `;`, if either are present.
///
/// The grammar of `<V>` isn't defined here - it'll be dependant on the property name. Consequently, `<V>` must
/// implement the [DeclarationValue] trait, which must provide the
/// `parse_declaration_value(&mut Parser<'a>, Cursor) -> Result<Self>` method - the [Cursor] given to said method
/// represents the Ident of the property name, so it can be reasoned about in order to dispatch to the right
/// declaration value parsing step.
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#consume-a-declaration
#[derive(ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct Declaration<'a, V: DeclarationValue<'a>> {
	pub name: token_macros::Ident,
	pub colon: token_macros::Colon,
	pub value: V,
	pub important: Option<BangImportant>,
	pub semicolon: Option<token_macros::Semicolon>,
	#[cfg_attr(feature = "serde", serde(skip))]
	_phantom: PhantomData<&'a ()>,
}

impl<'a, V: DeclarationValue<'a>> Peek<'a> for Declaration<'a, V> {
	fn peek(p: &Parser<'a>, c: css_lexer::Cursor) -> bool {
		c == Kind::Ident && p.peek_n(2) == Kind::Colon
	}
}

impl<'a, V: DeclarationValue<'a>> Parse<'a> for Declaration<'a, V> {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let name = p.parse::<T![Ident]>()?;
		let colon = p.parse::<T![:]>()?;
		let c: Cursor = name.into();
		let value = <V>::parse_declaration_value(p, c)?;
		let important = p.parse_if_peek::<BangImportant>()?;
		let semicolon = p.parse_if_peek::<T![;]>()?;
		Ok(Self { name, colon, value, important, semicolon, _phantom: PhantomData })
	}
}

impl<'a, V: DeclarationValue<'a>> ToCursors for Declaration<'a, V> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.name, s);
		ToCursors::to_cursors(&self.colon, s);
		ToCursors::to_cursors(&self.value, s);
		ToCursors::to_cursors(&self.important, s);
		ToCursors::to_cursors(&self.semicolon, s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;

	#[derive(Debug, ToSpan)]
	struct Decl(T![Ident]);
	impl<'a> DeclarationValue<'a> for Decl {
		type ComputedValue = T![Eof];

		fn is_initial(&self) -> bool {
			false
		}

		fn is_inherit(&self) -> bool {
			false
		}

		fn is_unset(&self) -> bool {
			false
		}

		fn is_revert(&self) -> bool {
			false
		}

		fn is_revert_layer(&self) -> bool {
			false
		}

		fn needs_computing(&self) -> bool {
			false
		}

		fn parse_specified_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
			p.parse::<T![Ident]>().map(Self)
		}
	}
	impl ToCursors for Decl {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			ToCursors::to_cursors(&self.0, s);
		}
	}

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Declaration<Decl>>(), 80);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Declaration<Decl>, "color:black;");
	}
}
