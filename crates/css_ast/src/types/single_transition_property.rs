#![allow(warnings)]
use css_parse::{T, keyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan, Visitable};

keyword_set!(pub struct AllKeyword "all");

// https://drafts.csswg.org/css-transitions-1/#single-transition-property
// <single-transition-property> = all | <custom-ident>
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub enum SingleTransitionProperty {
	#[parse(keyword = AllKeyword)]
	All(T![Ident]),
	CustomIdent(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SingleTransitionProperty>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SingleTransitionProperty, "all");
		assert_parse!(SingleTransitionProperty, "bar");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(SingleTransitionProperty, "123deg");
	}
}
