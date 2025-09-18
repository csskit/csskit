use bumpalo::collections::Vec;
use css_parse::{
	CompoundSelector as CompoundSelectorTrait, Parse, Parser, Result as ParserResult,
	SelectorComponent as SelectorComponentTrait, T, syntax::CommaSeparated,
};
use csskit_derives::{IntoCursor, Parse, Peek, ToCursors, ToSpan, Visitable};

mod attribute;
mod class;
mod combinator;
mod functional_pseudo_class;
mod functional_pseudo_element;
mod moz;
mod ms;
mod namespace;
mod nth;
mod o;
mod pseudo_class;
mod pseudo_element;
mod tag;
mod webkit;

pub use attribute::*;
pub use class::*;
pub use combinator::*;
pub use functional_pseudo_class::*;
pub use functional_pseudo_element::*;
pub use moz::*;
pub use ms::*;
pub use namespace::*;
pub use nth::*;
pub use o::*;
pub use pseudo_class::*;
pub use pseudo_element::*;
pub use tag::*;
pub use webkit::*;

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

#[derive(Peek, Parse, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Id(T![Hash]);

#[derive(Peek, Parse, ToCursors, IntoCursor, Visitable, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(self)]
pub struct Wildcard(T![*]);

// This encapsulates all `simple-selector` subtypes (e.g. `wq-name`,
// `id-selector`) into one enum, as it makes parsing and visiting much more
// practical.
#[derive(ToSpan, ToCursors, Visitable, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[visit(children)]
pub enum SelectorComponent<'a> {
	Id(Id),
	Class(Class),
	Tag(Tag),
	Wildcard(Wildcard),
	Combinator(Combinator),
	Attribute(Attribute),
	PseudoClass(PseudoClass),
	PseudoElement(PseudoElement),
	FunctionalPseudoElement(FunctionalPseudoElement<'a>),
	LegacyPseudoElement(LegacyPseudoElement),
	FunctionalPseudoClass(FunctionalPseudoClass<'a>),
	Namespace(Namespace),
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
	type LegacyPseudoElement = LegacyPseudoElement;
	type Class = Class;
	type NsType = Namespace;
	type Combinator = Combinator;
	type Attribute = Attribute;
	type FunctionalPseudoClass = FunctionalPseudoClass<'a>;
	type FunctionalPseudoElement = FunctionalPseudoElement<'a>;

	fn build_wildcard(node: Wildcard) -> Self {
		Self::Wildcard(node)
	}

	fn build_id(node: Id) -> Self {
		Self::Id(node)
	}

	fn build_class(node: Class) -> Self {
		Self::Class(node)
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

	fn build_legacy_pseudo_element(node: LegacyPseudoElement) -> Self {
		Self::LegacyPseudoElement(node)
	}

	fn build_ns_type(node: Namespace) -> Self {
		Self::Namespace(node)
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

	fn build_functional_pseudo_element(node: FunctionalPseudoElement<'a>) -> Self {
		Self::FunctionalPseudoElement(node)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::assert_visits;
	use css_parse::assert_parse;

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<SelectorList>(), 32);
		assert_eq!(std::mem::size_of::<ComplexSelector>(), 32);
		assert_eq!(std::mem::size_of::<ForgivingSelector>(), 32);
		assert_eq!(std::mem::size_of::<RelativeSelector>(), 32);
		assert_eq!(std::mem::size_of::<SelectorComponent>(), 128);
		assert_eq!(std::mem::size_of::<LegacyPseudoElement>(), 28);
		assert_eq!(std::mem::size_of::<Combinator>(), 28);
	}

	#[test]
	fn test_writes() {
		assert_parse!(SelectorList, ":root");
		assert_parse!(SelectorList, "body,body");
		assert_parse!(SelectorList, ".body .body");
		assert_parse!(SelectorList, "*");
		assert_parse!(SelectorList, "[attr|='foo']");
		assert_parse!(SelectorList, "*|x");
		assert_parse!(SelectorList, "* x");
		assert_parse!(SelectorList, "a b");
		assert_parse!(SelectorList, "  a b", "a b");
		assert_parse!(SelectorList, "body [attr|='foo']");
		assert_parse!(SelectorList, "*|x :focus-within");
		assert_parse!(SelectorList, ".foo[attr*=\"foo\"]");
		assert_parse!(SelectorList, "a > b");
		assert_parse!(SelectorList, ".foo[attr*=\"foo\"] > *");
		assert_parse!(SelectorList, ".foo[attr*=\"foo\"] > * + *");
		assert_parse!(SelectorList, ":after");
		assert_parse!(SelectorList, "::after");
		assert_parse!(SelectorList, ":before");
		assert_parse!(SelectorList, "::before");
		assert_parse!(SelectorList, "::before:focus:target:right:playing:popover-open:blank");
		assert_parse!(SelectorList, ":dir(ltr)");
		assert_parse!(SelectorList, "tr:nth-child(n-1):state(foo)");
		assert_parse!(SelectorList, " /**/ .foo", ".foo");
		assert_parse!(SelectorList, ":lang(en-gb,en-us)");
		assert_parse!(SelectorList, "& .foo");
		assert_parse!(SelectorList, "&:hover");
		assert_parse!(SelectorList, ".foo &:hover");
		assert_parse!(SelectorList, ".foo & & &", ".foo & & &");
		assert_parse!(SelectorList, ".class&");
		assert_parse!(SelectorList, "&&");
		assert_parse!(SelectorList, "& + .foo,&.bar");
		assert_parse!(SelectorList, ":state(foo)&", ":state(foo)&");
		assert_parse!(SelectorList, ":heading(1)");
		assert_parse!(SelectorList, ":heading(1,2,3)");
		// Non Standard
		assert_parse!(SelectorList, "::-moz-focus-inner");
		assert_parse!(
			SelectorList,
			"::-moz-list-bullet::-webkit-scrollbar::-ms-clear:-ms-input-placeholder::-o-scrollbar:-o-prefocus"
		);
		assert_parse!(SelectorList, "button:-moz-focusring");
		assert_parse!(SelectorList, "::view-transition-group(*)");
		assert_parse!(SelectorList, "::view-transition-new(thing.foo.bar.baz)");
	}

	#[test]
	fn test_visits() {
		assert_visits!(".foo", CompoundSelector, Class);
		assert_visits!("#bar", CompoundSelector, Id);
		assert_visits!(".foo", SelectorList, CompoundSelector, Class);
		assert_visits!(".foo, #bar", SelectorList, CompoundSelector, Class, CompoundSelector, Id);
		assert_visits!(".foo#bar", CompoundSelector, Class, Id);
		assert_visits!(".foo.bar", CompoundSelector, Class, Class);
		assert_visits!(".foo", CompoundSelector, Class);
		assert_visits!(".foo#bar", CompoundSelector, Class, Id);
		assert_visits!(".foo", CompoundSelector, Class);
		assert_visits!("*.foo#bar", CompoundSelector, Wildcard, Class, Id);
	}

	#[test]
	#[should_panic]
	fn test_assert_visits_fails() {
		assert_visits!(".foo", CompoundSelector, visit_id<Id>);
	}
}
