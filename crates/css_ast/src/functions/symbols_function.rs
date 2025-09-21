use super::prelude::*;
use crate::{CssAtomSet, types::Image};

/// <https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols>
///
/// ```text,ignore
/// symbols() = symbols( <symbols-type>? [ <string> | <image> ]+ )
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct SymbolsFunction<'a> {
	#[atom(CssAtomSet::Symbols)]
	pub name: T![Function],
	pub params: SymbolsFunctionParams<'a>,
	pub close: T![')'],
}

#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct SymbolsFunctionParams<'a> {
	pub symbols_type: Option<SymbolsType>,
	pub symbols: Vec<'a, Symbol<'a>>,
}

/// <https://drafts.csswg.org/css-counter-styles-3/#typedef-symbols-type>
///
/// ```text,ignore
/// <symbols-type> = cyclic | numeric | alphabetic | symbolic | fixed
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(skip)]
pub enum SymbolsType {
	#[atom(CssAtomSet::Cyclic)]
	Cyclic(T![Ident]),
	#[atom(CssAtomSet::Numeric)]
	Numeric(T![Ident]),
	#[atom(CssAtomSet::Alphabetic)]
	Alphabetic(T![Ident]),
	#[atom(CssAtomSet::Symbolic)]
	Symbolic(T![Ident]),
	#[atom(CssAtomSet::Fixed)]
	Fixed(T![Ident]),
}

/// <https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols>
///
/// A single Symbol from the `<symbols()>` syntax
///
/// ```text,ignore
/// <string> | <image>
/// ```
#[derive(Parse, Peek, ToCursors, ToSpan, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[allow(clippy::large_enum_variant)] // TODO: Box or shrink Image
#[visit(children)]
pub enum Symbol<'a> {
	#[visit(skip)]
	String(T![String]),
	Image(Image<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SymbolsFunction>(), 72);
		assert_eq!(std::mem::size_of::<Symbol>(), 208);
		assert_eq!(std::mem::size_of::<SymbolsType>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, SymbolsFunction, "symbols(symbolic'+')");
		assert_parse!(CssAtomSet::ATOMS, SymbolsFunction, "symbols(symbolic'*''†''‡')");
	}
}
