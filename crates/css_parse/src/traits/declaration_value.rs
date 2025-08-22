use crate::{Parser, Peek, Result, T, ToCursors, diagnostics};
use css_lexer::{Cursor, KindSet, ToSpan};

/// A trait that can be used for AST nodes representing a Declaration's Value. It offers some
/// convenience functions for handling such values.
pub trait DeclarationValue<'a>: Sized + ToCursors + ToSpan {
	type ComputedValue: Peek<'a>;

	/// Determines if the given [Cursor] represents a valid [Ident][crate::token_macros::Ident] matching a known property
	/// name.
	///
	/// If implementing a set of declarations where ony limited property-ids are valid (such as the declarations allowed
	/// by an at-rule) then it might be worthwhile changing this to sometimes return `false`, which consumers of this
	/// trait can use to error early without having to do too much backtracking.
	fn valid_declaration_name(_p: &Parser<'a>, _c: Cursor) -> bool {
		true
	}

	/// Determines if the parsed Self was parsed as an unknown value.
	///
	/// If implementing a set of declarations where any name is accepted, or where the value might result in re-parsing
	/// as unknown, this method can be used to signal that to upstream consumers of this trait. By default this returns
	/// `false` because `valid_declaration_name` returns `true`, the assumption being that any successful construction of
	/// Self is indeed a valid and known declaration.
	fn is_unknown(&self) -> bool {
		false
	}

	/// Determines if the parsed Self was parsed as a Custom value.
	///
	/// If implementing a set of declarations where custom names are accepted, or where the value might result in
	/// re-parsing as unknown, this method can be used to signal that to upstream consumers of this trait. By default
	/// this returns `false` because `valid_declaration_name` returns `true`, the assumption being that any successful
	/// construction of Self is indeed a valid and known declaration.
	fn is_custom(&self) -> bool {
		false
	}

	/// Determines if the parsed Self was parsed as the "initial" keyword.
	///
	/// If implementing a set of declarations where the "initial" keyword is accepted this method can be used to signal
	/// that to upstream consumers of this trait.
	fn is_initial(&self) -> bool;

	/// Determines if the parsed Self was parsed as the "inherit" keyword.
	///
	/// If implementing a set of declarations where the "inherit" keyword is accepted this method can be used to signal
	/// that to upstream consumers of this trait.
	fn is_inherit(&self) -> bool;

	/// Determines if the parsed Self was parsed as the "unset" keyword.
	///
	/// If implementing a set of declarations where the "unset" keyword is accepted this method can be used to signal
	/// that to upstream consumers of this trait.
	fn is_unset(&self) -> bool;

	/// Determines if the parsed Self was parsed as the "revert" keyword.
	///
	/// If implementing a set of declarations where the "revert" keyword is accepted this method can be used to signal
	/// that to upstream consumers of this trait.
	fn is_revert(&self) -> bool;

	/// Determines if the parsed Self was parsed as the "revert" keyword.
	///
	/// If implementing a set of declarations where the "revert" keyword is accepted this method can be used to signal
	/// that to upstream consumers of this trait.
	fn is_revert_layer(&self) -> bool;

	/// Determines if the parsed Self is not a valid literal production of the grammar, and instead some of its
	/// constituent parts will need additional computation to reify into a known value.
	///
	/// CSS properties are allowed to include substitutions, such as `calc()` or `var()`. These are not defined in the
	/// declaration's grammar but are instead stored so that when a style object is reified the declarations that had
	/// those tokens can be recomputed against the context of their node.
	fn needs_computing(&self) -> bool;

	/// Like `parse()` but with the additional context of the `name` [Cursor]. This cursor is known to be dashed ident,
	/// therefore this should return a `Self` reflecting a Custom property. Alternatively, if this DeclarationValue
	/// disallows custom declarations then this is the right place to return a parse Error.
	///
	/// The default implementation of this method is to return an Unexpected Err.
	fn parse_custom_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		let c = p.peek_n(1);
		Err(diagnostics::Unexpected(c.into(), c.into()))?
	}

	/// Like `parse()` but with the additional context of the `name` [Cursor]. This is only called before verifying that
	/// the next token was peeked to be a ComputedValue, therefore this should return a `Self` reflecting a Computed
	/// property. Alternatively, if this DeclarationValue disallows computed declarations then this is the right place to
	/// return a parse Error.
	///
	/// The default implementation of this method is to return an Unexpected Err.
	fn parse_computed_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		let c = p.peek_n(1);
		Err(diagnostics::Unexpected(c.into(), c.into()))?
	}

	/// Like `parse()` but with the additional context of the `name` [Cursor]. This is only called on values that are
	/// assumed to be _specified_, that is, they're not custom and not computed. Therefore this should return a `Self`
	/// reflecting a specified value. If this results in a Parse error then ComputedValue will be checked to see if the
	/// parser stopped because it saw a computed value function. If this results in a success, the next token is still
	/// checked as it may be a ComputedValue, which - if so - the parsed value will be discarded, and the parser rewound
	/// to re-parse this as a ComputedValue.
	///
	/// The default implementation of this method is to return an Unexpected Err.
	fn parse_specified_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		let c = p.peek_n(1);
		Err(diagnostics::Unexpected(c.into(), c.into()))?
	}

	/// Like `parse()` but with the additional context of the `name` [Cursor]. This is only called on values that are
	/// didn't parse as either a Custom, Computed or Specified value therefore this should return a `Self` reflecting an
	/// unknown property, or alternatively the right place to return a parse error.
	///
	/// The default implementation of this method is to return an Unexpected Err.
	fn parse_unknown_declaration_value(p: &mut Parser<'a>, _name: Cursor) -> Result<Self> {
		let c = p.peek_n(1);
		Err(diagnostics::Unexpected(c.into(), c.into()))?
	}

	// Like `parse()` but with the additional context of the `name` [Cursor] - the same [Cursor]
	// passed to [DeclarationValue::valid_declaration_name()].
	fn parse_declaration_value(p: &mut Parser<'a>, name: Cursor) -> Result<Self> {
		if name.token().is_dashed_ident() {
			return Self::parse_custom_declaration_value(p, name);
		}
		if !Self::valid_declaration_name(p, name) {
			return Self::parse_unknown_declaration_value(p, name);
		}
		if p.peek::<Self::ComputedValue>() {
			return Self::parse_computed_declaration_value(p, name);
		}
		let checkpoint = p.checkpoint();
		if let Ok(val) = Self::parse_specified_declaration_value(p, name) {
			if p.at_end() || p.peek_n(1) == KindSet::RIGHT_CURLY_OR_SEMICOLON || p.peek::<T![!]>() {
				return Ok(val);
			}
		}
		if p.peek::<Self::ComputedValue>() {
			p.rewind(checkpoint);
			if let Ok(val) = Self::parse_computed_declaration_value(p, name) {
				return Ok(val);
			}
		}
		p.rewind(checkpoint);
		Self::parse_unknown_declaration_value(p, name)
	}
}
