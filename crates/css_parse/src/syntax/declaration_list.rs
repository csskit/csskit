use crate::{
	CursorSink, Declaration, DeclarationValue, Kind, KindSet, NodeMetadata, NodeWithMetadata, Parse, Parser, Peek,
	Result, SemanticEq, Span, T, ToCursors, ToSpan, token_macros,
};
use bumpalo::collections::Vec;

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(feature = "serde", serde(bound(serialize = "V: serde::Serialize")))]
pub struct DeclarationList<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	pub open_curly: token_macros::LeftCurly,
	pub declarations: Vec<'a, Declaration<'a, V, M>>,
	pub close_curly: Option<token_macros::RightCurly>,
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
		let open_curly = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.bump());
		let mut meta = Default::default();
		loop {
			if p.at_end() {
				return Ok(Self { open_curly, declarations, close_curly: None, meta });
			}
			let close_curly = p.parse_if_peek::<T!['}']>()?;
			if close_curly.is_some() {
				return Ok(Self { open_curly, declarations, close_curly, meta });
			}
			let declaration = p.parse::<Declaration<'a, V, M>>()?;
			meta = meta.merge(declaration.metadata());
			declarations.push(declaration);
		}
	}
}

impl<'a, V, M> ToCursors for DeclarationList<'a, V, M>
where
	V: DeclarationValue<'a, M> + ToCursors,
	M: NodeMetadata,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
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
		self.open_curly.to_span()
			+ if let Some(close) = self.close_curly { close.to_span() } else { self.declarations.to_span() }
	}
}

impl<'a, V, M> SemanticEq for DeclarationList<'a, V, M>
where
	V: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn semantic_eq(&self, other: &Self) -> bool {
		self.open_curly.semantic_eq(&other.open_curly)
			&& self.declarations.semantic_eq(&other.declarations)
			&& self.close_curly.semantic_eq(&other.close_curly)
	}
}
