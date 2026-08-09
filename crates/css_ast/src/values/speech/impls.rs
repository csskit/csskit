use super::{VoicePitchStyleValue, VoiceRangeStyleValue};
use crate::{CalcableValue, CssAtomSet, Frequency, KeywordValue, NonNegative, Percentage, Semitones, Value};
use css_parse::{Cursor, Diagnostic, Parse, Parser, Peek, Result as ParseResult, T};

macro_rules! voice_pitch_parse {
	($ty:ident) => {
		impl<'a> Parse<'a> for $ty<'a> {
			fn parse<I>(p: &mut Parser<'a, I>) -> ParseResult<Self>
			where
				I: Iterator<Item = Cursor> + Clone,
			{
				let c = p.peek_n(1);
				if <T![Ident]>::peek(p, c) && p.equals_atom(c, &CssAtomSet::Absolute) {
					let absolute = p.parse::<KeywordValue<'a, T![Ident]>>()?;
					return Ok(Self::FrequencyAbsolute(
						p.parse::<CalcableValue<'a, NonNegative<Frequency>>>()?,
						absolute,
					));
				}
				let mut keyword = if <T![Ident]>::peek(p, c)
					&& matches!(
						p.to_atom::<CssAtomSet>(c),
						CssAtomSet::XLow | CssAtomSet::Low | CssAtomSet::Medium | CssAtomSet::High | CssAtomSet::XHigh
					) {
					Some((p.to_atom::<CssAtomSet>(c), p.parse::<KeywordValue<'a, T![Ident]>>()?))
				} else {
					None
				};
				let mut frequency = None;
				let mut semitones = None;
				let mut percentage = None;
				if p.peek::<CalcableValue<'a, NonNegative<Frequency>>>() {
					frequency = Some(p.parse::<CalcableValue<'a, NonNegative<Frequency>>>()?);
				} else if p.peek::<Value<'a, Semitones>>() {
					semitones = Some(p.parse::<Value<'a, Semitones>>()?);
				} else if p.peek::<CalcableValue<'a, Percentage>>() {
					percentage = Some(p.parse::<CalcableValue<'a, Percentage>>()?);
				}
				let c = p.peek_n(1);
				if keyword.is_none() && <T![Ident]>::peek(p, c) {
					if frequency.is_some() && p.equals_atom(c, &CssAtomSet::Absolute) {
						let absolute = p.parse::<KeywordValue<'a, T![Ident]>>()?;
						return Ok(Self::FrequencyAbsolute(frequency.expect("checked above"), absolute));
					}
					if matches!(
						p.to_atom::<CssAtomSet>(c),
						CssAtomSet::XLow | CssAtomSet::Low | CssAtomSet::Medium | CssAtomSet::High | CssAtomSet::XHigh
					) {
						keyword = Some((p.to_atom::<CssAtomSet>(c), p.parse::<KeywordValue<'a, T![Ident]>>()?));
					}
				}
				let (atom, keyword) = match keyword {
					Some((atom, keyword)) => (atom, Some(keyword)),
					None if frequency.is_none() && semitones.is_none() && percentage.is_none() => {
						Err(Diagnostic::new(c, Diagnostic::unexpected))?
					}
					None => (CssAtomSet::XLow, None),
				};
				Ok(match atom {
					CssAtomSet::XLow if semitones.is_some() => Self::XLow1 { x_low: keyword, semitones },
					CssAtomSet::XLow if percentage.is_some() => Self::XLow2 { x_low: keyword, percentage },
					CssAtomSet::XLow => Self::XLow { x_low: keyword, frequency },
					CssAtomSet::Low if semitones.is_some() => Self::Low1 { low: keyword, semitones },
					CssAtomSet::Low if percentage.is_some() => Self::Low2 { low: keyword, percentage },
					CssAtomSet::Low => Self::Low { low: keyword, frequency },
					CssAtomSet::Medium if semitones.is_some() => Self::Medium1 { medium: keyword, semitones },
					CssAtomSet::Medium if percentage.is_some() => Self::Medium2 { medium: keyword, percentage },
					CssAtomSet::Medium => Self::Medium { medium: keyword, frequency },
					CssAtomSet::High if semitones.is_some() => Self::High1 { high: keyword, semitones },
					CssAtomSet::High if percentage.is_some() => Self::High2 { high: keyword, percentage },
					CssAtomSet::High => Self::High { high: keyword, frequency },
					CssAtomSet::XHigh if semitones.is_some() => Self::XHigh1 { x_high: keyword, semitones },
					CssAtomSet::XHigh if percentage.is_some() => Self::XHigh2 { x_high: keyword, percentage },
					CssAtomSet::XHigh => Self::XHigh { x_high: keyword, frequency },
					_ => Err(Diagnostic::new(c, Diagnostic::unexpected))?,
				})
			}
		}
	};
}

voice_pitch_parse!(VoicePitchStyleValue);
voice_pitch_parse!(VoiceRangeStyleValue);

#[cfg(test)]
mod tests {
	use super::super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error, assert_peek_false};

	#[test]
	fn test_voice_family() {
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyStyleValue, "preserve");
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyStyleValue, "male");
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyStyleValue, "female");
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyStyleValue, "neutral");
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyStyleValue, "child male");
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyStyleValue, "old neutral");
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyStyleValue, "Alice");
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyStyleValue, "\"Alice\"");
		assert_parse!(CssAtomSet::ATOMS, VoiceFamilyStyleValue, "Alice,male");
		assert_peek_false!(CssAtomSet::ATOMS, VoiceFamilyStyleValue, "");
	}

	#[test]
	fn test_voice_volume() {
		assert_parse!(CssAtomSet::ATOMS, VoiceVolumeStyleValue, "silent");
		assert_parse!(CssAtomSet::ATOMS, VoiceVolumeStyleValue, "x-soft");
		assert_parse!(CssAtomSet::ATOMS, VoiceVolumeStyleValue, "soft");
		assert_parse!(CssAtomSet::ATOMS, VoiceVolumeStyleValue, "medium");
		assert_parse!(CssAtomSet::ATOMS, VoiceVolumeStyleValue, "loud");
		assert_parse!(CssAtomSet::ATOMS, VoiceVolumeStyleValue, "x-loud");
		assert_parse!(CssAtomSet::ATOMS, VoiceVolumeStyleValue, "soft 6db");
		assert_parse!(CssAtomSet::ATOMS, VoiceVolumeStyleValue, "loud -3db");
		assert_peek_false!(CssAtomSet::ATOMS, VoiceVolumeStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, VoiceVolumeStyleValue, "1px");
	}

	#[test]
	fn test_voice_rate() {
		assert_parse!(CssAtomSet::ATOMS, VoiceRateStyleValue, "normal");
		assert_parse!(CssAtomSet::ATOMS, VoiceRateStyleValue, "x-slow");
		assert_parse!(CssAtomSet::ATOMS, VoiceRateStyleValue, "slow");
		assert_parse!(CssAtomSet::ATOMS, VoiceRateStyleValue, "medium");
		assert_parse!(CssAtomSet::ATOMS, VoiceRateStyleValue, "fast");
		assert_parse!(CssAtomSet::ATOMS, VoiceRateStyleValue, "x-fast");
		assert_parse!(CssAtomSet::ATOMS, VoiceRateStyleValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, VoiceRateStyleValue, "normal 50%");
		assert_peek_false!(CssAtomSet::ATOMS, VoiceRateStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, VoiceRateStyleValue, "1px");
	}

	#[test]
	fn test_voice_pitch() {
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "absolute 200hz");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "200hz absolute");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "x-low");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "low");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "medium");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "high");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "x-high");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "medium 6st");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "high -2st");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "low 20hz");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "50%");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "6st medium");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "20hz");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "6st");
		assert_parse!(CssAtomSet::ATOMS, VoicePitchStyleValue, "calc(20hz*2)");
		assert_peek_false!(CssAtomSet::ATOMS, VoicePitchStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, VoicePitchStyleValue, "1px");
		assert_parse_error!(CssAtomSet::ATOMS, VoicePitchStyleValue, "absolute -200hz");
	}

	#[test]
	fn test_voice_range() {
		assert_parse!(CssAtomSet::ATOMS, VoiceRangeStyleValue, "absolute 200hz");
		assert_parse!(CssAtomSet::ATOMS, VoiceRangeStyleValue, "x-low");
		assert_parse!(CssAtomSet::ATOMS, VoiceRangeStyleValue, "x-high 30%");
		assert_parse!(CssAtomSet::ATOMS, VoiceRangeStyleValue, "6st");
		assert_peek_false!(CssAtomSet::ATOMS, VoiceRangeStyleValue, "");
		assert_peek_false!(CssAtomSet::ATOMS, VoiceRangeStyleValue, "1px");
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CueBeforeStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, CueBeforeStyleValue, "url(foo)");
		assert_parse!(CssAtomSet::ATOMS, CueBeforeStyleValue, "url(foo)20db");
		assert_parse!(CssAtomSet::ATOMS, CueAfterStyleValue, "none");
		assert_parse!(CssAtomSet::ATOMS, CueAfterStyleValue, "url(foo)");
		assert_parse!(CssAtomSet::ATOMS, CueAfterStyleValue, "url(foo)20db");
	}
}
