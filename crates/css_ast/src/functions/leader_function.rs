use super::prelude::*;

/// <https://drafts.csswg.org/css-content-3/#leader-function>
///
/// ```text,ignore
/// leader() = leader( <leader-type> )
/// <leader-type> = dotted | solid | space | <string>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct LeaderFunction {
	#[atom(CssAtomSet::Leader)]
	pub name: T![Function],
	pub params: LeaderType,
	pub close: T![')'],
}

// https://drafts.csswg.org/css-content-3/#typedef-leader-type
// <leader-type> = dotted | solid | space | <string>
#[derive(ToSpan, Parse, Peek, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub enum LeaderType {
	#[atom(CssAtomSet::Dotted)]
	Dotted(T![Ident]),
	#[atom(CssAtomSet::Solid)]
	Solid(T![Ident]),
	#[atom(CssAtomSet::Space)]
	Space(T![Ident]),
	String(T![String]),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<LeaderFunction>(), 40);
		assert_eq!(std::mem::size_of::<LeaderType>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, LeaderType, "dotted");
		assert_parse!(CssAtomSet::ATOMS, LeaderType, "'.'");
		assert_parse!(CssAtomSet::ATOMS, LeaderType, "'abc'");
		assert_parse!(CssAtomSet::ATOMS, LeaderFunction, "leader(dotted)");
		assert_parse!(CssAtomSet::ATOMS, LeaderFunction, "leader('.')");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, LeaderType, "foo");
		assert_parse_error!(CssAtomSet::ATOMS, LeaderFunction, "leader(foo)");
	}
}
