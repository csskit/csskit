use crate::{
	BadDeclaration, Block, CursorSink, DeclarationValue, Parse, Parser, Peek, Result, State, T, ToCursors, diagnostics,
};
use css_lexer::{Kind, KindSet, ToSpan};
use csskit_derives::ToSpan;

#[derive(ToSpan, Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize), serde(tag = "type"))]
pub struct QualifiedRule<'a, P, D, R>
where
	P: Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R>: Parse<'a> + ToCursors + ToSpan,
{
	pub prelude: P,
	pub block: Block<'a, D, R>,
}

impl<'a, P, D, R> Peek<'a> for QualifiedRule<'a, P, D, R>
where
	P: Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R>: Parse<'a> + ToCursors + ToSpan,
{
	fn peek(p: &Parser<'a>, c: css_lexer::Cursor) -> bool {
		<P>::peek(p, c)
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
/// A QualifiedRule represents a block with a prelude which may contain other rules.
/// Examples of QualifiedRules are StyleRule, KeyframeRule (no s!).
impl<'a, P, D, R> Parse<'a> for QualifiedRule<'a, P, D, R>
where
	P: Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R>: Parse<'a> + ToCursors + ToSpan,
{
	fn parse(p: &mut Parser<'a>) -> Result<Self> {
		// Let rule be a new qualified rule with its prelude, declarations, and child rules all initially set to empty lists.

		// Process input:

		// <EOF-token>
		// stop token (if passed)
		//   This is a parse error. Return nothing.
		if p.at_end() {
			Err(diagnostics::UnexpectedEnd())?
		}

		// <}-token>
		//   This is a parse error. If nested is true, return nothing. Otherwise, consume a token and append the result to rule’s prelude.
		if p.is(State::Nested) && p.peek::<T!['}']>() {
			Err(diagnostics::UnexpectedCloseCurly(p.peek_n(1).into()))?;
		}

		// <{-token>
		//	If the first two non-<whitespace-token> values of rule’s prelude are an <ident-token> whose value starts with "--" followed by a <colon-token>, then:
		let checkpoint = p.checkpoint();
		if p.peek::<T![DashedIdent]>() {
			p.parse::<T![DashedIdent]>().ok();
			if p.peek::<T![:]>() {
				// If nested is true, consume the remnants of a bad declaration from input, with nested set to true, and return nothing.
				if p.is(State::Nested) {
					p.rewind(checkpoint);
					p.parse::<BadDeclaration>()?;
					Err(diagnostics::BadDeclaration(checkpoint.to_span()))?
				// If nested is false, consume a block from input, and return nothing.
				} else {
					// QualifiedRules must be able to consume a block from their input when encountering
					// a custom property like declaration that doesn't end but opens a `{` block. This
					// is implemented as parsing the existing block as that' simplifies downstream logic
					// but consumers of this trait can instead opt to implement an optimised version of
					// this which doesn't build up an AST and just throws away tokens.
					p.parse::<Block<'a, D, R>>()?;
					Err(diagnostics::BadDeclaration(checkpoint.to_span()))?
				}
			}
			p.rewind(checkpoint);
		}

		// Set the StopOn Curly to signify to prelude parsers that they shouldn't consume beyond the curly
		let old_stop = p.set_stop(KindSet::new(&[Kind::LeftCurly]));
		let prelude = p.parse::<P>();
		p.set_stop(old_stop);
		let prelude = prelude?;

		// Otherwise, consume a block from input, and let child rules be the result.
		// If the first item of child rules is a list of declarations,
		// remove it from child rules and assign it to rule’s declarations.
		// If any remaining items of child rules are lists of declarations,
		// replace them with nested declarations rules containing the list as its sole child.
		// Assign child rules to rule’s child rules.
		Ok(Self { prelude, block: p.parse::<Block<'a, D, R>>()? })
	}
}

impl<'a, P, D, R> ToCursors for QualifiedRule<'a, P, D, R>
where
	P: Peek<'a> + Parse<'a> + ToCursors + ToSpan,
	D: DeclarationValue<'a>,
	R: Parse<'a> + ToCursors + ToSpan,
	Block<'a, D, R>: Parse<'a> + ToCursors + ToSpan,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.prelude, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;
	use css_lexer::Cursor;

	#[derive(Debug, ToSpan)]
	struct Decl(T![Ident]);
	impl<'a> DeclarationValue<'a> for Decl {
		type ComputedValue = T![Eof];

		fn is_initial(&self) -> bool {
			false
		}

		fn is_inherit(&self) -> bool {
			false
		}

		fn is_unset(&self) -> bool {
			false
		}

		fn is_revert(&self) -> bool {
			false
		}

		fn is_revert_layer(&self) -> bool {
			false
		}

		fn needs_computing(&self) -> bool {
			false
		}

		fn parse_specified_declaration_value(p: &mut Parser<'a>, _: Cursor) -> Result<Self> {
			p.parse::<T![Ident]>().map(Self)
		}
	}
	impl ToCursors for Decl {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			ToCursors::to_cursors(&self.0, s);
		}
	}

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<QualifiedRule<T![Ident], Decl, T![Ident]>>(), 112);
	}

	#[test]
	fn test_writes() {
		assert_parse!(QualifiedRule<T![Ident], Decl, T![Ident]>, "body{color:black}");
	}
}
