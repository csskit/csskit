use crate::{CursorSink, Parse, Parser, Peek, Result, T, ToCursors, token_macros};
use bumpalo::collections::Vec;
use css_lexer::ToSpan;
use csskit_derives::ToSpan;

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
#[derive(ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct RuleList<'a, R>
where
	R: Parse<'a> + ToCursors + ToSpan,
{
	pub open_curly: token_macros::LeftCurly,
	pub rules: Vec<'a, R>,
	pub close_curly: Option<token_macros::RightCurly>,
}

impl<'a, R> Peek<'a> for RuleList<'a, R>
where
	R: Parse<'a> + ToCursors + ToSpan,
{
	fn peek(p: &Parser<'a>, c: css_lexer::Cursor) -> bool {
		<token_macros::LeftCurly>::peek(p, c)
	}
}

impl<'a, R: Parse<'a> + ToCursors + ToSpan> Parse<'a> for RuleList<'a, R> {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
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

impl<'a, R> ToCursors for RuleList<'a, R>
where
	R: Parse<'a> + ToCursors + ToSpan,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.rules, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}
