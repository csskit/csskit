use super::prelude::*;

/// <https://drafts.csswg.org/css-backgrounds-4/#background-repeat>
///
/// ```text,ignore
/// <repeat-style> = repeat-x | repeat-y | <repetition>{1,2}
/// ```
#[derive(ToCursors, ToSpan, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
#[visit(self)]
pub enum RepeatStyle {
	RepeatX(T![Ident]),
	RepeatY(T![Ident]),
	Repetition(Repetition, Option<Repetition>),
}

impl<'a> Peek<'a> for RepeatStyle {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<Repetition>::peek(p, c) || (<T![Ident]>::peek(p, c) && matches!(p.parse_str_lower(c), "repeat-x" | "repeat-y"))
	}
}

impl<'a> Parse<'a> for RepeatStyle {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		let c = p.peek_n(1);
		match p.parse_str_lower(c) {
			"repeat-x" => p.parse::<T![Ident]>().map(Self::RepeatX),
			"repeat-y" => p.parse::<T![Ident]>().map(Self::RepeatY),
			_ => {
				let first = p.parse::<Repetition>()?;
				let second = p.parse_if_peek::<Repetition>()?;
				Ok(Self::Repetition(first, second))
			}
		}
	}
}

keyword_set!(
	/// https://drafts.csswg.org/css-backgrounds-4/#typedef-repetition
	///
	/// ```text,ignore
	/// <repetition> = repeat | space | round | no-repeat
	/// ```
	#[derive(Visitable)]
	#[visit(skip)]
	pub enum Repetition {
		Repeat: "repeat",
		Space: "space",
		Round: "round",
		NoRepeat: "no-repeat"
	}
);
