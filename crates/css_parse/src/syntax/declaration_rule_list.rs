use crate::{Declaration, DeclarationValue, Parse, Parser, Peek, Result, T, ToCursors, diagnostics, token_macros};
use bumpalo::collections::Vec;
use css_lexer::{Kind, KindSet, ToSpan};
use csskit_derives::ToSpan;

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
/// `<R>` may make use of the [AtRule][crate::AtRule] type but there is no requirement for this, but the parse steps
/// will only parse `<R>` if an [AtKeyword][crate::token_macros::AtKeyword] can be peeked.
///
/// It is an [implementation of "declaration-rule-list"][1]. It includes an error tolerance in that the ending `}`
/// token can be omitted, if at the end of the file.
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#typedef-declaration-list
#[derive(ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct DeclarationRuleList<'a, D, R>
where
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
{
	pub open_curly: token_macros::LeftCurly,
	pub declarations: Vec<'a, Declaration<'a, D>>,
	pub at_rules: Vec<'a, R>,
	pub close_curly: Option<token_macros::RightCurly>,
}

impl<'a, D, R> Peek<'a> for DeclarationRuleList<'a, D, R>
where
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
{
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly]);
}

impl<'a, D, R> Parse<'a> for DeclarationRuleList<'a, D, R>
where
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
{
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let open_curly = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.bump());
		let mut at_rules = Vec::new_in(p.bump());
		loop {
			if p.at_end() {
				return Ok(Self { open_curly, declarations, at_rules, close_curly: None });
			}
			let close_curly = p.parse_if_peek::<T!['}']>()?;
			if close_curly.is_some() {
				return Ok(Self { open_curly, declarations, at_rules, close_curly });
			}
			if p.peek::<T![AtKeyword]>() {
				at_rules.push(p.parse::<R>()?);
			} else if p.peek::<T![Ident]>() {
				let rule = p.parse::<Declaration<D>>()?;
				declarations.push(rule);
			} else {
				let c = p.peek_n(1);
				Err(diagnostics::Unexpected(c.into(), c.into()))?;
			}
		}
	}
}

impl<'a, D, R> ToCursors for DeclarationRuleList<'a, D, R>
where
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
{
	fn to_cursors(&self, s: &mut impl crate::CursorSink) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.declarations, s);
		ToCursors::to_cursors(&self.at_rules, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}
