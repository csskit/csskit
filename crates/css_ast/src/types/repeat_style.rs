use css_lexer::Cursor;
use css_parse::{
	Build, CursorSink, Parse, Parser, Peek, Result as ParserResult, T, ToCursors, diagnostics, keyword_set,
};

// https://drafts.csswg.org/css-backgrounds-4/#background-repeat
// <repeat-style> = repeat-x | repeat-y | <repetition>{1,2}
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(rename_all = "kebab-case"))]
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
		let ident = p.parse::<T![Ident]>()?;
		let c: Cursor = ident.into();
		match p.parse_str_lower(c) {
			"repeat-x" => Ok(Self::RepeatX(<T![Ident]>::build(p, c))),
			"repeat-y" => Ok(Self::RepeatY(<T![Ident]>::build(p, c))),
			_ if <Repetition>::peek(p, c) => {
				let first = Repetition::build(p, c);
				let second = p.parse_if_peek::<Repetition>()?;
				Ok(Self::Repetition(first, second))
			}
			_ => Err(diagnostics::UnexpectedIdent(p.parse_str(c).into(), c.into()))?,
		}
	}
}

impl<'a> ToCursors for RepeatStyle {
	fn to_cursors(&self, s: &mut impl CursorSink) {
		match self {
			Self::RepeatX(c) => s.append(c.into()),
			Self::RepeatY(c) => s.append(c.into()),
			Self::Repetition(p1, p2) => {
				s.append(p1.into());
				if let Some(p2) = p2 {
					s.append(p2.into());
				}
			}
		}
	}
}

// https://drafts.csswg.org/css-backgrounds-4/#typedef-repetition
// <repetition> = repeat | space | round | no-repeat
keyword_set!(Repetition { Repeat: "repeat", Space: "space", Round: "round", NoRepeat: "no-repeat" });
