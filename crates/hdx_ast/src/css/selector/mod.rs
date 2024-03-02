use hdx_atom::Atom;
use hdx_lexer::Token;
use hdx_parser::{
	diagnostics, discard, expect, peek, unexpected, unexpected_ident, Parse, Parser, Result as ParserResult, Span,
	Spanned,
};
use hdx_writer::{CssWriter, Result as WriterResult, WriteCss};
#[cfg(feature = "serde")]
use serde::Serialize;
use smallvec::{smallvec, SmallVec};

use crate::{Atomizable, Vec};

mod attribute;
mod pseudo_class;

use attribute::Attribute;
use pseudo_class::PseudoClass;

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde())]
pub struct Selectors<'a>(pub SmallVec<[Spanned<Selector<'a>>; 1]>);

impl<'a> Parse<'a> for Selectors<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		let mut selectors = smallvec![];
		loop {
			selectors.push(Selector::parse_spanned(parser)?);
			discard!(parser, Token::Whitespace);
			match parser.cur() {
				Token::Comma => {
					parser.advance();
				}
				_ => break,
			}
		}
		Ok(Selectors(selectors))
	}
}

impl<'a> WriteCss<'a> for Selectors<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		let mut iter = self.0.iter().peekable();
		while let Some(selector) = iter.next() {
			selector.write_css(sink)?;
			if iter.peek().is_some() {
				sink.write_char(',')?;
				sink.write_whitespace()?;
			}
		}
		Ok(())
	}
}

// This encapsulates both `simple-selector` and `compound-selector`.
// As `simple-selector` is a `compound-selector` but with only one `Component`.
// Having `Selector` be both ` simple-selector` and `compound-selector` makes
// parsing and visiting more practical.
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct Selector<'a> {
	pub components: Vec<'a, Spanned<Component<'a>>>,
}

impl<'a> Parse<'a> for Selector<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		discard!(parser, Token::Whitespace);

		let mut components: Vec<'a, Spanned<Component>> = parser.new_vec();
		loop {
			match parser.cur() {
				Token::Eof | Token::Semicolon | Token::Comma | Token::LeftCurly => {
					break;
				}
				Token::Whitespace if peek!(parser, Token::Eof | Token::Semicolon | Token::Comma | Token::LeftCurly) => {
					break;
				}
				token @ Token::RightCurly => unexpected!(parser, token),
				_ => {
					let span = parser.span();
					let component = Component::parse_spanned(parser)?;
					if let Some(Spanned { node, span: component_span }) = components.last() {
						match (node, &component.node) {
							// A selector like `a /**/ b` would parse as // <Type>, <Descendant>,
							// <Descendant>, <Type>. The CSS selector grammar implicitly swallows adjacent
							// descendant combinators as whitespace, but due to simplifying AST nodes in our
							// parser, it means we must explicitly check for, and elide adjacent descendant
							// combinators. Adjacent Descendant Combinator Elision is the name of my metal
							// band, btw.
							(Component::Combinator(_), Component::Combinator(Combinator::Descendant))
							| (Component::Combinator(Combinator::Descendant), Component::Combinator(_)) => {
								continue;
							}
							// Combinators cannot be next to eachother.
							(Component::Combinator(_), Component::Combinator(_)) => {
								Err(diagnostics::AdjacentSelectorCombinators(
									*component_span,
									Span::new(span.start, component_span.start),
								))?
							}
							// Types cannot be next to eachother.
							(Component::Type(_), Component::Type(_)) => Err(diagnostics::AdjacentSelectorTypes(
								*component_span,
								Span::new(span.start, component_span.start),
							))?,
							_ => {}
						}
					}
					components.push(component);
				}
			}
		}
		// Given selector parsing is Whitespace sensitive, trailing whitespace should be
		// discarded before moving onto the next parser which is likely a block parser
		discard!(parser, Token::Whitespace);
		Ok(Self { components })
	}
}

impl<'a> WriteCss<'a> for Selector<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		for component in &(*self.components) {
			component.write_css(sink)?;
		}
		Ok(())
	}
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ForgivingSelector<'a> {
	pub components: Vec<'a, Spanned<Component<'a>>>,
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct RelativeSelector<'a> {
	pub components: Vec<'a, Spanned<Component<'a>>>,
}

// This encapsulates all `simple-selector` subtypes (e.g. `wq-name`,
// `id-selector`) into one enum, as it makes parsing and visiting much more
// practical.
#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum Component<'a> {
	Id(Atom),
	Class(Atom),
	Type(Atom),
	Wildcard,
	Combinator(Combinator),
	Attribute(Attribute),
	PseudoClass(PseudoClass),
	PseudoElement(PseudoElement),
	LegacyPseudoElement(LegacyPseudoElement),
	PseudoFunction(PseudoFunction<'a>),
	NSPrefixedType((NSPrefix, Atom)),
	NSPrefixedWildcard(NSPrefix),
}

