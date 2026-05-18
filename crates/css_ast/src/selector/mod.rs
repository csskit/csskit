use crate::{
	CssMetadata,
	specificity::{Specificity, ToSpecificity},
};
use bumpalo::collections::Vec;
use css_parse::{
	CompoundSelector as CompoundSelectorTrait, Cursor, NodeMetadata, NodeWithMetadata, Parse, Parser,
	Result as ParserResult, SelectorComponent as SelectorComponentTrait, T, syntax::CommaSeparated,
};
use csskit_derives::{IntoCursor, Parse, Peek, SemanticEq, ToCursors, ToSpan};

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
#[derive(Peek, Parse, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
pub struct SelectorList<'a>(pub CommaSeparated<'a, CompoundSelector<'a>>);

impl<'a> NodeWithMetadata<CssMetadata> for SelectorList<'a> {
	fn self_metadata(&self) -> CssMetadata {
		CssMetadata::default().with_size(self.0.len().min(u16::MAX as usize) as u16)
	}

	fn metadata(&self) -> CssMetadata {
		self.self_metadata()
	}
}

#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit)]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct CompoundSelector<'a>(pub Vec<'a, SelectorComponent<'a>>);

impl<'a> CompoundSelectorTrait<'a> for CompoundSelector<'a> {
	type SelectorComponent = SelectorComponent<'a>;
}

impl<'a> Parse<'a> for CompoundSelector<'a> {
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Ok(Self(Self::parse_compound_selector(p)?))
	}
}

pub type ComplexSelector<'a> = SelectorList<'a>;
pub type ForgivingSelector<'a> = SelectorList<'a>;
pub type RelativeSelector<'a> = SelectorList<'a>;

#[derive(Peek, Parse, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Id(T![Hash]);

#[derive(Peek, Parse, ToCursors, IntoCursor, Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(self))]
#[derive(csskit_derives::NodeWithMetadata)]
pub struct Wildcard(T![*]);

// This encapsulates all `simple-selector` subtypes (e.g. `wq-name`,
// `id-selector`) into one enum, as it makes parsing and visiting much more
// practical.
#[derive(Peek, ToSpan, ToCursors, SemanticEq, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
#[cfg_attr(feature = "visitable", derive(csskit_derives::Visitable), visit(children))]
#[derive(csskit_derives::NodeWithMetadata)]
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
	fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		Self::parse_selector_component(p)
	}
}

impl<'a> ToSpecificity for SelectorComponent<'a> {
	fn specificity(&self) -> Specificity {
		match self {
			Self::Id(_) => Specificity(1, 0, 0),
			Self::Class(_) | Self::Attribute(_) | Self::PseudoClass(_) => Specificity(0, 1, 0),
			Self::Tag(_) | Self::PseudoElement(_) | Self::LegacyPseudoElement(_) => Specificity(0, 0, 1),
			Self::FunctionalPseudoElement(_) => Specificity(0, 0, 1),
			Self::Combinator(_) | Self::Namespace(_) | Self::Wildcard(_) => Specificity(0, 0, 0),
			Self::FunctionalPseudoClass(f) => f.specificity(),
		}
	}
}

impl<'a> ToSpecificity for CompoundSelector<'a> {
	fn specificity(&self) -> Specificity {
		self.0.iter().map(ToSpecificity::specificity).sum()
	}
}

