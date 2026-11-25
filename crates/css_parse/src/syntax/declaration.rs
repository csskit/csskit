use super::prelude::*;
use crate::{BangImportant, Box, token_macros};
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
/// A second `!important` is preserved for legacy browser compatibility.
///
/// The grammar of `<V>` isn't defined here - it'll be dependant on the property name. Consequently, `<V>` must
/// implement the [DeclarationValue] trait, which must provide the
/// `parse_declaration_value(&mut Parser<'a>, Cursor) -> Result<Self>` method - the [Cursor] given to said method
/// represents the Ident of the property name, so it can be reasoned about in order to dispatch to the right
/// declaration value parsing step.
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#consume-a-declaration
#[node]
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
	pub important: Option<Box<'a, DeclarationImportance>>,
	pub semicolon: Option<token_macros::Semicolon>,
	#[cfg_attr(feature = "serde", serde(skip))]
	_phantom: PhantomData<(&'a (), M)>,
}

/// The `!important` suffix of a declaration, including the legacy form with two flags.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(untagged))]
pub enum DeclarationImportance {
	Single(BangImportant),
	Duplicate([BangImportant; 2]),
}

impl DeclarationImportance {
	/// The flags in source order.
	pub fn flags(&self) -> &[BangImportant] {
		match self {
			Self::Single(flag) => std::slice::from_ref(flag),
			Self::Duplicate(flags) => flags,
		}
	}
}

impl ToCursors for DeclarationImportance {
	fn to_cursors(&self, s: &mut (impl CursorSink + ?Sized)) {
		for flag in self.flags() {
			flag.to_cursors(s);
		}
	}
}

impl ToSpan for DeclarationImportance {
	fn to_span(&self) -> Span {
		self.flags().to_span()
	}
}

impl SemanticEq for DeclarationImportance {
	fn semantic_eq(&self, other: &Self, _: &str) -> bool {
		self.flags().len() == other.flags().len()
	}
}

impl<'a, V, M> Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	pub fn is_unknown(&self) -> bool {
		self.value.is_unknown()
	}

	/// Whether the declaration uses the legacy two-flag suffix.
	pub fn has_duplicate_important(&self) -> bool {
		matches!(self.important.as_deref(), Some(DeclarationImportance::Duplicate(_)))
	}
}

impl<'a, V, M> NodeWithMetadata<M> for Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn self_metadata(&self) -> M {
		V::declaration_self_metadata(self)
	}

	fn metadata(&self) -> M {
		V::declaration_metadata(self)
	}
}

impl<'a, V, M> Peek<'a> for Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Ident]);

	#[inline]
	fn peek<Iter>(p: &Parser<'a, Iter>, c: Cursor) -> bool
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		// A declaration must be an Ident followed by a Colon (with any number of whitespace inbetween). If that is not the
		// case then it definitely cannot be parsed as a Declaration.
		//
		// https://drafts.csswg.org/css-syntax-3/#consume-a-blocks-contents
		// ... "If the next non-whitespace token isn’t a <colon-token>, you can similarly immediately stop parsing as a
		// declaration." ... "(That is, font+ ... is guaranteed to not be a property"...
		if c != Kind::Ident || p.peek_n(2) != Kind::Colon {
			return false;
		}

		// https://drafts.csswg.org/css-syntax-3/#consume-a-blocks-contents
		// ... "If the first two non-whitespace tokens are a custom property name and a colon, it’s definitely a custom
		// property and won’t ever produce a valid rule" ... "(That is, --foo:hover {...} is guaranteed to be a custom
		// property, not a rule.)".
		if c.token().is_dashed_ident() {
			return true;
		}

		// If the third token is a `Colon` then it's likely a Pseudo Element selector. Colons are not valid value tokens
		// inside of a declaration at current, however this is _technically_ a non-standard affordance that may be removed
		// in future.
		if p.peek_n(3) == Kind::Colon {
			return false;
		}

		// https://drafts.csswg.org/css-syntax-3/#consume-a-blocks-contents
		// ... "If the first three non-whitespace tokens are a valid property name, a colon, and anything other than a
		// <{-token>, and then while parsing the declaration's value you encounter a <{-token>, you can immediately stop
		// parsing as a declaration and reparse as a rule instead.
		// (That is, font:bar {... is guaranteed to be an invalid property.)"
		if p.peek_n(4) == Kind::LeftCurly || p.peek_n(5) == Kind::LeftCurly {
			return false;
		}

		// All early checks have been exhausted, so the next step is to parse the Declaration to see if it is valid.
		true
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
		let checkpoint = p.checkpoint();
		let mut value = <V>::parse_declaration_value(p, c)?;
		let mut important = match p.parse_if_peek::<BangImportant>()? {
			Some(first) => Some(match p.parse_if_peek::<BangImportant>()? {
				Some(second) => DeclarationImportance::Duplicate([first, second]),
				None => DeclarationImportance::Single(first),
			}),
			None => None,
		};
		if !(p.next_is_stop() || p.peek_n(1) == KindSet::RIGHT_CURLY_SEMICOLON_OR_RIGHT_PAREN.add(Kind::Eof)) {
			p.rewind(checkpoint);
			value = <V>::parse_unknown_declaration_value(p, c)?;
			important = p.parse_if_peek::<BangImportant>()?.map(DeclarationImportance::Single);
		}
		let important = important.map(|flags| Box::new_in(p.alloc(), flags));
		let semicolon = p.parse_if_peek::<T![;]>()?;
		Ok(Self { name, colon, value, important, semicolon, _phantom: PhantomData })
	}
}

