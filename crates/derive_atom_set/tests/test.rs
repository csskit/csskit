use derive_atom_set::AtomSet;

// Define the trait that our derive macro implements
pub trait AtomSet {
	fn from_str(s: &str) -> Self;
	fn to_str(self) -> &'static str;
	fn len(&self) -> u32;
	fn from_bits(value: u32) -> Self;
	fn as_bits(&self) -> u32;
	fn is_empty(&self) -> bool {
		self.len() == 0
	}
}

#[derive(AtomSet, Debug, Clone, Copy, PartialEq, Eq)]
enum TestAtomSet {
	#[default]
	_None,

	// Tuple matching (1-5 chars)
	A,  // 1 char
	Px, // 2 chars
	Dpx,
	Dpi,   // 3 Chars
	Auto,  // 4 chars
	Block, // 5 chars

	// u64 lookup (6-8 chars)
	Medium,   // 6 chars
	Display,  // 7 chars
	Position, // 8 chars

	// u128 lookup (9-16 chars)
	Background,     // 10 chars
	BorderWidth,    // 12 chars
	FlexDirection,  // 14 chars
	TextDecoration, // 15 chars

	// Multi-u128 lookup (>16 chars)
	VeryLongAtomStr,               // 18 chars
	ExtremelyLongAtomString,       // 27 chars
	SuperLongAtomStringForTesting, // 33 chars

	// Custom atoms
	#[atom("%")]
	Percentage,
}

#[test]
fn test() {
	// 1-5 chars
	assert_eq!(TestAtomSet::from_str("a"), TestAtomSet::A);
	assert_eq!(TestAtomSet::from_str("A"), TestAtomSet::A);
	assert_eq!(TestAtomSet::from_str("px"), TestAtomSet::Px);
	assert_eq!(TestAtomSet::from_str("dpx"), TestAtomSet::Dpx);
	assert_eq!(TestAtomSet::from_str("dpi"), TestAtomSet::Dpi);
	assert_eq!(TestAtomSet::from_str("auto"), TestAtomSet::Auto);
	assert_eq!(TestAtomSet::from_str("block"), TestAtomSet::Block);

	// 6-8 chars
	assert_eq!(TestAtomSet::from_str("medium"), TestAtomSet::Medium);
	assert_eq!(TestAtomSet::from_str("display"), TestAtomSet::Display);
	assert_eq!(TestAtomSet::from_str("position"), TestAtomSet::Position);

	// 9-16 chars
	assert_eq!(TestAtomSet::from_str("background"), TestAtomSet::Background);
	assert_eq!(TestAtomSet::from_str("border-width"), TestAtomSet::BorderWidth);
	assert_eq!(TestAtomSet::from_str("flex-direction"), TestAtomSet::FlexDirection);
	assert_eq!(TestAtomSet::from_str("text-decoration"), TestAtomSet::TextDecoration);

	// >16 chars
	assert_eq!(TestAtomSet::from_str("very-long-atom-str"), TestAtomSet::VeryLongAtomStr);
	assert_eq!(TestAtomSet::from_str("extremely-long-atom-string"), TestAtomSet::ExtremelyLongAtomString);
	assert_eq!(TestAtomSet::from_str("super-long-atom-string-for-testing"), TestAtomSet::SuperLongAtomStringForTesting);

	// case insensitive
	assert_eq!(TestAtomSet::from_str("BACKGROUND"), TestAtomSet::Background);
	assert_eq!(TestAtomSet::from_str("VERY-LONG-ATOM-STR"), TestAtomSet::VeryLongAtomStr);
	assert_eq!(TestAtomSet::from_str("SUPER-LONG-ATOM-STRING-FOR-TESTING"), TestAtomSet::SuperLongAtomStringForTesting);

	// custom atoms
	assert_eq!(TestAtomSet::from_str("%"), TestAtomSet::Percentage);

	// non-matches
	assert_eq!(TestAtomSet::from_str("unknown"), TestAtomSet::_None);
	assert_eq!(TestAtomSet::from_str("very-long-nonexistent-string"), TestAtomSet::_None);

	// round-trip
	assert_eq!(TestAtomSet::VeryLongAtomStr.to_str(), "very-long-atom-str");
	assert_eq!(TestAtomSet::ExtremelyLongAtomString.to_str(), "extremely-long-atom-string");
	assert_eq!(TestAtomSet::SuperLongAtomStringForTesting.to_str(), "super-long-atom-string-for-testing");
}

