use super::prelude::*;

/// <https://drafts.csswg.org/css-speech-1/#typedef-voice-pitch-semitones>
///
/// ```text,ignore
/// <semitones> = <dimension-token>
/// ```
#[node]
#[derive(
	IntoCursor, ToSpan, SemanticEq, Parse, Peek, ToCursors, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash,
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
#[metadata(node_kinds = Dimension)]
pub struct Semitones(#[atom(CssAtomSet::St)] T![Dimension]);

impl From<Semitones> for f32 {
	fn from(semitones: Semitones) -> Self {
		semitones.0.into()
	}
}

impl ToNumberValue for Semitones {
	fn to_number_value(&self) -> Option<f32> {
		Some((*self).into())
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_peek_false};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, Semitones, "1st");
		assert_parse!(CssAtomSet::ATOMS, Semitones, "-3st");
	}

	#[test]
	fn test_errors() {
		assert_peek_false!(CssAtomSet::ATOMS, Semitones, "1");
		assert_peek_false!(CssAtomSet::ATOMS, Semitones, "1db");
	}
}
