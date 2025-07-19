use crate::{CursorSink, Declaration, DeclarationValue, Parse, Parser, Peek, Result, T, ToCursors, token_macros};
use bumpalo::collections::Vec;
use css_lexer::Kind;
use csskit_derives::ToSpan;

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
#[derive(ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct DeclarationList<'a, V: DeclarationValue<'a>> {
	pub open_curly: token_macros::LeftCurly,
	pub declarations: Vec<'a, Declaration<'a, V>>,
	pub close_curly: Option<token_macros::RightCurly>,
}

impl<'a, V: DeclarationValue<'a>> Peek<'a> for DeclarationList<'a, V> {
	const PEEK_KINDSET: css_lexer::KindSet = css_lexer::KindSet::new(&[Kind::LeftCurly]);
}

impl<'a, V: DeclarationValue<'a>> Parse<'a> for DeclarationList<'a, V> {
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		let open_curly = p.parse::<T!['{']>()?;
		let mut declarations = Vec::new_in(p.bump());
		loop {
			if p.at_end() {
				return Ok(Self { open_curly, declarations, close_curly: None });
			}
			let close_curly = p.parse_if_peek::<T!['}']>()?;
			if close_curly.is_some() {
				return Ok(Self { open_curly, declarations, close_curly });
			}
			let declaration = p.parse::<Declaration<V>>()?;
			declarations.push(declaration);
		}
	}
}

impl<'a, V: DeclarationValue<'a>> ToCursors for DeclarationList<'a, V> {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.open_curly, s);
		ToCursors::to_cursors(&self.declarations, s);
		ToCursors::to_cursors(&self.close_curly, s);
	}
}
