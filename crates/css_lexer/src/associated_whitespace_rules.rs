use bitmask_enum::bitmask;

/// A [bitmask][bitmask_enum] representing rules around the whitespace surrounding a [Kind::Delim][crate::Kind::Delim]
/// token.
///
/// A [Token][crate::Token] with [Kind::Delim][crate::Kind::Delim] or one of the other single character tokens (such as
/// [Kind::LeftCurly][crate::Kind::LeftCurly] will store this data internal to the token. Using
/// [Token::associated_whitespace()][crate::Token::associated_whitespace()] will return this bitmask, depending on what
/// rules are set for this token. By default the [Lexer][crate::Lexer] will produce tokens with
/// [AssociatedWhitespaceRules::None], but new tokens can be created which can be accompanied with a different set of
/// rules.
///
/// ```
/// use css_lexer::*;
/// let mut lexer = Lexer::new(".");
/// {
///		// This token will be a Delim of `.`
///		let token = lexer.advance();
///		assert_eq!(token, Kind::Delim);
///		assert_eq!(token, AssociatedWhitespaceRules::None);
/// }
/// ```
#[bitmask(u8)]
#[bitmask_config(vec_debug)]
pub enum AssociatedWhitespaceRules {
	None = 0b000,
	/// If the token before this one is not whitespace, then whitespace must be placed before this token.
	EnforceBefore = 0b100,
	/// The token must produce a whitespace token to separate it and the next token (if the next token is not already
	/// whitespace).
	EnforceAfter = 0b010,
	/// The token after this one must not be whitespace, doing so would result in breaking a higher level association with
	/// the adjacent token (for example a pseudo class such as `:hover`).
	BanAfter = 0b001,
}

impl AssociatedWhitespaceRules {
	pub(crate) const fn from_bits(bits: u8) -> Self {
		Self { bits: bits & 0b111 }
	}

	pub(crate) const fn to_bits(self) -> u8 {
		self.bits
	}
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<AssociatedWhitespaceRules>(), 1);
}

#[test]
fn test_from_bits() {
	assert!(
		AssociatedWhitespaceRules::from_bits(AssociatedWhitespaceRules::EnforceBefore.bits)
			.contains(AssociatedWhitespaceRules::EnforceBefore)
	);
	assert!(
		AssociatedWhitespaceRules::from_bits(AssociatedWhitespaceRules::EnforceAfter.bits)
			.contains(AssociatedWhitespaceRules::EnforceAfter)
	);
	assert!(
		AssociatedWhitespaceRules::from_bits(AssociatedWhitespaceRules::BanAfter.bits)
			.contains(AssociatedWhitespaceRules::BanAfter)
	);
}
