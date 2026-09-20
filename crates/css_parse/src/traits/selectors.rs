use super::prelude::*;
use crate::{Kind, KindSet};

pub trait CompoundSelector<'a>: Sized + Parse<'a> {
	/// SelectorComponent represents a Selector, or Combinator.
	/// <https://drafts.csswg.org/selectors-4/#typedef-combinator>
	/// <https://drafts.csswg.org/selectors-4/#typedef-type-selector>
	/// <https://drafts.csswg.org/selectors-4/#typedef-subclass-selector>
	/// <https://drafts.csswg.org/selectors-4/#typedef-pseudo-element-selector>
	type SelectorComponent: Parse<'a> + SelectorComponent<'a>;

	/// Parse the next selector component, or return Ok(None) if at a terminator.
	/// This allows implementors to process components incrementally without building the full Vec first.
	fn parse_compound_selector_part<I>(p: &mut Parser<'a, I>) -> Result<Option<Self::SelectorComponent>>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		// If a stop token has been reached (skipping whitespace), return None
		let skip = p.set_skip(KindSet::TRIVIA);
		let next = p.peek_n(1);
		p.set_skip(skip);
		if next == Kind::Eof || next == KindSet::LEFT_CURLY_RIGHT_PAREN_COMMA_OR_SEMICOLON {
			return Ok(None);
		}
		p.parse::<Self::SelectorComponent>().map(Some)
	}

	fn parse_compound_selector<I>(p: &mut Parser<'a, I>) -> Result<Vec<'a, Self::SelectorComponent>>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		let mut components = Vec::new_in(p.alloc());
		// Trim leading whitespace
		p.consume_trivia();
		while let Some(component) = Self::parse_compound_selector_part(p)? {
			components.push(component);
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

	fn parse_selector_component<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
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
						Err(Diagnostic::new(c, Diagnostic::unexpected_tag))?
					}
				}
			},
			Kind::Hash if t.hash_is_id_like() => {
				p.set_skip(skip);
				if Self::Id::peek(p, c) {
					Ok(Self::build_id(p.parse::<Self::Id>()?))
				} else {
					Err(Diagnostic::new(c, Diagnostic::unexpected_id))?
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
						_ => Err(Diagnostic::new(c, Diagnostic::expected_ident))?,
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
					let value = p.parse::<Self::Combinator>().map(Self::build_combinator);
					p.set_skip(KindSet::WHITESPACE);
					p.consume_trivia_as_leading();
					p.set_skip(skip);
					value
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
							_ => Err(Diagnostic::new(c3, Diagnostic::unexpected))?,
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
					_ => Err(Diagnostic::new(c2, Diagnostic::unexpected))?,
				}
			}
			_ => {
				// If this is whitespace, check if there's an explicit combinator ahead.
				// Combinators cannot be adjacent, so whitespace before an explicit
				// combinator (>, +, ~, ||, &) should be consumed as trivia, not parsed
				// as a Descendant combinator.
				if t.kind() == Kind::Whitespace {
					p.set_skip(KindSet::TRIVIA);
					let next = p.peek_n(1);
					let next_is_explicit_combinator = match next.token().kind() {
						Kind::Delim => matches!(next.token().char(), Some('>' | '+' | '~' | '|' | '&')),
						_ => false,
					};
					if next_is_explicit_combinator {
						p.consume_trivia_as_leading();
						p.set_skip(skip);
						return Self::parse_selector_component(p);
					}
					p.set_skip(skip);
				}
				let value = p.parse::<Self::Combinator>().map(Self::build_combinator);
				// Given descendant combinators cannot appear in sequence with other combinators, we can safely eat trivia here
				// in order to remove unecessary conjoined descendant combinators
				p.set_skip(KindSet::WHITESPACE);
				p.consume_trivia_as_leading();
				p.set_skip(skip);
				value
			}
		}
	}
}

