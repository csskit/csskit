use crate::{
	CursorSink, DeclarationValue, Kind, KindSet, Parse, Parser, Peek, Result, Span, State, T, ToCursors, ToSpan,
	token_macros,
};
use bumpalo::collections::Vec;

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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Block<'a, D, R>
where
	D: DeclarationValue<'a>,
{
	pub open_curly: token_macros::LeftCurly,
	pub declarations: Vec<'a, Declaration<'a, D>>,
	pub rules: Vec<'a, R>,
	pub close_curly: Option<token_macros::RightCurly>,
}

impl<'a, D, R> Peek<'a> for Block<'a, D, R>
where
	D: DeclarationValue<'a>,
{
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::LeftCurly]);
}

impl<'a, D, R> Parse<'a> for Block<'a, D, R>
where
	D: DeclarationValue<'a>,
	R: Parse<'a>,
{
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
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
			let c = p.peek_n(1);
			if <T!['}']>::peek(p, c) {
				break;
			}
			let old_state = p.set_state(State::Nested);
			if <T![AtKeyword]>::peek(p, c) {
				let rule = p.parse::<R>();
				p.set_state(old_state);
				rules.push(rule?);
			} else if let Ok(Some(decl)) = p.try_parse_if_peek::<Declaration<'a, D>>() {
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
	D: DeclarationValue<'a> + ToCursors,
	R: ToCursors,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.declarations, s);
		ToCursors::to_cursors(&self.rules, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}

impl<'a, D, R> ToSpan for Block<'a, D, R>
where
	D: DeclarationValue<'a> + ToSpan,
	R: ToSpan,
{
	fn to_span(&self) -> Span {
		self.open_curly.to_span()
			+ if self.close_curly.is_some() {
				self.close_curly.to_span()
			} else {
				self.declarations.to_span() + self.rules.to_span() + self.close_curly.to_span()
			}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::EmptyAtomSet;
	use crate::{Cursor, test_helpers::*};

	#[derive(Debug)]
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

		fn parse_specified_declaration_value<Iter>(p: &mut Parser<'a, Iter>, _: Cursor) -> Result<Self>
		where
			Iter: Iterator<Item = crate::Cursor> + Clone,
		{
			p.parse::<T![Ident]>().map(Self)
		}
	}

	impl ToCursors for Decl {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			ToCursors::to_cursors(&self.0, s);
		}
	}

	impl ToSpan for Decl {
		fn to_span(&self) -> Span {
			self.0.to_span()
		}
	}

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Block<Decl, T![Ident]>>(), 96);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, Block<Decl, T![Ident]>, "{color:black}");
	}
}
