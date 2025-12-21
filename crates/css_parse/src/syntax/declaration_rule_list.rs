use crate::{
	Declaration, DeclarationValue, Diagnostic, Kind, KindSet, NodeMetadata, NodeWithMetadata, Parse, Parser, Peek,
	Result, Span, T, ToCursors, ToSpan, token_macros,
};
use bumpalo::collections::Vec;

/// A generic struct for AST nodes representing a rule's block that is only capable of having child declarations or
/// at-rules. Qualified Rules are not allowed. This is defined as:
///
/// ```md
/// <declaration-rule-list>
///  │├─ "{" ─╮─╭─╮─ <declaration <D>> ─╮─╭─╮─ "}" ─╭─┤│
///           │ │ │                     │ │ ╰───────╯
///           │ │ ╰─ <R> ───────────────┤ │
///           │ ╰───────────────────────╯ │
///           ╰───────────────────────────╯
/// ```
///
/// `<D>` must implement the [Declaration][crate::Declaration] trait.
///
/// `<R>` should be an At-Rule. `<R>` is only parsed if an [AtKeyword][crate::token_macros::AtKeyword] can be peeked.
///
/// It is an [implementation of "declaration-rule-list"][1]. It includes an error tolerance in that the ending `}`
/// token can be omitted, if at the end of the file.
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-declaration-list
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DeclarationRuleList<'a, D, R, M>
where
	D: DeclarationValue<'a, M>,
	R: NodeWithMetadata<M>,
	M: NodeMetadata,
{
	pub open_curly: token_macros::LeftCurly,
	pub declarations: Vec<'a, Declaration<'a, D, M>>,
	pub at_rules: Vec<'a, R>,
	pub close_curly: Option<token_macros::RightCurly>,
	meta: M,
}

impl<'a, D, R, M> NodeWithMetadata<M> for DeclarationRuleList<'a, D, R, M>
where
	D: DeclarationValue<'a, M>,
	R: NodeWithMetadata<M>,
	M: NodeMetadata,
{
	fn metadata(&self) -> M {
		self.meta
	}
}

impl<'a, D, R, M> Peek<'a> for DeclarationRuleList<'a, D, R, M>
where
	D: DeclarationValue<'a, M>,
	R: NodeWithMetadata<M>,
	M: NodeMetadata,
{
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly]);
}

impl<'a, D, R, M> Parse<'a> for DeclarationRuleList<'a, D, R, M>
where
	D: DeclarationValue<'a, M>,
	R: NodeWithMetadata<M> + Parse<'a>,
	M: NodeMetadata,
	Declaration<'a, D, M>: Parse<'a>,
{
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let open_curly = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.bump());
		let mut at_rules = Vec::new_in(p.bump());
		let mut meta = Default::default();
		loop {
			if p.at_end() {
				return Ok(Self { open_curly, declarations, at_rules, meta, close_curly: None });
			}
			let close_curly = p.parse_if_peek::<T!['}']>()?;
			if close_curly.is_some() {
				return Ok(Self { open_curly, declarations, at_rules, meta, close_curly });
			}
			let c = p.peek_n(1);
			if <T![AtKeyword]>::peek(p, c) {
				at_rules.push(p.parse::<R>()?);
			} else if <T![Ident]>::peek(p, c) {
				let rule = p.parse::<Declaration<'a, D, M>>()?;
				meta = meta.merge(rule.metadata());
				declarations.push(rule);
			} else {
				Err(Diagnostic::new(p.next(), Diagnostic::unexpected))?;
			}
		}
	}
}

impl<'a, D, R, M> ToCursors for DeclarationRuleList<'a, D, R, M>
where
	D: DeclarationValue<'a, M> + ToCursors,
	R: NodeWithMetadata<M> + ToCursors,
	M: NodeMetadata,
{
	fn to_cursors(&self, s: &mut impl crate::CursorSink) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.declarations, s);
		ToCursors::to_cursors(&self.at_rules, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}

impl<'a, D, R, M> ToSpan for DeclarationRuleList<'a, D, R, M>
where
	D: DeclarationValue<'a, M> + ToSpan,
	R: NodeWithMetadata<M> + ToSpan,
	M: NodeMetadata,
{
	fn to_span(&self) -> Span {
		self.open_curly.to_span()
			+ if let Some(close) = self.close_curly {
				close.to_span()
			} else {
				self.declarations.to_span() + self.at_rules.to_span()
			}
	}
}
