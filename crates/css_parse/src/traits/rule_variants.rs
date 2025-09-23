use crate::{BadDeclaration, Cursor, Diagnostic, Parser, Peek, Result, State, T, ToCursors, ToSpan};

/// A trait that can be used for AST nodes representing a Declaration's Value. It offers some
/// convenience functions for handling such values.
pub trait RuleVariants<'a>: Sized + ToCursors + ToSpan {
	/// Like [crate::Parse::parse()] but with the additional context of the `name` [Cursor]. This cursor is known to be
	/// an [AtKeyword][crate::token_macros::AtKeyword], therefore this should return a `Self` reflecting a AtRule. If the
	/// AtRule is not _known_, or otherwise fails then this should [Err] and [RuleVariants::parse_unknown_at_rule()] can
	/// be called.
	///
	/// The default implementation of this method is to return an Unexpected [Err].
	fn parse_at_rule(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		let c = p.peek_n(1);
		Err(Diagnostic::new(c, Diagnostic::unexpected))?
	}

	/// Like [crate::Parse::parse()] but with the additional context of the `name` [Cursor]. This cursor is known to be
	/// an AtKeyword and that [RuleVariants::parse_at_rule()] failed. This should therefore return a Self that represents
	/// an Unknwon AtRule, or otherwise [Err].
	///
	/// The default implementation of this method is to return an Unexpected [Err].
	fn parse_unknown_at_rule(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		let c = p.peek_n(1);
		Err(Diagnostic::new(c, Diagnostic::unexpected))?
	}

	/// Like [crate::Parse::parse()] but with the additional context that the next cursor is _not_ an
	/// [AtKeyword][crate::token_macros::AtKeyword], therefore this can attempt to parse a Qualified Rule. If the rule
	/// fails to parse, then [RuleVariants::parse_unknown_qualified_rule()] will be called.
	///
	/// The default implementation of this method is to return an Unexpected [Err].
	fn parse_qualified_rule(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		let c = p.peek_n(1);
		Err(Diagnostic::new(c, Diagnostic::unexpected))?
	}

	/// Like [crate::Parse::parse()] but with the additional context that the next cursor is _not_ an
	/// [AtKeyword][crate::token_macros::AtKeyword], and that [RuleVariants::parse_qualified_rule()] has failed.
	/// Therefore this should attempt to parse an Unknown Qualified Rule, or [Err].
	///
	/// The default implementation of this method is to return an Unexpected [Err].
	fn parse_unknown_qualified_rule(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		let c = p.peek_n(1);
		Err(Diagnostic::new(c, Diagnostic::unexpected))?
	}

	/// If all of the parse steps have failed, including parsing the Unknown Qualified Rule, we may want to consume a bad
	/// declaration (especially if the parser is in a nested context). This is done automatically on failing to parse
	/// an Unknown Qualified Rule, and this method is given the [BadDeclaration].
	///
	/// This should attempt to build a Self that represents the [BadDeclaration], or return [None] so
	/// [RuleVariants::parse_rule_variants()] can [Err].
	///
	/// The default implementation of this method is to return [None].
	fn bad_declaration(_: BadDeclaration<'a>) -> Option<Self> {
		None
	}

	fn parse_rule_variants(p: &mut Parser<'a>) -> Result<Self> {
		let checkpoint = p.checkpoint();
		let c: Cursor = p.peek_n(1);
		if <T![AtKeyword]>::peek(p, c) {
			Self::parse_at_rule(p, c).or_else(|_| {
				p.rewind(checkpoint);
				Self::parse_unknown_at_rule(p, c)
			})
		} else {
			Self::parse_qualified_rule(p, c)
				.or_else(|_| {
					p.rewind(checkpoint);
					Self::parse_unknown_qualified_rule(p, c)
				})
				.or_else(|_| {
					p.rewind(checkpoint);
					let state = p.set_state(State::Nested);
					let declaration = p.parse::<BadDeclaration>();
					p.set_state(state);
					if let Some(s) = Self::bad_declaration(declaration?) {
						Ok(s)
					} else {
						Err(Diagnostic::new(c, Diagnostic::unexpected))?
					}
				})
		}
	}
}
