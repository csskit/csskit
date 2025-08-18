use bumpalo::collections::Vec;
use css_parse::{Function, T, function_set, keyword_set};
use csskit_derives::{Parse, Peek, ToCursors, ToSpan};

use crate::types::Image;

function_set!(pub struct SymbolsFunctionName "symbols");

/// <https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols>
///
/// ```text,ignore
/// symbols() = symbols( <symbols-type>? [ <string> | <image> ]+ )
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct SymbolsFunction<'a>(Function<'a, SymbolsFunctionName, (Option<SymbolsType>, Vec<'a, Symbol<'a>>)>);

keyword_set!(
	/// <https://drafts.csswg.org/css-counter-styles-3/#typedef-symbols-type>
	///
	/// ```text,ignore
	/// <symbols-type> = cyclic | numeric | alphabetic | symbolic | fixed
	/// ```
	pub enum SymbolsType {
		Cyclic: "cyclic",
		Numeric: "numeric",
		Alphabetic: "alphabetic",
		Symbolic: "symbolic",
		Fixed: "fixed",
	}
);

/// <https://drafts.csswg.org/css-counter-styles-3/#funcdef-symbols>
///
/// A single Symbol from the `<symbols()>` syntax
///
/// ```text,ignore
/// <string> | <image>
/// ```
#[derive(Parse, Peek, ToSpan, ToCursors, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[allow(clippy::large_enum_variant)] // TODO: Box or shrink Image
pub enum Symbol<'a> {
	String(T![String]),
	Image(Image<'a>),
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SymbolsFunction>(), 80);
		assert_eq!(std::mem::size_of::<Symbol>(), 216);
		assert_eq!(std::mem::size_of::<SymbolsType>(), 16);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SymbolsFunction, "symbols(symbolic'+')");
		assert_parse!(SymbolsFunction, "symbols(symbolic'*''†''‡')");
	}
}