impl<'a> ToSpecificity for SelectorList<'a> {
	fn specificity(&self) -> Specificity {
		(&self.0).into_iter().map(|(s, _)| s.specificity()).max().unwrap_or_default()
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
	use crate::{CssAtomSet, specificity::ToSpecificity};
	use bumpalo::Bump;
	use css_lexer::Lexer;
	use css_parse::Parser;
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
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":root");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "body,body");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".body .body");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "*");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "[attr|='foo']");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "*|x");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "* x");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "a b");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "  a b");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "body [attr|='foo']");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "*|x :focus-within");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".foo[attr*=\"foo\"]");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "a > b");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".foo[attr*=\"foo\"] > *");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".foo[attr*=\"foo\"] > * + *");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":after");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::after");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":before");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::before");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::before:focus:target:right:playing:popover-open:blank");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":dir(ltr)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "tr:nth-child(n-1):state(foo)");
		// assert_parse!(CssAtomSet::ATOMS, SelectorList, " /**/ .foo");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":lang(en-gb,en-us)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "& .foo");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "&:hover");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".foo &:hover");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".foo & & &");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ".class&");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "&&");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "& + .foo,&.bar");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":state(foo)&");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":heading(1)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, ":heading(1,2,3)");
		// Non Standard
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::-moz-focus-inner");
		assert_parse!(
			CssAtomSet::ATOMS,
			SelectorList,
			"::-moz-list-bullet::-webkit-scrollbar::-ms-clear:-ms-input-placeholder::-o-scrollbar:-o-prefocus"
		);
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "button:-moz-focusring");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::view-transition-group(*)");
		assert_parse!(CssAtomSet::ATOMS, SelectorList, "::view-transition-new(thing.foo.bar.baz)");
	}

	#[test]
	#[cfg(feature = "visitable")]
	fn test_visits() {
		use crate::assert_visits;
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
		assert_visits!(".foo .bar", CompoundSelector, Class, Combinator, Class);
		assert_visits!(".foo ", CompoundSelector, Class);
		assert_visits!("a > b", CompoundSelector, Tag, HtmlTag, Combinator, Tag, HtmlTag);
		assert_visits!("a>b", CompoundSelector, Tag, HtmlTag, Combinator, Tag, HtmlTag);
		assert_visits!("a + b", CompoundSelector, Tag, HtmlTag, Combinator, Tag, HtmlTag);
		assert_visits!("a ~ b", CompoundSelector, Tag, HtmlTag, Combinator, Tag, HtmlTag);
		assert_visits!(".foo > .bar + .baz", CompoundSelector, Class, Combinator, Class, Combinator, Class);
	}

	#[test]
	#[should_panic]
	#[cfg(feature = "visitable")]
	fn test_assert_visits_fails() {
		use crate::assert_visits;
		assert_visits!(".foo", CompoundSelector, visit_id<Id>);
	}

	macro_rules! assert_specificity {
		($sel:literal, $a:literal, $b:literal, $c:literal) => {{
			let bump = Bump::default();
			let lexer = Lexer::new(&CssAtomSet::ATOMS, $sel);
			let mut parser = Parser::new(&bump, $sel, lexer);
			let result = parser.parse_entirely::<SelectorList>().with_trivia();
			assert!(result.errors.is_empty(), "parse failed for {:?}: {:?}", $sel, result.errors);
			let s = result.output.unwrap().specificity();
			assert_eq!(
				s,
				Specificity($a, $b, $c),
				"selector {:?}: expected ({},{},{}) got ({},{},{})",
				$sel,
				$a,
				$b,
				$c,
				s.0,
				s.1,
				s.2
			);
		}};
	}

	#[test]
	fn test_specificity_arithmetic() {
		assert_eq!(Specificity(0, 1, 0) + Specificity(0, 1, 0), Specificity(0, 2, 0));
		assert_eq!(Specificity(1, 0, 0) + Specificity(0, 1, 0), Specificity(1, 1, 0));
		assert_eq!(Specificity(255, 0, 0) + Specificity(1, 0, 0), Specificity(255, 0, 0));
	}

	#[test]
	fn test_specificity() {
		assert_specificity!("#foo", 1, 0, 0);
		assert_specificity!(".foo", 0, 1, 0);
		assert_specificity!(".a.b.c", 0, 3, 0);
		assert_specificity!("div", 0, 0, 1);
		assert_specificity!(":hover", 0, 1, 0);
		assert_specificity!("::before", 0, 0, 1);
		assert_specificity!(":before", 0, 0, 1);
		assert_specificity!("[href]", 0, 1, 0);
		assert_specificity!("*", 0, 0, 0);
		assert_specificity!("a.foo", 0, 1, 1);
		assert_specificity!("a.foo:hover", 0, 2, 1);
		assert_specificity!("#a.b", 1, 1, 0);
		assert_specificity!(":where(.a.b)", 0, 0, 0);
		assert_specificity!(":is(.a, #b)", 1, 0, 0);
		assert_specificity!(":not(.a, .b)", 0, 1, 0);
		assert_specificity!("a:has(.b)", 0, 1, 1);
		assert_specificity!(":nth-child(2)", 0, 1, 0);
		assert_specificity!(":nth-of-type(2n+1)", 0, 1, 0);
		assert_specificity!(".a, #b", 1, 0, 0);
	}

	#[test]
	fn test_specificity_complex() {
		assert_specificity!("nav ul li:nth-child(even) a:not([href^='#'])", 0, 2, 4);
		assert_specificity!("button:only-of-type:enabled:active:hover", 0, 4, 1);
		assert_specificity!("table tr:not(:first-child):hover td:nth-child(2n+1)", 0, 3, 3);
		assert_specificity!("input[type='checkbox'][checked]:indeterminate + label", 0, 3, 2);
	}
}
