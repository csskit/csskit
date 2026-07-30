use super::prelude::*;
use crate::CalcableValue;

/// <https://drafts.csswg.org/css-speech-1/#typedef-voice-family-age>
///
/// ```text,ignore
/// <age> = child | young | old
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum VoiceAge {
	#[atom(CssAtomSet::Child)]
	Child(T![Ident]),
	#[atom(CssAtomSet::Young)]
	Young(T![Ident]),
	#[atom(CssAtomSet::Old)]
	Old(T![Ident]),
}

/// <https://drafts.csswg.org/css-speech-1/#typedef-voice-family-gender>
///
/// ```text,ignore
/// <gender> = male | female | neutral
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(skip))]
#[derive(csskit_derives::NodeWithMetadata)]
pub enum VoiceGender {
	#[atom(CssAtomSet::Male)]
	Male(T![Ident]),
	#[atom(CssAtomSet::Female)]
	Female(T![Ident]),
	#[atom(CssAtomSet::Neutral)]
	Neutral(T![Ident]),
}

/// <https://drafts.csswg.org/css-speech-1/#typedef-generic-voice>
///
/// ```text,ignore
/// <generic-voice> = <age>? <gender> <integer>?
/// ```
#[node]
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GenericVoice<'a> {
	pub age: Option<VoiceAge>,
	pub gender: VoiceGender,
	pub variant: Option<CalcableValue<'a, T![Number]>>,
}

impl<'a> Peek<'a> for GenericVoice<'a> {
	const PEEK_KINDSET: KindSet = VoiceAge::PEEK_KINDSET.combine(VoiceGender::PEEK_KINDSET);

	#[inline(always)]
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		VoiceAge::peek(p, c) || VoiceGender::peek(p, c)
	}
}

impl<'a> Parse<'a> for GenericVoice<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let age = p.parse_if_peek::<VoiceAge>()?;
		let gender = p.parse::<VoiceGender>()?;
		let variant = p.parse_if_peek::<CalcableValue<T![Number]>>()?;
		Ok(Self { age, gender, variant })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_parses() {
		assert_parse!(CssAtomSet::ATOMS, GenericVoice, "male");
		assert_parse!(CssAtomSet::ATOMS, GenericVoice, "female");
		assert_parse!(CssAtomSet::ATOMS, GenericVoice, "neutral");
		assert_parse!(CssAtomSet::ATOMS, GenericVoice, "child male");
		assert_parse!(CssAtomSet::ATOMS, GenericVoice, "young female 2");
		assert_parse!(CssAtomSet::ATOMS, GenericVoice, "old neutral 1");
	}

	#[test]
	fn test_substitution() {
		assert_parse!(CssAtomSet::ATOMS, GenericVoice, "male var(--n)");
		assert_parse!(CssAtomSet::ATOMS, GenericVoice, "young female calc(1 + 1)");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, GenericVoice, "");
		assert_parse_error!(CssAtomSet::ATOMS, GenericVoice, "child");
		assert_peek_false!(CssAtomSet::ATOMS, GenericVoice, "auto");
	}
}