/// An `<an+b>` value, such as the `2n+1` of `:nth-child(2n+1)`.
///
/// <https://drafts.csswg.org/css-syntax-3/#anb-microsyntax>
pub trait Nth<'a>: Sized {
	/// The set of [Kinds][Kind] that can start an `<an+b>` value.
	const NTH_KINDSET: KindSet = KindSet::new(&[Kind::Number, Kind::Ident, Kind::Dimension, Kind::Delim]);

	/// Returns true if the cursor is the `odd` keyword.
	fn peek_odd<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone;

	/// Returns true if the cursor is the `even` keyword.
	fn peek_even<I>(p: &Parser<'a, I>, c: Cursor) -> bool
	where
		I: Iterator<Item = Cursor> + Clone;

	fn build_odd(node: T![Ident]) -> Self;
	fn build_even(node: T![Ident]) -> Self;
	fn build_integer(node: T![Number]) -> Self;
	fn build_anb(a: i32, b: i32, cursors: [Cursor; 4]) -> Self;

	/// Returns true if the 1-based `index` is one of the indices selected by `an+b`.
	fn anb_matches(a: i32, b: i32, index: i32) -> bool {
		if a == 0 {
			index == b
		} else {
			let diff = index - b;
			diff % a == 0 && diff / a >= 0
		}
	}

	fn parse_nth<I>(p: &mut Parser<'a, I>) -> Result<Self>
	where
		I: Iterator<Item = Cursor> + Clone,
	{
		if p.peek::<T![Number]>() && p.peek_n(1).token().is_int() {
			return p.parse::<T![Number]>().map(Self::build_integer);
		} else if p.peek::<T![Ident]>() {
			let peek_cursor = p.peek_n(1);
			if Self::peek_odd(p, peek_cursor) {
				return p.parse::<T![Ident]>().map(Self::build_odd);
			} else if Self::peek_even(p, peek_cursor) {
				return p.parse::<T![Ident]>().map(Self::build_even);
			}
		}

		let mut c = p.next();

		let mut b_sign = 0;
		let mut cursors = [c, Cursor::EMPTY, Cursor::EMPTY, Cursor::EMPTY];

		if c == '+' {
			let skip = p.set_skip(KindSet::NONE);
			c = p.next();
			p.set_skip(skip);
			debug_assert!(cursors[1] == Cursor::EMPTY);
			cursors[1] = c;
		}
		if !matches!(c.token().kind(), Kind::Number | Kind::Dimension | Kind::Ident) {
			Err(Diagnostic::new(c, Diagnostic::unexpected))?
		}
		if c.token().is_float() {
			Err(Diagnostic::new(c, Diagnostic::expected_int))?
		}

		let source_cursor = p.to_source_cursor(c);
		let anb = source_cursor.parse(p.alloc());
		let mut chars = anb.chars();
		let mut char = chars.next();
		let a = if c.token().is_int() {
			c.token().value() as i32
		} else if char == Some('-') {
			char = chars.next();
			-1
		} else {
			1
		};
		if !matches!(char, Some('n') | Some('N')) {
			Err(Diagnostic::new(c, Diagnostic::unexpected))?
		}
		let rest = chars.as_str();
		if rest == "-" {
			// A trailing `-` on the unit, as in `n-`, `-n-` or `5n-`, makes b negative and leaves its
			// digits to arrive as a separate unsigned token.
			b_sign = -1;
		} else if let Ok(b) = rest.parse::<i32>() {
			return Ok(Self::build_anb(a, b, cursors));
		} else if !rest.is_empty() {
			Err(Diagnostic::new(c, Diagnostic::unexpected))?
		}

		if b_sign == 0 {
			if p.peek::<T![+]>() {
				b_sign = 1;
				c = p.parse::<T![+]>()?.into();
				debug_assert!(cursors[2] == Cursor::EMPTY);
				cursors[2] = c;
			} else if p.peek::<T![-]>() {
				b_sign = -1;
				c = p.parse::<T![-]>()?.into();
				debug_assert!(cursors[2] == Cursor::EMPTY);
				cursors[2] = c;
			}
		}

		let b = if p.peek::<T![Number]>() {
			c = p.parse::<T![Number]>()?.into();
			debug_assert!(cursors[3] == Cursor::EMPTY);
			if c.token().is_float() {
				Err(Diagnostic::new(c, Diagnostic::expected_int))?
			}
			if c.token().has_sign() && b_sign != 0 {
				Err(Diagnostic::new(c, Diagnostic::expected_unsigned))?
			}
			// If the number has a sign (like +1 or -1), mark it as required for minification
			if c.token().has_sign() {
				c = c.map_token(|t| t.with_sign_required());
			}
			cursors[3] = c;
			if b_sign == 0 {
				b_sign = 1;
			}
			let i = c.token().value();
			(i.abs() as i32) * b_sign
		} else {
			0
		};
		Ok(Self::build_anb(a, b, cursors))
	}
}
