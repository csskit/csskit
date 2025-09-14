use crate::{Cursor, Kind, KindSet, Parse, Parser, Peek, Result, diagnostics};
use bumpalo::collections::Vec;

pub trait CompoundSelector<'a>: Sized + Parse<'a> {
	// SelectorComponent represents a Selector, or Combinator.
	// https://drafts.csswg.org/selectors-4/#typedef-combinator
	// https://drafts.csswg.org/selectors-4/#typedef-type-selector
	// https://drafts.csswg.org/selectors-4/#typedef-subclass-selector
	// https://drafts.csswg.org/selectors-4/#typedef-pseudo-element-selector
	type SelectorComponent: Parse<'a> + SelectorComponent<'a>;

	fn parse_compound_selector(p: &mut Parser<'a>) -> Result<Vec<'a, Self::SelectorComponent>> {
		let mut components = Vec::new_in(p.bump());
		// Trim leading whitespace
		p.consume_trivia();
		loop {
			// If a stop token has been reached, break the loop
			if p.at_end() || p.peek_n(1) == KindSet::LEFT_CURLY_RIGHT_PAREN_COMMA_OR_SEMICOLON {
				break;
			}
			components.push(p.parse::<Self::SelectorComponent>()?);
		}
		Ok(components)
	}
}

pub trait SelectorComponent<'a>: Sized {
	type Wildcard: Peek<'a> + Parse<'a>;
	type Id: Peek<'a> + Parse<'a>;
	type Type: Peek<'a> + Parse<'a>;
	type PseudoClass: Parse<'a>;
	type PseudoElement: Parse<'a>;
	type LegacyPseudoElement: Peek<'a> + Parse<'a>;
	type Class: Parse<'a>;
	type NsType: Parse<'a>;
	type Combinator: Parse<'a>;
	type Attribute: Parse<'a>;
	type FunctionalPseudoClass: Parse<'a>;
	type FunctionalPseudoElement: Parse<'a>;

	fn build_wildcard(node: Self::Wildcard) -> Self;
	fn build_id(node: Self::Id) -> Self;
	fn build_class(node: Self::Class) -> Self;
	fn build_type(node: Self::Type) -> Self;
	fn build_pseudo_class(node: Self::PseudoClass) -> Self;
	fn build_pseudo_element(node: Self::PseudoElement) -> Self;
	fn build_legacy_pseudo_element(node: Self::LegacyPseudoElement) -> Self;
	fn build_ns_type(node: Self::NsType) -> Self;
	fn build_combinator(node: Self::Combinator) -> Self;
	fn build_attribute(node: Self::Attribute) -> Self;
	fn build_functional_pseudo_class(node: Self::FunctionalPseudoClass) -> Self;
	fn build_functional_pseudo_element(node: Self::FunctionalPseudoElement) -> Self;

	fn parse_selector_component(p: &mut Parser<'a>) -> Result<Self> {
		let skip = p.set_skip(KindSet::COMMENTS);
		let c = p.peek_n(1);
		let t = c.token();
		match t.kind() {
			Kind::Ident => match p.peek_n(2) {
				t if t == '|' => {
					p.set_skip(skip);
					p.parse::<Self::NsType>().map(Self::build_ns_type)
				}
				_ => {
					p.set_skip(skip);
					if Self::Type::peek(p, c) {
						Ok(Self::build_type(p.parse::<Self::Type>()?))
					} else {
						Err(diagnostics::UnexpectedTag(p.parse_str_lower(c).to_owned(), c))?
					}
				}
			},
			Kind::Hash if t.hash_is_id_like() => {
				p.set_skip(skip);
				if Self::Id::peek(p, c) {
					Ok(Self::build_id(p.parse::<Self::Id>()?))
				} else {
					Err(diagnostics::UnexpectedId(p.parse_str_lower(c).to_owned(), c))?
				}
			}
			Kind::LeftSquare => {
				p.set_skip(skip);
				p.parse::<Self::Attribute>().map(Self::build_attribute)
			}
			Kind::Delim => match t.char().unwrap() {
				'.' => {
					let c = p.peek_n(2);
					p.set_skip(skip);
					match c.token().kind() {
						Kind::Ident => p.parse::<Self::Class>().map(Self::build_class),
						_ => Err(diagnostics::ExpectedIdent(c))?,
					}
				}
				'*' => {
					let t = p.peek_n(2);
					p.set_skip(skip);
					if t == '|' {
						p.parse::<Self::NsType>().map(Self::build_ns_type)
					} else {
						Ok(Self::build_wildcard(p.parse::<Self::Wildcard>()?))
					}
				}
				_ => {
					p.set_skip(skip);
					p.parse::<Self::Combinator>().map(Self::build_combinator)
				}
			},
			Kind::Colon => {
				let c2 = p.peek_n(2);
				match c2.token().kind() {
					Kind::Colon => {
						let c3 = p.peek_n(3);
						p.set_skip(skip);
						match c3.token().kind() {
							Kind::Ident => p.parse::<Self::PseudoElement>().map(Self::build_pseudo_element),
							Kind::Function => {
								p.parse::<Self::FunctionalPseudoElement>().map(Self::build_functional_pseudo_element)
							}
							_ => Err(diagnostics::Unexpected(c3))?,
						}
					}
					Kind::Ident => {
						p.set_skip(skip);
						if Self::LegacyPseudoElement::peek(p, c) {
							p.parse::<Self::LegacyPseudoElement>().map(Self::build_legacy_pseudo_element)
						} else {
							p.parse::<Self::PseudoClass>().map(Self::build_pseudo_class)
						}
					}
					Kind::Function => {
						p.set_skip(skip);
						p.parse::<Self::FunctionalPseudoClass>().map(Self::build_functional_pseudo_class)
					}
					_ => Err(diagnostics::Unexpected(c2))?,
				}
			}
			_ => {
				let value = p.parse::<Self::Combinator>().map(Self::build_combinator);
				// Given descendant combinators cannot appear in sequence with other combinators, we can safely eat trivia here
				// in order to remove unecessary conjoined descendant combinators
				p.set_skip(KindSet::WHITESPACE);
				p.consume_trivia();
				p.set_skip(skip);
				value
			}
		}
	}
}

impl<'a, T> Peek<'a> for T
where
	T: SelectorComponent<'a>,
{
	const PEEK_KINDSET: KindSet = KindSet::new(&[Kind::Hash, Kind::Ident, Kind::Delim, Kind::Colon, Kind::LeftSquare]);

	fn peek(_: &Parser<'a>, c: Cursor) -> bool {
		c == Self::PEEK_KINDSET
	}
}
