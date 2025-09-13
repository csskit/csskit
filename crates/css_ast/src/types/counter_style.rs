use super::prelude::*;

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

// https://drafts.csswg.org/css-counter-styles-3/#predefined-counters
#[derive(Parse, Peek, ToCursors, ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum PredefinedCounter {
	#[atom(CssAtomSet::Decimal)]
	Decimal(T![Ident]),
	#[atom(CssAtomSet::DecimalLeadingZero)]
	DecimalLeadingZero(T![Ident]),
	#[atom(CssAtomSet::ArabicIndic)]
	ArabicIndic(T![Ident]),
	#[atom(CssAtomSet::Armenian)]
	Armenian(T![Ident]),
	#[atom(CssAtomSet::UpperArmenian)]
	UpperArmenian(T![Ident]),
	#[atom(CssAtomSet::LowerArmenian)]
	LowerArmenian(T![Ident]),
	#[atom(CssAtomSet::Bengali)]
	Bengali(T![Ident]),
	#[atom(CssAtomSet::Cambodian)]
	Cambodian(T![Ident]),
	#[atom(CssAtomSet::Khmer)]
	Khmer(T![Ident]),
	#[atom(CssAtomSet::CjkDecimal)]
	CjkDecimal(T![Ident]),
	#[atom(CssAtomSet::Devanagari)]
	Devanagari(T![Ident]),
	#[atom(CssAtomSet::Georgian)]
	Georgian(T![Ident]),
	#[atom(CssAtomSet::Gujarati)]
	Gujarati(T![Ident]),
	#[atom(CssAtomSet::Gurmukhi)]
	Gurmukhi(T![Ident]),
	#[atom(CssAtomSet::Hebrew)]
	Hebrew(T![Ident]),
	#[atom(CssAtomSet::Kannada)]
	Kannada(T![Ident]),
	#[atom(CssAtomSet::Lao)]
	Lao(T![Ident]),
	#[atom(CssAtomSet::Malayalam)]
	Malayalam(T![Ident]),
	#[atom(CssAtomSet::Mongolian)]
	Mongolian(T![Ident]),
	#[atom(CssAtomSet::Myanmar)]
	Myanmar(T![Ident]),
	#[atom(CssAtomSet::Oriya)]
	Oriya(T![Ident]),
	#[atom(CssAtomSet::Persian)]
	Persian(T![Ident]),
	#[atom(CssAtomSet::LowerRoman)]
	LowerRoman(T![Ident]),
	#[atom(CssAtomSet::UpperRoman)]
	UpperRoman(T![Ident]),
	#[atom(CssAtomSet::Tamil)]
	Tamil(T![Ident]),
	#[atom(CssAtomSet::Telugu)]
	Telugu(T![Ident]),
	#[atom(CssAtomSet::Thai)]
	Thai(T![Ident]),
	#[atom(CssAtomSet::Tibetan)]
	Tibetan(T![Ident]),
	#[atom(CssAtomSet::LowerAlpha)]
	LowerAlpha(T![Ident]),
	#[atom(CssAtomSet::UpperAlpha)]
	UpperAlpha(T![Ident]),
	#[atom(CssAtomSet::UpperLatin)]
	UpperLatin(T![Ident]),
	#[atom(CssAtomSet::LowerGreek)]
	LowerGreek(T![Ident]),
	#[atom(CssAtomSet::Hiragana)]
	Hiragana(T![Ident]),
	#[atom(CssAtomSet::HiraganaIroha)]
	HiraganaIroha(T![Ident]),
	#[atom(CssAtomSet::Katakana)]
	Katakana(T![Ident]),
	#[atom(CssAtomSet::KatakanaIroha)]
	KatakanaIroha(T![Ident]),
	#[atom(CssAtomSet::Disc)]
	Disc(T![Ident]),
	#[atom(CssAtomSet::Square)]
	Square(T![Ident]),
	#[atom(CssAtomSet::DisclousureOpen)]
	DisclousureOpen(T![Ident]),
	#[atom(CssAtomSet::DisclousureClosed)]
	DisclousureClosed(T![Ident]),
	#[atom(CssAtomSet::CjkEarthlyBranch)]
	CjkEarthlyBranch(T![Ident]),
	#[atom(CssAtomSet::CjkHeavenlyStem)]
	CjkHeavenlyStem(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<CounterStyle>(), 72);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, CounterStyle, "cjk-heavenly-stem");
		assert_parse!(CssAtomSet::ATOMS, CounterStyle, "foobar");
		assert_parse!(CssAtomSet::ATOMS, CounterStyle, "symbols(symbolic'+')");
	}
}
