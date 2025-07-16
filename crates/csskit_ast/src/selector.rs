include!(concat!(env!("OUT_DIR"), "/css_ast_node_tag.rs"));

use bumpalo::collections::Vec;
pub use css_ast::{Attribute, Class, Combinator, Nth};
use css_lexer::Cursor;
use css_parse::{
	Build, CommaSeparated, CompoundSelector as CompoundSelectorTrait, Parse, Parser, Peek, Result as ParserResult,
	SelectorComponent as SelectorComponentTrait, T, ToCursors, diagnostics, pseudo_class, pseudo_element,
};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, ToSpan, Visitable};

mod functional_pseudo_class;
mod functional_pseudo_element;

pub use functional_pseudo_class::*;
pub use functional_pseudo_element::*;

pseudo_element!(
	///
	#[derive(Visitable)]
	#[visit(self)]
	pub enum PseudoElement {
		AnonymousBlock: "anonymous-block",
	}
);

pseudo_class!(
	///
	#[derive(Visitable)]
	#[visit(self)]
	pub enum PseudoClass {
		Root: "root",
		Empty: "empty",
	}
);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
pub struct Unparseable;

impl<'a> Peek<'a> for Unparseable {
	fn peek(_: &Parser<'a>, _: Cursor) -> bool {
		false
	}
}

impl<'a> Parse<'a> for Unparseable {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Err(diagnostics::Unexpected(p.peek_n(1).into(), p.peek_n(1).into()))?
	}
}

impl<'a> ToCursors for Unparseable {
	fn to_cursors(&self, _: &mut impl css_parse::CursorSink) {}
}

/// Represents a list of [CompoundSelectors][CompoundSelector], such as `body, dialog:modal`.
///
/// ```md
/// <selector-list>
///  │├─╭─ <compound-selector> ─╮─ "," ─╭─╮─┤│
///     │                       ╰───────╯ │
///     ╰─────────────────────────────────╯
/// ```
#[derive(Peek, Parse, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct SelectorList<'a>(pub CommaSeparated<'a, CompoundSelector<'a>>);

#[derive(Peek, ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit]
pub struct CompoundSelector<'a>(pub Vec<'a, SelectorComponent<'a>>);

impl<'a> CompoundSelectorTrait<'a> for CompoundSelector<'a> {
	type SelectorComponent = SelectorComponent<'a>;
}

impl<'a> Parse<'a> for CompoundSelector<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Ok(Self(Self::parse_compound_selector(p)?))
	}
}

pub type ComplexSelector<'a> = SelectorList<'a>;
pub type ForgivingSelector<'a> = SelectorList<'a>;
pub type RelativeSelector<'a> = SelectorList<'a>;

#[derive(Peek, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Id(T![Hash]);

impl<'a> Build<'a> for Id {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(<T![Hash]>::build(p, c))
	}
}

#[derive(Peek, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Wildcard(T![*]);

impl<'a> Build<'a> for Wildcard {
	fn build(p: &Parser<'a>, c: Cursor) -> Self {
		Self(<T![*]>::build(p, c))
	}
}

#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(
	feature = "serde",
	derive(serde::Serialize),
	serde(tag = "type", content = "value", rename_all = "kebab-case")
)]
#[visit(children)]
pub enum SelectorComponent<'a> {
	Id(Id),
	Tag(Tag),
	Wildcard(Wildcard),
	Combinator(Combinator),
	Attribute(Attribute),
	PseudoClass(PseudoClass),
	PseudoElement(PseudoElement),
	FunctionalPseudoElement(FunctionalPseudoElement),
	FunctionalPseudoClass(FunctionalPseudoClass<'a>),
}

impl<'a> Parse<'a> for SelectorComponent<'a> {
	fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
		Self::parse_selector_component(p)
	}
}

impl<'a> SelectorComponentTrait<'a> for SelectorComponent<'a> {
	type Wildcard = Wildcard;
	type Id = Id;
	type Type = Tag;
	type PseudoClass = PseudoClass;
	type PseudoElement = PseudoElement;
	type LegacyPseudoElement = Unparseable;
	type Class = Unparseable;
	type NsType = Unparseable;
	type Combinator = Combinator;
	type Attribute = Attribute;
	type FunctionalPseudoClass = FunctionalPseudoClass<'a>;
	type FunctionalPseudoElement = FunctionalPseudoElement;

	fn build_wildcard(node: Wildcard) -> Self {
		Self::Wildcard(node)
	}

	fn build_id(node: Id) -> Self {
		Self::Id(node)
	}

	fn build_class(_: Unparseable) -> Self {
		panic!("Should never be able to parse a namespace!");
	}

	fn build_type(node: Tag) -> Self {
		Self::Tag(node)
	}

	fn build_pseudo_class(node: PseudoClass) -> Self {
		Self::PseudoClass(node)
	}

	fn build_pseudo_element(node: PseudoElement) -> Self {
		Self::PseudoElement(node)
	}

	fn build_legacy_pseudo_element(_: Unparseable) -> Self {
		panic!("Should never be able to parse a namespace!");
	}

	fn build_ns_type(_: Unparseable) -> Self {
		panic!("Should never be able to parse a namespace!");
	}

	fn build_combinator(node: Combinator) -> Self {
		Self::Combinator(node)
	}

	fn build_attribute(node: Attribute) -> Self {
		Self::Attribute(node)
	}

	fn build_functional_pseudo_class(node: FunctionalPseudoClass<'a>) -> Self {
		Self::FunctionalPseudoClass(node)
	}

	fn build_functional_pseudo_element(node: FunctionalPseudoElement) -> Self {
		Self::FunctionalPseudoElement(node)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use css_parse::{assert_parse, assert_parse_error};

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SelectorList>(), 32);
		assert_eq!(std::mem::size_of::<ComplexSelector>(), 32);
		assert_eq!(std::mem::size_of::<ForgivingSelector>(), 32);
		assert_eq!(std::mem::size_of::<RelativeSelector>(), 32);
		assert_eq!(std::mem::size_of::<SelectorComponent>(), 128);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SelectorList, ":root");
		assert_parse!(SelectorList, "selector-list");
		assert_parse!(SelectorList, "margin-right-style-value,font-weight-style-value");
		assert_parse!(SelectorList, "*");
		assert_parse!(SelectorList, "[attr|='foo']");
		assert_parse!(SelectorList, "* attribute");
		assert_parse!(SelectorList, "class charset-rule");
		assert_parse!(SelectorList, "  combinator document-matcher", "combinator document-matcher");
		assert_parse!(SelectorList, "keyframes-name [attr|='foo']");
		assert_parse!(SelectorList, "html-tag > id");
		assert_parse!(SelectorList, ":empty");
		assert_parse!(SelectorList, " /**/ width-container-feature", "width-container-feature");
		assert_parse!(SelectorList, "& html-non-standard-tag");
		assert_parse!(SelectorList, "&:empty");
		assert_parse!(SelectorList, "nth &:empty");
		assert_parse!(SelectorList, "style-value & & &", "style-value & & &");
		assert_parse!(SelectorList, "tag&");
		assert_parse!(SelectorList, "&&");
		assert_parse!(SelectorList, "& + #foo,&#bar");
	}

	#[test]
	fn test_errors() {
		// Namespace selector components are not valid for this language.
		assert_parse_error!(SelectorList, "*|layer-name");
		// Classnames selector components are not valid for this language.
		assert_parse_error!(SelectorList, ".foo[attr*=\"foo\"]");
		// Non Standard
		assert_parse_error!(SelectorList, "::-moz-focus-inner");
	}
}
