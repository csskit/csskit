use crate::{
	BadDeclaration, Block, Cursor, CursorSink, DeclarationValue, Diagnostic, Kind, KindSet, NodeMetadata,
	NodeWithMetadata, Parse, Parser, Peek, Result, SemanticEq, Span, State, T, ToCursors, ToSpan,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
#[cfg_attr(
	feature = "serde",
	serde(bound(serialize = "P: serde::Serialize, D: serde::Serialize, R: serde::Serialize"))
)]
pub struct QualifiedRule<'a, P, D, R, M>
where
	// TODO: P: NodeWithMetadata<M>,
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	pub prelude: P,
	pub block: Block<'a, D, R, M>,
	#[cfg_attr(feature = "serde", serde(skip))]
	meta: M,
}

impl<'a, P, D, R, M> NodeWithMetadata<M> for QualifiedRule<'a, P, D, R, M>
where
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn metadata(&self) -> M {
		self.meta
	}
}

impl<'a, P, D, R, M> Peek<'a> for QualifiedRule<'a, P, D, R, M>
where
	P: Peek<'a>,
	D: DeclarationValue<'a, M>,
	M: NodeMetadata,
{
	fn peek<Iter>(p: &Parser<'a, Iter>, c: Cursor) -> bool
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		<P>::peek(p, c)
	}
}

// https://drafts.csswg.org/css-syntax-3/#consume-a-qualified-rule
/// A QualifiedRule represents a block with a prelude which may contain other rules.
/// Examples of QualifiedRules are StyleRule, KeyframeRule (no s!).
impl<'a, P, D, R, M> Parse<'a> for QualifiedRule<'a, P, D, R, M>
where
	D: DeclarationValue<'a, M>,
	P: Parse<'a>,
	R: Parse<'a> + NodeWithMetadata<M> + crate::RuleVariants<'a, DeclarationValue = D, Metadata = M>,
	M: NodeMetadata,
{
	fn parse<Iter>(p: &mut Parser<'a, Iter>) -> Result<Self>
	where
		Iter: Iterator<Item = crate::Cursor> + Clone,
	{
		let c = p.peek_n(1);
		// Let rule be a new qualified rule with its prelude, declarations, and child rules all initially set to empty lists.

		// Process input:

		// <EOF-token>
		// stop token (if passed)
		//   This is a parse error. Return nothing.
		if p.at_end() {
			Err(Diagnostic::new(p.peek_n(1), Diagnostic::unexpected_end))?
		}

		// <}-token>
		//   This is a parse error. If nested is true, return nothing. Otherwise, consume a token and append the result to rule’s prelude.
		if p.is(State::Nested) && <T!['}']>::peek(p, c) {
			Err(Diagnostic::new(c, Diagnostic::unexpected_close_curly))?;
		}

		// <{-token>
		//	If the first two non-<whitespace-token> values of rule’s prelude are an <ident-token> whose value starts with "--" followed by a <colon-token>, then:
		let checkpoint = p.checkpoint();
		if <T![DashedIdent]>::peek(p, c) {
			p.parse::<T![DashedIdent]>().ok();
			if <T![:]>::peek(p, p.peek_n(1)) {
				// If nested is true, consume the remnants of a bad declaration from input, with nested set to true, and return nothing.
				if p.is(State::Nested) {
					p.rewind(checkpoint.clone());
					let start = p.peek_n(1);
					p.parse::<BadDeclaration>()?;
					let end = p.peek_n(0);
					Err(Diagnostic::new(start, Diagnostic::bad_declaration).with_end_cursor(end))?
				// If nested is false, consume a block from input, and return nothing.
				} else {
					// QualifiedRules must be able to consume a block from their input when encountering
					// a custom property like declaration that doesn't end but opens a `{` block. This
					// is implemented as parsing the existing block as that' simplifies downstream logic
					// but consumers of this trait can instead opt to implement an optimised version of
					// this which doesn't build up an AST and just throws away tokens.
					p.parse::<Block<'a, D, R, M>>()?;
					let start = p.peek_n(1);
					p.parse::<BadDeclaration>()?;
					let end = p.peek_n(0);
					Err(Diagnostic::new(start, Diagnostic::bad_declaration).with_end_cursor(end))?
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
		let block = p.parse::<Block<'a, D, R, M>>()?;
		let meta = block.metadata();
		Ok(Self { prelude, block, meta })
	}
}

impl<'a, P, D, R, M> ToCursors for QualifiedRule<'a, P, D, R, M>
where
	D: DeclarationValue<'a, M> + ToCursors,
	P: ToCursors,
	R: ToCursors,
	M: NodeMetadata,
{
	fn to_cursors(&self, s: &mut impl CursorSink) {
		ToCursors::to_cursors(&self.prelude, s);
		ToCursors::to_cursors(&self.block, s);
	}
}

impl<'a, P, D, R, M> ToSpan for QualifiedRule<'a, P, D, R, M>
where
	D: DeclarationValue<'a, M> + ToSpan,
	P: ToSpan,
	R: ToSpan,
	M: NodeMetadata,
{
	fn to_span(&self) -> Span {
		self.prelude.to_span() + self.block.to_span()
	}
}

impl<'a, P, D, R, M> SemanticEq for QualifiedRule<'a, P, D, R, M>
where
	D: DeclarationValue<'a, M> + SemanticEq,
	P: SemanticEq,
	R: SemanticEq,
	M: NodeMetadata,
{
	fn semantic_eq(&self, other: &Self) -> bool {
		self.prelude.semantic_eq(&other.prelude) && self.block.semantic_eq(&other.block)
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::{EmptyAtomSet, test_helpers::*};

	#[derive(Debug)]
	struct Decl(T![Ident]);

	impl NodeWithMetadata<()> for Decl {
		fn metadata(&self) {}
	}

	impl<'a> DeclarationValue<'a, ()> for Decl {
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

		fn parse_specified_declaration_value<Iter>(p: &mut Parser<'a, Iter>, _: Cursor) -> Result<Self>
		where
			Iter: Iterator<Item = crate::Cursor> + Clone,
		{
			p.parse::<T![Ident]>().map(Self)
		}
	}

	impl ToCursors for Decl {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			ToCursors::to_cursors(&self.0, s);
		}
	}

	impl ToSpan for Decl {
		fn to_span(&self) -> Span {
			self.0.to_span()
		}
	}

	impl SemanticEq for Decl {
		fn semantic_eq(&self, other: &Self) -> bool {
			self.0.semantic_eq(&other.0)
		}
	}

	#[derive(Debug)]
	struct Rule(T![Ident]);

	impl<'a> Parse<'a> for Rule {
		fn parse<I>(p: &mut Parser<'a, I>) -> Result<Self>
		where
			I: Iterator<Item = Cursor> + Clone,
		{
			Ok(Self(p.parse::<T![Ident]>()?))
		}
	}

	impl ToCursors for Rule {
		fn to_cursors(&self, s: &mut impl CursorSink) {
			ToCursors::to_cursors(&self.0, s);
		}
	}

	impl ToSpan for Rule {
		fn to_span(&self) -> Span {
			self.0.to_span()
		}
	}

	impl NodeWithMetadata<()> for Rule {
		fn metadata(&self) {}
	}

	impl<'a> crate::RuleVariants<'a> for Rule {
		type DeclarationValue = Decl;
		type Metadata = ();
	}

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<QualifiedRule<T![Ident], Decl, Rule, ()>>(), 112);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, QualifiedRule<T![Ident], Decl, Rule, ()>, "body{color:black}");
	}
}
