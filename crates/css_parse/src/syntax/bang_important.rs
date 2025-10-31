use crate::{
	Cursor, CursorSink, Diagnostic, Kind, Parse, Parser, Peek, Result, SemanticEq, Span, T, ToCursors, ToSpan,
};

/// Represents a two tokens, the first being [Kind::Delim] where the char is `!`, and the second being an `Ident` with
/// the value `important`. [CSS defines this as]:
///
/// ```md
/// <ws*>
///     ╭──────────────────────────╮
///  │├─╯─╭─ <whitespace-token> ─╮─╰─┤│
///       ╰──────────────────────╯
///
/// <!important>
///  │├─ "!" ─ <ws*> ─ <ident-token "important"> ─ <ws*> ─┤│
/// ```
///
/// `<ws*>` is any number of `<whitespace-token>`s, defined as [Kind::Whitespace][Kind::Whitespace]. This is
/// automatically skipped by default in the [Parser] anyway.
///
/// [1]: https://drafts.csswg.org/css-syntax-3/#!important-diagram
///
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct BangImportant {
	pub bang: T![!],
	pub important: T![Ident],
}

impl<'a> Peek<'a> for BangImportant {
	fn peek<Iter>(p: &Parser<'a, Iter>, c: Cursor) -> bool
	where
		Iter: Iterator<Item = Cursor> + Clone,
	{
		if c == Kind::Delim && c == '!' {
			let c = p.peek_n(2);
			c == Kind::Ident && p.to_source_cursor(c).eq_ignore_ascii_case("important")
		} else {
			false
		}
	}
}

impl<'a> Parse<'a> for BangImportant {
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = Cursor> + Clone,
	{
		let bang = p.parse::<T![!]>()?;
		let important = p.parse::<T![Ident]>()?;
		if !p.to_source_cursor(important.into()).eq_ignore_ascii_case("important") {
			Err(Diagnostic::new(important.into(), Diagnostic::unexpected_ident))?
		}
		Ok(Self { bang, important })
	}
}

impl ToCursors for BangImportant {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		s.append(self.bang.into());
		s.append(self.important.into());
	}
}

impl ToSpan for BangImportant {
	fn to_span(&self) -> Span {
		self.bang.to_span() + self.important.to_span()
	}
}

impl SemanticEq for BangImportant {
	fn semantic_eq(&self, _: &Self) -> bool {
		// The presence of !important is semantic in of itself, so this is just always true
		true
	}
}
