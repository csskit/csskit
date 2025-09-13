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
	A,     // 1 char
	Px,    // 2 chars
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
