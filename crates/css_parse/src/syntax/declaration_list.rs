use super::prelude::*;
use crate::{BadDeclaration, Declaration, DeclarationOrBad, token_macros};

/// A generic struct that can be used for AST nodes representing a rule's block, that is only capable of having child
/// declarations.
///
/// It is an [implementation of "declaration-list"][1]. It includes an error tolerance in that the ending `}` token can
/// be omitted, if at the end of the file.
///
/// The `<V>` must implement the [DeclarationValue] trait, as it is passed to [Declaration].
///
/// ```md
/// <declaration-list>
///  │├─ "{" ─╮─╭─ <declaration> ──╮─╭─╮─ "}" ─╭─┤│
///           │ │                  │ │ ╰───────╯
///           │ ╰──────────────────╯ │
///           ╰──────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-declaration-list
#[node]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "V: serde::Serialize")))]
pub struct DeclarationList<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	pub open_curly: token_macros::LeftCurly,
	pub declarations: Vec<'a, DeclarationOrBad<'a, V, M>>,
	pub close_curly: token_macros::RightCurly,
	#[cfg_attr(feature = "serde", serde(skip))]
	meta: M,
}

impl<'a, V, M> NodeWithMetadata<M> for DeclarationList<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn metadata(&self) -> M {
		self.meta
	}
}

impl<'a, V, M> Peek<'a> for DeclarationList<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly]);
}

impl<'a, V, M> Parse<'a> for DeclarationList<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let nested = p.is(State::Nested);
		let open_curly = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.alloc());
		let mut meta: M = Default::default();
		if nested {
			meta = meta.with_nested();
		}
		loop {
			if p.peek_n(1) == Kind::Eof {
				meta = meta.with_size(declarations.len().min(u16::MAX as usize) as u16);
				let close_curly = <T!['}']>::missing(p);
				return Ok(Self { open_curly, declarations, close_curly, meta });
			}
			if let Some(close_curly) = p.parse_if_peek::<T!['}']>()? {
				meta = meta.with_size(declarations.len().min(u16::MAX as usize) as u16);
				return Ok(Self { open_curly, declarations, close_curly, meta });
			}
			let cursor = p.peek_n(1);
			if cursor == Kind::Semicolon {
				let old_skip = p.set_skip(KindSet::TRIVIA.add(Kind::Semicolon));
				p.consume_trivia_as_leading();
				p.set_skip(old_skip);
				continue;
			}
			if cursor == super::block::STRAY_KINDS {
				declarations.push(DeclarationOrBad::Bad(BadDeclaration::parse_stray(p)?));
				continue;
			}
			let checkpoint = p.checkpoint();
			match p.parse::<Declaration<'a, V, M>>() {
				Ok(declaration) => {
					meta = meta.merge(declaration.metadata());
					declarations.push(DeclarationOrBad::Declaration(declaration));
				}
				Err(err) => {
					p.rewind(checkpoint);
					let cursor = p.peek_n(1);
					let old_state = p.set_state(State::Nested);
					let bad = p.parse::<BadDeclaration>();
					p.set_state(old_state);
					let bad = bad.map_err(|_| err)?;
					p.diagnose(Diagnostic::new(cursor, Diagnostic::unexpected));
					declarations.push(DeclarationOrBad::Bad(bad));
				}
			}
		}
	}
}

impl<'a, V, M> ToCursors for DeclarationList<'a, V, M>
where
	V: DeclarationValue<'a, M> + ToCursors,
	M: NodeMetadata,
{
	fn to_cursors(&self, s: &mut (impl CursorSink + ?Sized)) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.declarations, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}

impl<'a, V, M> ToSpan for DeclarationList<'a, V, M>
where
	V: DeclarationValue<'a, M> + ToSpan,
	M: NodeMetadata,
{
	fn to_span(&self) -> Span {
		self.open_curly.to_span() + self.close_curly.to_span()
	}
}

impl<'a, V, M> SemanticEq for DeclarationList<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		self.open_curly.semantic_eq(&other.open_curly, source_text)
			&& self.declarations.semantic_eq(&other.declarations, source_text)
			&& self.close_curly.semantic_eq(&other.close_curly, source_text)
	}
}
