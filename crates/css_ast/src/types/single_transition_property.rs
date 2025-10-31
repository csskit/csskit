use super::prelude::*;

// https://drafts.csswg.org/css-transitions-1/#single-transition-property
// <single-transition-property> = all | <custom-ident>
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
pub enum SingleTransitionProperty {
	#[atom(CssAtomSet::All)]
	All(T![Ident]),
	CustomIdent(T![Ident]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SingleTransitionProperty>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SingleTransitionProperty, "all");
		assert_parse!(CssAtomSet::ATOMS, SingleTransitionProperty, "bar");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, SingleTransitionProperty, "123deg");
	}
}
