use super::prelude::*;
use crate::{FontFamilyName, FontFormat, FontTech, Value};

/// <https://drafts.csswg.org/css-fonts-4/#src-desc>
///
/// ```text,ignore
/// format() = format( <font-format> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct FormatFunction<'a> {
	#[atom(CssAtomSet::Format)]
	pub name: T![Function],
	pub params: Value<'a, FontFormat>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-fonts-4/#src-desc>
///
/// ```text,ignore
/// tech() = tech( <font-tech># )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct TechFunction<'a> {
	#[atom(CssAtomSet::Tech)]
	pub name: T![Function],
	pub params: CommaSeparated<'a, Value<'a, FontTech>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-fonts-4/#src-desc>
///
/// ```text,ignore
/// local() = local( <font-family-name> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct LocalFunction<'a> {
	#[atom(CssAtomSet::Local)]
	pub name: T![Function],
	pub params: FontFamilyName<'a>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, FormatFunction, "format(woff2)");
		assert_parse!(CssAtomSet::ATOMS, FormatFunction, "format(\"woff2\")");
		assert_parse!(CssAtomSet::ATOMS, TechFunction, "tech(variations)");
		assert_parse!(CssAtomSet::ATOMS, TechFunction, "tech(features-opentype,color-COLRv1)");
		assert_parse!(CssAtomSet::ATOMS, LocalFunction, "local(Gentium)");
		assert_parse!(CssAtomSet::ATOMS, LocalFunction, "local(\"Gentium Book\")");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, FormatFunction, "format(1px)");
		assert_parse_error!(CssAtomSet::ATOMS, TechFunction, "tech()");
	}
}
