use css_lexer::Cursor;
use css_parse::{Parser, Peek, T, keyword_set};
use csskit_derives::{Parse, ToCursors, ToSpan, Visitable};

use crate::SymbolsFunction;

#[derive(Parse, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub enum CounterStyle<'a> {
	#[visit(skip)]
	Predefined(PredefinedCounter),
	#[visit(skip)]
	Named(T![Ident]),
	SymbolsFunction(SymbolsFunction<'a>),
}

impl<'a> Peek<'a> for CounterStyle<'a> {
	fn peek(p: &Parser<'a>, c: Cursor) -> bool {
		<T![Ident]>::peek(p, c) || <SymbolsFunction>::peek(p, c)
	}
}

keyword_set!(
	// https://drafts.csswg.org/css-counter-styles-3/#predefined-counters
	pub enum PredefinedCounter {
		Decimal: "decimal",
		DecimalLeadingZero: "decimal-leading-zero",
		ArabicIndic: "arabic-indic",
		Armenian: "armenian",
		UpperArmenian: "upper-armenian",
		LowerArmenian: "lower-armenian",
		Bengali: "bengali",
		Cambodian: "cambodian",
		Khmer: "khmer",
		CjkDecimal: "cjk-decimal",
		Devanagari: "devanagari",
		Georgian: "georgian",
		Gujarati: "gujarati",
		Gurmukhi: "gurmukhi",
		Hebrew: "hebrew",
		Kannada: "kannada",
		Lao: "lao",
		Malayalam: "malayalam",
		Mongolian: "mongolian",
		Myanmar: "myanmar",
		Oriya: "oriya",
		Persian: "persian",
		LowerRoman: "lower-roman",
		UpperRoman: "upper-roman",
		Tamil: "tamil",
		Telugu: "telugu",
		Thai: "thai",
		Tibetan: "tibetan",
		LowerAlpha: "lower-alpha",
		UpperAlpha: "upper-alpha",
		UpperLatin: "upper-latin",
		LowerGreek: "lower-greek",
		Hiragana: "hiragana",
		HiraganaIroha: "hiragana-iroha",
		Katakana: "katakana",
		KatakanaIroha: "katakana-iroha",
		Disc: "disc",
		Square: "square",
		DisclousureOpen: "disclousure-open",
		DisclousureClosed: "disclousure-closed",
		CjkEarthlyBranch: "cjk-earthly-branch",
		CjkHeavenlyStem: "cjk-heavenly-stem",
	}
);

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CounterStyle>(), 80);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CounterStyle, "cjk-heavenly-stem");
		assert_parse!(CounterStyle, "foobar");
		assert_parse!(CounterStyle, "symbols(symbolic'+')");
	}
}
