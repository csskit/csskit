use hdx_ast::css::values::{CounterStyle, PredefinedCounterStyle, Symbols};

use crate::{atom, diagnostics, Atomizable, Kind, Parse, Parser, Result, Spanned};

impl<'a> Parse<'a> for CounterStyle<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		let span = parser.span();
		match parser.cur() {
			Kind::Ident => {
				let ident = parser.expect_ident()?;
				if let Some(node) = PredefinedCounterStyle::from_atom(ident.clone()) {
					Ok(Self::Predefined(node).spanned(span))
				} else {
					Ok(Self::Named(ident).spanned(span))
				}
			}
			Kind::Function => {
				let ident = parser.expect_ident()?;
				if ident == atom!("symbols") {
					let node = Symbols::parse(parser)?;
					Ok(Self::Symbols(node).spanned(span.end(parser.pos())))
				} else {
					Err(diagnostics::ExpectedFunction(atom!("symbols"), ident, parser.span()))?
				}
			}
			k => Err(diagnostics::Unexpected(k, parser.span()))?,
		}
	}
}

impl<'a> Parse<'a> for Symbols<'a> {
	fn parse(parser: &mut Parser<'a>) -> Result<Spanned<Self>> {
		Err(diagnostics::Unimplemented(parser.span()))?
	}
}
