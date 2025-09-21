use super::prelude::*;

/// <https://drafts.csswg.org/css-backgrounds-4/#background-repeat>
///
/// ```text,ignore
/// <repeat-style> = repeat-x | repeat-y | <repetition>{1,2}
/// ```
#[derive(ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum RepeatStyle {
	RepeatX(T![Ident]),
	RepeatY(T![Ident]),
	Repetition(Repetition, Option<Repetition>),
}

impl<'a> Peek<'a> for RepeatStyle {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<Repetition>::peek(p, c)
			|| (<T![Ident]>::peek(p, c)
				&& matches!(p.to_atom::<CssAtomSet>(c), CssAtomSet::RepeatX | CssAtomSet::RepeatY))
	}
}

impl<'a> Parse<'a> for RepeatStyle {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let checkpoint = p.checkpoint();
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		match p.to_atom::<CssAtomSet>(c) {
			CssAtomSet::RepeatX => Ok(Self::RepeatX(ident)),
			CssAtomSet::RepeatY => Ok(Self::RepeatY(ident)),
			_ => {
				p.rewind(checkpoint);
				let first = p.parse::<Repetition>()?;
				let second = p.parse_if_peek::<Repetition>()?;
				Ok(Self::Repetition(first, second))
			}
		}
	}
}

/// <https://drafts.csswg.org/css-backgrounds-4/#typedef-repetition>
///
/// ```text,ignore
/// <repetition> = repeat | space | round | no-repeat
/// ```
#[derive(Parse, Peek, IntoCursor, ToCursors, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum Repetition {
	#[atom(CssAtomSet::Repeat)]
	Repeat(T![Ident]),
	#[atom(CssAtomSet::Space)]
	Space(T![Ident]),
	#[atom(CssAtomSet::Round)]
	Round(T![Ident]),
	#[atom(CssAtomSet::NoRepeat)]
	NoRepeat(T![Ident]),
}
