use super::prelude::*;
use crate::{RuleVariants, token_macros};

/// A struct representing an AST node block that only accepts child "Rules". This is defined as:
///
/// ```md
/// <rule-list>
///  │├─ "{" ─╭─ <R> ─╮─╮─ "}" ─╭──┤│
///           ╰───────╯ ╰───────╯
/// ```
///
/// This is an implementation of [`<at-rule-list>`][1] or [`<qualified-rule-list>`][2].
///
/// It simply parses the open `{` and iterates collecing `<R>`s until the closing `}`.
///
/// Every item in the list must implement the [Parse], [ToCursors] and [ToSpan] traits.
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-at-rule-list
/// [2]: https://drafts.csswg.org/css-syntax-3/#typedef-qualified-rule-list
#[node]
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(bound(serialize = "R: serde::Serialize, M: serde::Serialize"))
)]
pub struct RuleList<'a, R, M>
where
	R: NodeWithMetadata<M>,
	M: NodeMetadata,
{
	pub open_curly: token_macros::LeftCurly,
	pub rules: Vec<'a, R>,
	pub close_curly: token_macros::RightCurly,
	#[cfg_attr(feature = "serde", serde(skip))]
	pub meta: M,
}

impl<'a, R, M> Peek<'a> for RuleList<'a, R, M>
where
	R: NodeWithMetadata<M>,
	M: NodeMetadata,
{
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly]);
}

impl<'a, R, M> Parse<'a> for RuleList<'a, R, M>
where
	R: Parse<'a> + NodeWithMetadata<M> + RuleVariants<'a>,
	M: NodeMetadata,
{
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let nested = p.is(State::Nested);
		let open_curly = p.parse::<T!['{']>()?;
		let mut rules = Vec::new_in(p.alloc());
		let mut meta = if nested { M::default().with_nested() } else { M::default() };
		loop {
			let cursor = p.peek_n(1);
			if <T![;]>::peek(p, cursor) {
				let old_skip = p.set_skip(KindSet::TRIVIA.add(Kind::Semicolon));
				p.consume_trivia_as_leading();
				p.set_skip(old_skip);
				continue;
			}
			if p.at_end() {
				let close_curly = <T!['}']>::missing(p);
				return Ok(Self { open_curly, rules, close_curly, meta });
			}
			if let Some(close_curly) = p.parse_if_peek::<T!['}']>()? {
				return Ok(Self { open_curly, rules, close_curly, meta });
			}
			let old_stop = p.set_stop(KindSet::new(&[Kind::RightCurly]));
			let rule = p.parse::<R>();
			p.set_stop(old_stop);
			let rule = rule?;
			meta = meta.merge(rule.metadata());
			rules.push(rule);
		}
	}
}

impl<'a, R, M> ToCursors for RuleList<'a, R, M>
where
	R: ToCursors + NodeWithMetadata<M>,
	M: NodeMetadata,
{
	fn to_cursors(&self, s: &mut (impl CursorSink + ?Sized)) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.rules, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}

impl<'a, R, M> ToSpan for RuleList<'a, R, M>
where
	R: ToSpan + NodeWithMetadata<M>,
	M: NodeMetadata,
{
	fn to_span(&self) -> css_lexer::Span {
		self.open_curly.to_span() + self.close_curly.to_span()
	}
}

impl<'a, R, M> NodeWithMetadata<M> for RuleList<'a, R, M>
where
	R: NodeWithMetadata<M>,
	M: NodeMetadata,
{
	fn metadata(&self) -> M {
		self.meta
	}
}

impl<'a, R, M> SemanticEq for RuleList<'a, R, M>
where
	R: NodeWithMetadata<M> + SemanticEq,
	M: NodeMetadata,
{
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		self.open_curly.semantic_eq(&other.open_curly, source_text)
			&& self.rules.semantic_eq(&other.rules, source_text)
			&& self.close_curly.semantic_eq(&other.close_curly, source_text)
	}
}
