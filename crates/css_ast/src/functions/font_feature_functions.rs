use super::prelude::*;
use crate::{CustomIdent, Value};

/// <https://drafts.csswg.org/css-fonts-5/#font-variant-alternates-prop>
///
/// ```text,ignore
/// stylistic() = stylistic( <font-feature-value-name> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct StylisticFunction<'a> {
	#[atom(CssAtomSet::Stylistic)]
	pub name: T![Function],
	pub params: Value<'a, CustomIdent>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-fonts-5/#font-variant-alternates-prop>
///
/// ```text,ignore
/// styleset() = styleset( <font-feature-value-name># )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct StylesetFunction<'a> {
	#[atom(CssAtomSet::Styleset)]
	pub name: T![Function],
	pub params: CommaSeparated<'a, Value<'a, CustomIdent>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-fonts-5/#font-variant-alternates-prop>
///
/// ```text,ignore
/// character-variant() = character-variant( <font-feature-value-name># )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CharacterVariantFunction<'a> {
	#[atom(CssAtomSet::CharacterVariant)]
	pub name: T![Function],
	pub params: CommaSeparated<'a, Value<'a, CustomIdent>>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-fonts-5/#font-variant-alternates-prop>
///
/// ```text,ignore
/// swash() = swash( <font-feature-value-name> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct SwashFunction<'a> {
	#[atom(CssAtomSet::Swash)]
	pub name: T![Function],
	pub params: Value<'a, CustomIdent>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-fonts-5/#font-variant-alternates-prop>
///
/// ```text,ignore
/// ornaments() = ornaments( <font-feature-value-name> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct OrnamentsFunction<'a> {
	#[atom(CssAtomSet::Ornaments)]
	pub name: T![Function],
	pub params: Value<'a, CustomIdent>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

/// <https://drafts.csswg.org/css-fonts-5/#font-variant-alternates-prop>
///
/// ```text,ignore
/// annotation() = annotation( <font-feature-value-name> )
/// ```
#[node]
#[derive(Parse, Peek, ToCursors, ToSpan, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct AnnotationFunction<'a> {
	#[atom(CssAtomSet::Annotation)]
	pub name: T![Function],
	pub params: Value<'a, CustomIdent>,
	#[semantic_eq(skip)]
	pub close: T![')'],
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::CssAtomSet;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn test_writes() {
		assert_parse!(CssAtomSet::ATOMS, StylisticFunction, "stylistic(foo)");
		assert_parse!(CssAtomSet::ATOMS, SwashFunction, "swash(fancy)");
		assert_parse!(CssAtomSet::ATOMS, OrnamentsFunction, "ornaments(bullets)");
		assert_parse!(CssAtomSet::ATOMS, AnnotationFunction, "annotation(circled)");
		assert_parse!(CssAtomSet::ATOMS, StylesetFunction, "styleset(foo)");
		assert_parse!(CssAtomSet::ATOMS, StylesetFunction, "styleset(foo,bar)");
		assert_parse!(CssAtomSet::ATOMS, CharacterVariantFunction, "character-variant(foo,bar)");
	}

	#[test]
	fn test_errors() {
		assert_parse_error!(CssAtomSet::ATOMS, StylisticFunction, "stylistic()");
		assert_parse_error!(CssAtomSet::ATOMS, StylisticFunction, "stylistic(foo,bar)");
		assert_parse_error!(CssAtomSet::ATOMS, SwashFunction, "swash(1)");
		assert_parse_error!(CssAtomSet::ATOMS, StylesetFunction, "styleset()");
	}
}
