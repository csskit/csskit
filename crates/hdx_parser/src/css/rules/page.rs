use hdx_ast::css::{
	properties::Property,
	rules::page::{
		CSSMarginRule, CSSPageRule, PageMarginBox, PagePseudoClass, PageSelector, PageSelectorList,
	},
};
use oxc_allocator::Vec;

use super::NoPreludeAllowed;
use crate::{atom, diagnostics, Atom, Atomizable, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for CSSPageRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		parser.parse_at_rule(
			Some(atom!("page")),
			|parser: &mut Parser<'a>,
			 _name: Atom,
			 selectors: Option<Spanned<PageSelectorList<'a>>>,
			 rules: Vec<'a, Spanned<CSSMarginRule<'a>>>,
			 declarations: Vec<'a, Spanned<Property<'a>>>| {
				Ok(Self {
					selectors: parser.boxup(selectors.unwrap_or_else(|| {
						Spanned::dummy(PageSelectorList { children: parser.new_vec() })
					})),
					declarations: parser.boxup(declarations),
					rules: parser.boxup(rules),
				}
				.spanned(span.end(parser.pos())))
			},
		)
	}
}

impl<'a> Parse<'a> for PageSelectorList<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let ok = Ok(Self { children: parser.parse_comma_list_of::<PageSelector>()? }
			.spanned(span.end(parser.pos())));
		ok
	}
}

impl<'a> Parse<'a> for PageSelector<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		let mut page_type = None;
		let mut pseudos = parser.new_vec();
		if parser.at(Kind::Ident) {
			page_type = Some(parser.expect_ident()?);
		} else {
			parser.expect_without_advance(Kind::Colon)?;
		}
		if parser.at(Kind::Colon) {
			loop {
				pseudos.push(PagePseudoClass::parse(parser)?);
				if !parser.at(Kind::Colon) {
					break;
				}
			}
		}
		Ok(Self { page_type, pseudos }.spanned(span.end(parser.pos())))
	}
}

impl<'a> Parse<'a> for PagePseudoClass {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		parser.expect(Kind::Colon)?;
		let name = parser.expect_ident()?;
		match Self::from_atom(name.clone()) {
			Some(v) => Ok(v.spanned(span.end(parser.pos()))),
			_ => Err(diagnostics::UnexpectedPseudo(name, span).into()),
		}
	}
}

impl<'a> Parse<'a> for CSSMarginRule<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		parser.parse_at_rule(
			None,
			|parser: &mut Parser<'a>,
			 _name: Atom,
			 _prelude: Option<Spanned<NoPreludeAllowed>>,
			 _rules: Vec<'a, Spanned<CSSMarginRule<'a>>>,
			 declarations: Vec<'a, Spanned<Property<'a>>>| {
				Ok(Self { name: PageMarginBox::TopLeft, declarations }
					.spanned(span.end(parser.pos())))
			},
		)
	}
}

#[cfg(test)]
mod test {
	use hdx_ast::{
		css::{
			properties::{Background, Property},
			rules::{CSSPageRule, PagePseudoClass, PageSelector, PageSelectorList},
			values::{ColorValue, NamedColor},
		},
		Spanned,
	};
	use oxc_allocator::Allocator;

	use crate::{Atom, Parser, ParserOptions, Span, Vec};

	#[test]
	fn parses_toc_left_selector() {
		let allocator = Allocator::default();
		let parser = Parser::new(&allocator, "toc:left", ParserOptions::default());
		let parser_return = parser.parse_with::<PageSelectorList>();
		let ast = parser_return.output.unwrap();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		if !parser_return.warnings.is_empty() {
			panic!("{:?}", parser_return.warnings[0]);
		}
		let mut children = Vec::new_in(&allocator);
		let mut pseudos = Vec::new_in(&allocator);
		pseudos.push(Spanned { span: Span::new(3, 8), node: PagePseudoClass::Left });
		children.push(Spanned {
			span: Span::new(0, 8),
			node: PageSelector { page_type: Some(Atom::from("toc")), pseudos },
		});
		assert_eq!(ast, Spanned { span: Span::new(0, 8), node: PageSelectorList { children } });
	}

	#[test]
	fn parses_toc_left_page_rule_with_bakcground_black() {
		let allocator = Allocator::default();
		let parser = Parser::new(
			&allocator,
			"@page toc:left { background: black; }",
			ParserOptions::default(),
		);
		let mut children = Vec::new_in(&allocator);
		let mut pseudos = Vec::new_in(&allocator);
		pseudos.push(Spanned { span: Span::new(9, 15), node: PagePseudoClass::Left });
		children.push(Spanned {
			span: Span::new(6, 15),
			node: PageSelector { page_type: Some(Atom::from("toc")), pseudos },
		});
		let mut declarations = Vec::new_in(&allocator);
		declarations.push(Spanned {
			span: Span::new(17, 36),
			node: Property::Background({
				parser.boxup(Spanned {
					span: Span::new(17, 36),
					node: Background {
						value: parser.boxup(Spanned {
							span: Span::new(29, 34),
							node: ColorValue::Named(NamedColor::Black),
						}),
						important: false,
					},
				})
			}),
		});
		let expected = Spanned {
			span: Span::new(0, 37),
			node: CSSPageRule {
				selectors: parser
					.boxup(Spanned { span: Span::new(6, 15), node: PageSelectorList { children } }),
				declarations: parser.boxup(declarations),
				rules: parser.boxup(Vec::new_in(&allocator)),
			},
		};
		let parser_return = parser.parse_entirely_with::<CSSPageRule>();
		if !parser_return.errors.is_empty() {
			panic!("{:?}", parser_return.errors[0]);
		}
		if !parser_return.warnings.is_empty() {
			panic!("{:?}", parser_return.warnings[0]);
		}
		let ast = parser_return.output.unwrap();
		assert_eq!(ast, expected);
	}
}
