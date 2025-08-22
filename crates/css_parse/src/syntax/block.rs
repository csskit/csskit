use crate::{CursorSink, DeclarationValue, Parse, Parser, Peek, Result, State, T, ToCursors, token_macros};
use bumpalo::collections::Vec;
use css_lexer::{Kind, KindSet, ToSpan};
use csskit_derives::ToSpan;

use super::Declaration;

/// This trait provides an implementation for ["consuming a blocks contents"][1].
///
/// ```md
/// <block>
///
///  │├─ "{" ─╭──╮─╭─ <ws-*> ─╮─╭─╮─╭─ ";" ─╮─╭─╮─ <R> ─╭─╮─ "}" ─┤│
///           │  │ ╰──────────╯ │ │ ╰───────╯ │ ├─ <D> ─┤ │
///           │  ╰──────────────╯ ╰───────────╯ ╰───────╯ │
///           ╰───────────────────────────────────────────╯
/// ```
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#consume-block-contents
#[derive(ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Block<'a, D, R>
where
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
{
	pub open_curly: token_macros::LeftCurly,
	pub declarations: Vec<'a, Declaration<'a, D>>,
	pub rules: Vec<'a, R>,
	pub close_curly: Option<token_macros::RightCurly>,
}

impl<'a, D, R> Peek<'a> for Block<'a, D, R>
where
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
{
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly]);
}

impl<'a, D, R> Parse<'a> for Block<'a, D, R>
where
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
{
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let open_curly = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.bump());
		let mut rules = Vec::new_in(p.bump());
		loop {
			// While by default the parser will skip whitespace, the Declaration or Rule type may be a whitespace sensitive
			// node, for example `ComponentValues`. As such whitespace needs to be consumed here, before Declarations and
			// Rules are parsed.
			if p.parse_if_peek::<T![' ']>()?.is_some() || p.parse_if_peek::<T![;]>()?.is_some() {
				continue;
			}
			if p.at_end() {
				break;
			}
			if p.peek::<T!['}']>() {
				break;
			}
			let old_state = p.set_state(State::Nested);
			if p.peek::<T![AtKeyword]>() {
				let rule = p.parse::<R>();
				p.set_state(old_state);
				rules.push(rule?);
			} else if let Ok(Some(decl)) = p.try_parse_if_peek::<Declaration<D>>() {
				p.set_state(old_state);
				declarations.push(decl);
			} else {
				let rule = p.parse::<R>();
				p.set_state(old_state);
				rules.push(rule?);
			}
		}
		let close_curly = p.parse_if_peek::<T!['}']>()?;
		Ok(Self { open_curly, declarations, rules, close_curly })
	}
}

impl<'a, D, R> ToCursors for Block<'a, D, R>
where
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.declarations, s);
		ToCursors::to_cursors(&self.rules, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;
	use css_lexer::Cursor;

	#[derive(Debug, ToSpan)]
	struct Decl(T![Ident]);
	impl<'a> DeclarationValue<'a> for Decl {
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

		fn parse_specified_declaration_value(p: &mut Parser<'a>, _: Cursor) -> Result<Self> {
			p.parse::<T![Ident]>().map(Self)
		}
	}
	impl ToCursors for Decl {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			ToCursors::to_cursors(&self.0, s);
		}
	}

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Block<Decl, T![Ident]>>(), 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(Block<Decl, T![Ident]>, "{color:black}");
	}
}