impl<'a> Parse<'a> for Component<'a> {
	fn parse(parser: &mut Parser<'a>) -> ParserResult<Self> {
		match parser.cur() {
			Token::Whitespace => {
				parser.advance();
				Ok(Self::Combinator(Combinator::Descendant))
			}
			Token::Ident(name) => {
				parser.advance_including_whitespace();
				Ok(Self::Type(name.to_ascii_lowercase()))
			}
			Token::Colon => {
				parser.advance_including_whitespace();
				match parser.cur() {
					Token::Colon => {
						parser.advance_including_whitespace();
						match parser.cur() {
							Token::Ident(name) => {
								if let Some(selector) = PseudoElement::from_atom(name.clone()) {
									parser.advance_including_whitespace();
									Ok(Self::PseudoElement(selector))
								} else {
									unexpected_ident!(parser, name)
								}
							}
							token => unexpected!(parser, token),
						}
					}
					Token::Ident(ident) => {
						if let Some(selector) = PseudoClass::from_atom(ident.clone()) {
							parser.advance_including_whitespace();
							Ok(Self::PseudoClass(selector))
						} else if let Some(e) = LegacyPseudoElement::from_atom(ident.clone()) {
							parser.advance_including_whitespace();
							Ok(Self::LegacyPseudoElement(e))
						} else {
							Err(diagnostics::UnexpectedIdent(ident, parser.span()))?
						}
					}
					_ => Err(diagnostics::Unimplemented(parser.span()))?,
				}
			}
			Token::Hash(name) => {
				parser.advance_including_whitespace();
				Ok(Self::Id(name))
			}
			Token::Delim(char) => match char {
				'.' => {
					parser.advance_including_whitespace();
					match parser.cur() {
						Token::Ident(ident) => {
							parser.advance_including_whitespace();
							Ok(Self::Class(ident))
						}
						_ => Err(diagnostics::Unimplemented(parser.span()))?,
					}
				}
				'*' => match parser.peek() {
					Token::Delim('|') => {
						let (prefix, atom) = parse_wq_name(parser)?;
						Ok(Self::NSPrefixedType((prefix, atom)))
					}
					_ => {
						parser.advance_including_whitespace();
						Ok(Self::Wildcard)
					}
				},
				_ => Err(diagnostics::Unimplemented(parser.span()))?,
			},
			Token::LeftSquare => {
				let attr = Attribute::parse(parser)?;
				Ok(Component::Attribute(attr))
			}
			_ => Err(diagnostics::Unimplemented(parser.span()))?,
		}
	}
}

impl<'a> WriteCss<'a> for Component<'a> {
	fn write_css<W: CssWriter>(&self, sink: &mut W) -> WriterResult {
		match self {
			Self::Type(ty) => sink.write_str(ty),
			Self::Id(id) => {
				sink.write_char('#')?;
				sink.write_str(id)
			}
			Self::Class(class) => {
				sink.write_char('.')?;
				sink.write_str(class)
			}
			Self::PseudoClass(pseudo) => {
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())
			}
			Self::LegacyPseudoElement(pseudo) => {
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())
			}
			Self::PseudoElement(pseudo) => {
				sink.write_char(':')?;
				sink.write_char(':')?;
				sink.write_str(pseudo.to_atom().as_ref())
			}
			Self::Attribute(attr) => attr.write_css(sink),
			Self::Combinator(combinator) => match combinator {
				Combinator::Descendant => sink.write_char(' '),
				Combinator::Child => {
					sink.write_whitespace()?;
					sink.write_char('>')?;
					sink.write_whitespace()
				}
				Combinator::NextSibling => {
					sink.write_whitespace()?;
					sink.write_char('+')?;
					sink.write_whitespace()
				}
				Combinator::SubsequentSibling => {
					sink.write_whitespace()?;
					sink.write_char('~')?;
					sink.write_whitespace()
				}
				Combinator::ColumnCombintor => {
					sink.write_whitespace()?;
					sink.write_char('|')?;
					sink.write_char('|')?;
					sink.write_whitespace()
				}
			},
			Self::Wildcard => sink.write_char('*'),
			_ => todo!(),
		}
	}
}

// https://drafts.csswg.org/css-pseudo/#index-defined-here
#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum PseudoElement {
	After,              // atom!("after")
	Backdrop,           // atom!("backdrop")
	Before,             // atom!("after")
	Cue,                // atom!("cue")
	CueRegion,          // atom!("cue-region")
	FirstLetter,        // atom!("first-letter")
	FirstLine,          // atom!("first-line")
	FileSelectorButton, // atom!("file-selector-button")
	GrammarError,       // atom!("grammar-error")
	Marker,             // atom!("marker")
	Placeholder,        // atom!("placeholder")
	Selection,          // atom!("selection")
	SpellingError,      // atom!("spelling-error")
	TargetText,         // atom!("target-text")
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum LegacyPseudoElement {
	After,       // atom!("after")
	Before,      // atom!("before")
	FirstLetter, // atom!("first-letter")
	FirstLine,   // atom!("first-line")
}