impl<'a, V, M> ToCursors for Declaration<'a, V, M>
where
	V: DeclarationValue<'a, M> + ToCursors,
	M: NodeMetadata,
{
	fn to_cursors(&self, s: &mut (impl CursorSink + ?Sized)) {
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
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		// Semicolon is not semantically relevant!
		self.name.semantic_eq(&other.name, source_text)
			&& self.value.semantic_eq(&other.value, source_text)
			&& self.important.semantic_eq(&other.important, source_text)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;
	use crate::{Arena, EmptyAtomSet, SemanticEq};

	#[derive(Debug)]
	struct Decl(T![Ident]);

	impl<M: NodeMetadata> NodeWithMetadata<M> for Decl {
		fn metadata(&self) -> M {
			M::default()
		}
	}

	impl<'a, M: NodeMetadata> DeclarationValue<'a, M> for Decl {
		fn parse_specified_declaration_value<Iter>(p: &mut Parser<'a, Iter>, _name: Cursor) -> Result<Self>
		where
			Iter: Iterator<Item = crate::Cursor> + Clone,
		{
			p.parse::<T![Ident]>().map(Self)
		}
	}

	impl ToCursors for Decl {
		fn to_cursors(&self, s: &mut (impl CursorSink + ?Sized)) {
			s.append(self.0.into())
		}
	}

	impl ToSpan for Decl {
		fn to_span(&self) -> Span {
			self.0.to_span()
		}
	}

	impl SemanticEq for Decl {
		fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
			self.0.semantic_eq(&other.0, source_text)
		}
	}

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, Declaration<Decl, ()>, "color:black;");
	}

	#[test]
	fn parses_duplicate_important_at_eof() {
		assert_parse!(
			EmptyAtomSet::ATOMS,
			Declaration<Decl, ()>,
			"color:black !important !important",
			|node| {
				assert!(node.has_duplicate_important());
				assert!(node.semicolon.is_none());
			}
		);
	}

	#[test]
	fn leaves_the_configured_stop_token_unconsumed() {
		let source = "color:black!important!important,";
		let alloc = Arena::new();
		let lexer = css_lexer::Lexer::new(&EmptyAtomSet::ATOMS, source);
		let mut parser = Parser::new(&alloc, source, lexer);
		parser.set_stop(KindSet::new(&[Kind::Comma]));

		let declaration = parser.parse::<Declaration<Decl, ()>>().unwrap();
		assert!(declaration.has_duplicate_important());
		assert_eq!(parser.peek_n(1), Kind::Comma);
	}

	#[test]
	fn rejects_garbage_after_duplicate_important_without_unknown_fallback() {
		let source = "color:black!important!important !foo;";
		let alloc = Arena::new();
		let lexer = css_lexer::Lexer::new(&EmptyAtomSet::ATOMS, source);
		let mut parser = Parser::new(&alloc, source, lexer);

		assert!(parser.parse::<Declaration<Decl, ()>>().is_err());
	}
}
