use super::prelude::*;

/// <https://drafts.csswg.org/css-speech-1/#typedef-voice-family-age>
///
/// ```text,ignore
/// <age> = child | young | old
/// ```
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
#[derive(ToCursors, ToSpan, SemanticEq, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct GenericVoice {
	pub age: Option<VoiceAge>,
	pub gender: VoiceGender,
	pub variant: Option<T![Number]>,
}

impl<'a> Peek<'a> for GenericVoice {
	fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		VoiceAge::peek(p, c) || VoiceGender::peek(p, c)
	}
}

impl<'a> Parse<'a> for GenericVoice {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let age = p.parse_if_peek::<VoiceAge>()?;
		let gender = p.parse::<VoiceGender>()?;
		let variant = p.parse_if_peek::<T![Number]>()?;
		Ok(Self { age, gender, variant })
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<VoiceAge>(), 16);
		assert_eq!(std::mem::size_of::<VoiceGender>(), 16);
		assert_eq!(std::mem::size_of::<GenericVoice>(), 48);
	}

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
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, GenericVoice, "");
		assert_parse_error!(CssAtomSet::ATOMS, GenericVoice, "child");
		assert_parse_error!(CssAtomSet::ATOMS, GenericVoice, "auto");
	}
}
