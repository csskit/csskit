use crate::{Cursor, CursorSink, Parse, Parser, Peek, Result, T, ToCursors, ToSpan, token_macros};
use bumpalo::collections::Vec;

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct RuleList<'a, R> {
	pub open_curly: token_macros::LeftCurly,
	pub rules: Vec<'a, R>,
	pub close_curly: Option<token_macros::RightCurly>,
}

impl<'a, R> Peek<'a> for RuleList<'a, R> {
	fn peek<Iter>(p: &Parser<'a, Iter>, c: Cursor) -> bool
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		<token_macros::LeftCurly>::peek(p, c)
	}
}

impl<'a, R: Parse<'a>> Parse<'a> for RuleList<'a, R> {
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let open_curly = p.parse::<T!['{']>()?;
		let mut rules = Vec::new_in(p.bump());
		loop {
			p.parse_if_peek::<T![;]>().ok();
			if p.at_end() {
				return Ok(Self { open_curly, rules, close_curly: None });
			}
			let close_curly = p.parse_if_peek::<T!['}']>()?;
			if close_curly.is_some() {
				return Ok(Self { open_curly, rules, close_curly });
			}
			rules.push(p.parse::<R>()?);
		}
	}
}

impl<'a, R: ToCursors> ToCursors for RuleList<'a, R> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.rules, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}

impl<'a, R: ToSpan> ToSpan for RuleList<'a, R> {
	fn to_span(&self) -> css_lexer::Span {
		self.open_curly.to_span()
			+ if let Some(close) = self.close_curly { close.to_span() } else { self.rules.to_span() }
	}
}