#[derive(AtomSet, Debug, Clone, Copy, PartialEq, Eq)]
enum PunctuationAtomSet {
	#[default]
	_None,

	#[atom("_")]
	Underscore,
	#[atom("a_b")]
	ShortUnderscore,
	#[atom("a[b")]
	ShortBracket,
	#[atom("under_score_x")]
	MediumUnderscore,
	#[atom("very_long_under_score_atom")]
	LongUnderscore,
	#[atom("café")]
	NonAscii,
	#[atom("abcde")]
	AsciiSibling,
	#[atom("ünder_score_ünder_score")]
	LongNonAscii,
}

#[test]
fn matches_atoms_with_punctuation() {
	assert_eq!(PunctuationAtomSet::from_str("_"), PunctuationAtomSet::Underscore);
	assert_eq!(PunctuationAtomSet::from_str("a_b"), PunctuationAtomSet::ShortUnderscore);
	assert_eq!(PunctuationAtomSet::from_str("a[b"), PunctuationAtomSet::ShortBracket);
	assert_eq!(PunctuationAtomSet::from_str("under_score_x"), PunctuationAtomSet::MediumUnderscore);
	assert_eq!(PunctuationAtomSet::from_str("very_long_under_score_atom"), PunctuationAtomSet::LongUnderscore);

	assert_eq!(PunctuationAtomSet::from_str("A_B"), PunctuationAtomSet::ShortUnderscore);
	assert_eq!(PunctuationAtomSet::from_str("UNDER_SCORE_X"), PunctuationAtomSet::MediumUnderscore);
	assert_eq!(PunctuationAtomSet::from_str("VERY_LONG_UNDER_SCORE_ATOM"), PunctuationAtomSet::LongUnderscore);
}

#[test]
fn matches_atoms_with_non_ascii_bytes() {
	assert_eq!(PunctuationAtomSet::from_str("café"), PunctuationAtomSet::NonAscii);
	assert_eq!(PunctuationAtomSet::from_str("CAFé"), PunctuationAtomSet::NonAscii);
	assert_eq!(PunctuationAtomSet::from_str("CAFÉ"), PunctuationAtomSet::_None);
	assert_eq!(PunctuationAtomSet::from_str("abcde"), PunctuationAtomSet::AsciiSibling);
	assert_eq!(PunctuationAtomSet::from_str("ABCDE"), PunctuationAtomSet::AsciiSibling);
	assert_eq!(PunctuationAtomSet::from_str("ünder_score_ünder_score"), PunctuationAtomSet::LongNonAscii);
	assert_eq!(PunctuationAtomSet::from_str("üNDER_SCORE_üNDER_SCORE"), PunctuationAtomSet::LongNonAscii);
}

#[test]
fn matches_atoms_with_unicode() {
	assert_eq!(PunctuationAtomSet::from_str("\u{7f}"), PunctuationAtomSet::_None);
	assert_eq!(PunctuationAtomSet::from_str("a\u{7f}b"), PunctuationAtomSet::_None);
	assert_eq!(PunctuationAtomSet::from_str("a{b"), PunctuationAtomSet::_None);
	assert_eq!(PunctuationAtomSet::from_str("under\u{7f}score\u{7f}x"), PunctuationAtomSet::_None);
	assert_eq!(
		PunctuationAtomSet::from_str("very\u{7f}long\u{7f}under\u{7f}score\u{7f}atom"),
		PunctuationAtomSet::_None
	);
}
