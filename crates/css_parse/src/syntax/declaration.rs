use crate::{
	BangImportant, Cursor, CursorSink, DeclarationValue, Kind, NodeMetadata, NodeWithMetadata, Parse, Parser, Peek,
	Result, SemanticEq, Span, T, ToCursors, ToSpan, token_macros,
};
use std::marker::PhantomData;

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	pub name: token_macros::Ident,
	pub colon: token_macros::Colon,
	pub value: V,
	pub important: Option<BangImportant>,
	pub semicolon: Option<token_macros::Semicolon>,
	#[cfg_attr(feature = "serde", serde(skip))]
	_phantom: PhantomData<&'a M>,
}

impl<'a, V, M> NodeWithMetadata<M> for Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn metadata(&self) -> M {
		DeclarationValue::declaration_metadata(self)
	}
}

impl<'a, V, M> Peek<'a> for Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn peek<Iter>(p: &Parser<'a, Iter>, c: Cursor) -> bool
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		c == Kind::Ident && p.peek_n(2) == Kind::Colon
	}
}

impl<'a, V, M> Parse<'a> for Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let name = p.parse::<T![Ident]>()?;
		let colon = p.parse::<T![:]>()?;
		let c: Cursor = name.into();
		let value = <V>::parse_declaration_value(p, c)?;
		let important = p.parse_if_peek::<BangImportant>()?;
		let semicolon = p.parse_if_peek::<T![;]>()?;
		Ok(Self { name, colon, value, important, semicolon, _phantom: PhantomData })
	}
}

impl<'a, V, M> ToCursors for Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M> + ToCursors,
	M: NodeMetadata,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.name, s);
		ToCursors::to_cursors(&self.colon, s);
		ToCursors::to_cursors(&self.value, s);
		ToCursors::to_cursors(&self.important, s);
		ToCursors::to_cursors(&self.semicolon, s);
	}
}

impl<'a, V, M> ToSpan for Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M> + ToSpan,
	M: NodeMetadata,
{
	fn to_span(&self) -> Span {
		self.name.to_span() + self.value.to_span() + self.important.to_span() + self.semicolon.to_span()
	}
}

impl<'a, V, M> SemanticEq for Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn semantic_eq(&self, other: &Self) -> bool {
		// Semicolon is not semantically relevant!
		self.name.semantic_eq(&other.name)
			&& self.value.semantic_eq(&other.value)
			&& self.important.semantic_eq(&other.important)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::EmptyAtomSet;
	use crate::SemanticEq;
	use crate::test_helpers::*;

	#[derive(Debug)]
	struct Decl(T![Ident]);

	impl<M: NodeMetadata> NodeWithMetadata<M> for Decl {
		fn metadata(&self) -> M {
			M::default()
		}
	}

	impl<'a, M: NodeMetadata> DeclarationValue<'a, M> for Decl {
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

		fn parse_specified_declaration_value<Iter>(p: &mut Parser<'a, Iter>, _name: Cursor) -> Result<Self>
		where
			Iter: Iterator<Item = crate::Cursor> + Clone,
		{
			p.parse::<T![Ident]>().map(Self)
		}
	}

	impl ToCursors for Decl {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			s.append(self.0.into())
		}
	}

	impl ToSpan for Decl {
		fn to_span(&self) -> Span {
			self.0.to_span()
		}
	}

	impl SemanticEq for Decl {
		fn semantic_eq(&self, other: &Self) -> bool {
			self.0.semantic_eq(&other.0)
		}
	}

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Declaration<Decl, ()>>(), 80);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, Declaration<Decl, ()>, "color:black;");
	}
}
