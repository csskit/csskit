use crate::{Result, ToCursors, parser::Parser};
use css_lexer::{Cursor, ToSpan};

/// A trait that can be used for AST nodes representing a Declaration's Value. It offers some
/// convenience functions for handling such values.
pub trait DeclarationValue<'a>: Sized + ToCursors + ToSpan {
	/// Determines if the given [Cursor] represents a valid [Ident][crate::token_macros::Ident]
	/// matching a known property name.
	///
	/// If implementing a set of declarations where ony limited property-ids are valid (such as the
	/// declarations allowed by an at-rule) then it might be worthwhile changing this to sometimes
	/// return `false`, which consumers of this trait can use to error early without having to do too
	/// much backtracking.
	fn valid_declaration_name(_p: &Parser<'a>, _c: Cursor) -> bool {
		true
	}

	/// Determines if the parsed Self was parsed as an unknown value.
	///
	/// If implementing a set of declarations where any name is accepted, or where the value might
	/// result in re-parsing as unknown, this method can be used to signal that to upstream consumers
	/// of this trait. By default this returns `false` because `valid_declaration_name` returns
	/// `true`, the assumption being that any successful construction of Self is indeed a valid and
	/// known declaration.
	fn is_unknown(&self) -> bool {
		false
	}

	/// Determines if the parsed Self is not a valid literal production of the grammar, and instead
	/// some of its constituent parts will need additional computation to reify into a known value.
	///
	/// CSS properties are allowed to include substitutions, such as `calc()` or `var()`. These are
	/// not defined in the declaration's grammar but are instead stored so that when a style object
	/// is reified the declarations that had those tokens can be recomputed against the context of
	/// their node.
	fn needs_computing(&self) -> bool;

	// Like `parse()` but with the additional context of the `name` [Cursor] - the same [Cursor]
	// passed to [DeclarationValue::valid_declaration_name()].
	fn parse_declaration_value(p: &mut Parser<'a>, name: Cursor) -> Result<Self>;
}
