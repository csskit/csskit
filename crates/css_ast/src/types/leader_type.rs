use css_parse::{T, keyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

keyword_set!(pub enum LeaderTypeKeywords { Dotted: "dotted", Solid: "solid", Space: "space" });

// https://drafts.csswg.org/css-content-3/#typedef-leader-type
// <leader-type> = dotted | solid | space | <string>
#[derive(ToSpan, Parse, Peek, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LeaderType {
	Keyword(LeaderTypeKeywords),
	String(T![String]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LeaderType>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(LeaderType, "dotted");
		assert_parse!(LeaderType, "'.'");
		assert_parse!(LeaderType, "'abc'");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(LeaderType, "foo");
	}
}
