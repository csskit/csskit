use crate::{CursorSink, DeclarationValue, Parse, Parser, Peek, Result as ParserResult, T, ToCursors};
use bumpalo::collections::{Vec, vec::IntoIter};

/// This is a generic type that can be used for AST nodes representing a [Declaration][1], aka "property".
///
/// An ident is parsed first, as the property name, followed by a `:`. After this the given `<T>` will be parsed as the
/// style value. Parsing may continue to swallow a `!important`, or the optional trailing semi `;`, if either are
/// present.
///
/// The grammar of `<T>` isn't defined here - it'll be dependant on the property name. Consequently, `<T>` must
/// implement the [DeclarationValue] trait, which must provide the
/// `parse_declaration_value(&mut Parser<'a>, Cursor) -> Result<Self>` method - the [Cursor] given to said method
/// represents the Ident of the property name, so it can be reasoned about in order to dispatch to the right
/// declaration value parsing step.
///
/// Also provided is a [Declaration::valid_property()] method. It defaults to returning `true`, which means
/// all property-ids are valid. If implementing a set of declarations where ony limited property-ids are valid (such as
/// the declarations allowed by an at-rule) then it might be worthwhile changing this to sometimes return `false`, which
/// will cause `parse_declaration` to error early without having to do too much backtracking.
///
/// The steps `parse` takes can be defined as:
///
/// ```md
/// <property-id>
///  │├─ <ident> ─┤│
///
/// <declaration>
///  │├─ <property-id> ─ ":" ─ <T> ──╮─────────────────────────────╭──╮───────╭┤│
///                                  ╰─ "!" ─ <ident "important"> ─╯  ╰─ ";" ─╯
/// ```
///
#[derive(ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type", rename = "property"))]
pub struct Declaration<T: DeclarationValue> {
	pub name: T![Ident],
	pub colon: T![:],
	pub value: StyleValue<'a>,
	pub important: Option<BangImportant>,
	pub semicolon: Option<T![;]>,
}

impl<'a> Peek<'a> for Property<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c) && p.peek_n(2) == Kind::Colon
	}
}

impl<'a> Parse<'a> for Property<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let name = p.parse::<T![Ident]>()?;
		let c: Cursor = name.into();
		if !Self::valid_property(p, c) {
			Err(diagnostics::UnknownDeclaration(c.into()))?;
		}
		let colon = p.parse::<T![:]>()?;
		let value = Self::DeclarationValue::parse_declaration_value(p, c)?;
		let important = p.parse_if_peek::<BangImportant>()?;
		let semicolon = p.parse_if_peek::<T![;]>()?;
		Ok(Self { name, colon, value, important, semicolon })
	}
}