#[derive(Debug, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub enum PseudoFunction<'a> {
	Dir(DirValue),                // atom!("dir")
	Has(RelativeSelector<'a>),    // atom!("has")
	Host(Selector<'a>),           // atom!("host")
	HostContext(Selector<'a>),    // atom!("host-context")
	Is(ForgivingSelector<'a>),    // atom!("is")
	Lang(SmallVec<[Atom; 3]>),    // atom!("lang")
	Not(Selector<'a>),            // atom!("not")
	NthChild(ANBEvenOdd),         // atom!("nth-child")
	NthCol(ANB),                  // atom!("nth-col")
	NthLastChild(ANBEvenOdd),     // atom!("nth-last-child")
	NthLastCol(ANB),              // atom!("nth-last-col")
	NthLastOfType(ANBEvenOdd),    // atom!("nth-last-of-type")
	NthOfType(ANBEvenOdd),        // atom!("nth-of-type")
	Where(ForgivingSelector<'a>), // atom!("where")
}

#[derive(Atomizable, Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", rename_all = "kebab-case"))]
pub enum DirValue {
	Rtl, // atom!("rtl")
	Ltr, // atom!("ltr")
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type", content = "value"))]
pub enum NSPrefix {
	None,
	Wildcard,
	Named(Atom),
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
// https://drafts.csswg.org/selectors/#combinators
pub enum Combinator {
	Descendant,        // (Space)
	Child,             // >
	NextSibling,       // +
	SubsequentSibling, // ~
	ColumnCombintor,   // ||
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ANB {
	string: Atom,
}

#[derive(Debug, PartialEq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(tag = "type"))]
pub struct ANBEvenOdd {
	string: Atom,
}

pub(crate) fn parse_wq_name(parser: &mut Parser) -> ParserResult<(NSPrefix, Atom)> {
	let nsprefix = match parser.cur() {
		Token::Delim('|') if peek!(parser, Token::Ident(_)) => {
			parser.advance_including_whitespace();
			NSPrefix::None
		}
		Token::Delim('*') if peek!(parser, Token::Delim('|')) => {
			parser.advance_including_whitespace();
			expect!(parser, Token::Delim('|'));
			parser.advance_including_whitespace();
			NSPrefix::Wildcard
		}
		Token::Ident(name) => {
			parser.advance_including_whitespace();
			match parser.cur() {
				Token::Delim('|') if peek!(parser, Token::Ident(_)) => {
					parser.advance_including_whitespace();
					NSPrefix::Named(name)
				}
				_ => return Ok((NSPrefix::None, name)),
			}
		}
		token => unexpected!(parser, token),
	};
	match parser.cur() {
		Token::Ident(name) => {
			parser.advance_including_whitespace();
			Ok((nsprefix, name))
		}
		token => unexpected!(parser, token),
	}
}

#[cfg(test)]
mod test {
	use oxc_allocator::Allocator;

	use super::*;
	use crate::test_helpers::{test_write, test_write_min};

	#[test]
	fn size_test() {
		assert_eq!(::std::mem::size_of::<Selectors>(), 56);
		assert_eq!(::std::mem::size_of::<Selector>(), 32);
		assert_eq!(::std::mem::size_of::<ForgivingSelector>(), 32);
		assert_eq!(::std::mem::size_of::<RelativeSelector>(), 32);
		assert_eq!(::std::mem::size_of::<Component>(), 48);
		assert_eq!(::std::mem::size_of::<PseudoElement>(), 1);
		assert_eq!(::std::mem::size_of::<LegacyPseudoElement>(), 1);
		assert_eq!(::std::mem::size_of::<PseudoClass>(), 1);
		assert_eq!(::std::mem::size_of::<PseudoFunction>(), 40);
		assert_eq!(::std::mem::size_of::<DirValue>(), 1);
		assert_eq!(::std::mem::size_of::<Combinator>(), 1);
		assert_eq!(::std::mem::size_of::<ANB>(), 8);
		assert_eq!(::std::mem::size_of::<ANBEvenOdd>(), 8);
	}

	#[test]
	fn test_writes() {
		let allocator = Allocator::default();
		test_write::<Component>(&allocator, ":root", ":root");
		test_write::<Component>(&allocator, "*", "*");
		test_write::<Component>(&allocator, "[attr|='foo']", "[attr|='foo']");
		// test_write::<Component>(&allocator, "*|x", "*|x");
		test_write::<Selector>(&allocator, "a b ", "a b");
		test_write::<Selector>(&allocator, ":root", ":root");
		test_write::<Selector>(&allocator, "body [attr|='foo']", "body [attr|='foo']");
		// test_write::<Selector>(&allocator, "*|x :focus-within", "*|x
		// :focus-within");
		test_write::<Selectors>(&allocator, "a b ", "a b");
		test_write::<Selectors>(&allocator, ":root", ":root");
		test_write::<Selectors>(&allocator, "body [attr|='foo']", "body [attr|='foo']");
	}

	#[test]
	fn test_minify() {
		let allocator = Allocator::default();
		test_write_min::<Component>(&allocator, "[attr|='foo']", "[attr|=foo]");
		test_write_min::<Selector>(&allocator, "a   b", "a b");
		test_write_min::<Selector>(&allocator, "a   b ", "a b");
	}
}
