use crate::Vec;
use allocator_api2::alloc::Allocator;
use css_lexer::{AssociatedWhitespaceRules, Cursor};

/// Trait for semantic equality comparison that ignores source positions and whitespace.
///
/// This trait provides semantic comparison for CSS AST nodes, comparing their structural
/// content and meaning rather than their exact representation in source code. Two nodes
/// are semantically equal if they represent the same CSS construct, regardless of source
/// position or trivia.
pub trait SemanticEq {
	/// Returns `true` if `self` and `other` are semantically equal.
	fn semantic_eq(&self, other: &Self) -> bool;
}

// Implement for Cursor - compare tokens without considering source offset
impl SemanticEq for Cursor {
	fn semantic_eq(&self, other: &Self) -> bool {
		// Associated whitespace rules are formatting hints, not semantic content, so ignore
		// them. `with_associated_whitespace` is a no-op for kinds that don't carry such rules
		// (only "delim-like" kinds do: Delim, Colon, Semicolon, Comma, and the paren/curly/
		// square brackets), so this is safe to apply unconditionally.
		self.token().with_associated_whitespace(AssociatedWhitespaceRules::none())
			== other.token().with_associated_whitespace(AssociatedWhitespaceRules::none())
	}
}

impl<T> SemanticEq for Option<T>
where
	T: SemanticEq,
{
	fn semantic_eq(&self, s: &Self) -> bool {
		match (self, s) {
			(Some(a), Some(b)) => a.semantic_eq(b),
			(None, None) => true,
			(_, _) => false,
		}
	}
}

impl<'a, T, A: Allocator> SemanticEq for Vec<'a, T, A>
where
	T: SemanticEq,
{
	fn semantic_eq(&self, s: &Self) -> bool {
		if self.len() != s.len() {
			return false;
		}
		for i in 0..self.len() {
			if !self[i].semantic_eq(&s[i]) {
				return false;
			}
		}
		true
	}
}

macro_rules! impl_tuple {
		($($T:ident [ $A:ident, $B:ident ]),+) => {
        impl<$($T),*> SemanticEq for ($($T),*)
        where
            $($T: SemanticEq,)*
        {
            fn semantic_eq(&self, o: &Self) -> bool {
                let ($($A),*) = self;
                let ($($B),*) = o;
                $($A.semantic_eq(&$B))&&*
            }
        }
    };
}

impl_tuple!(A[sa,oa], B[sb,ob]);
impl_tuple!(A[sa,oa], B[sb,ob], C[sc,oc]);
impl_tuple!(A[sa,oa], B[sb,ob], C[sc,oc], D[sd,od]);
impl_tuple!(A[sa,oa], B[sb,ob], C[sc,oc], D[sd,od], E[se,oe]);
impl_tuple!(A[sa,oa], B[sb,ob], C[sc,oc], D[sd,od], E[se,oe], F[sf,of]);
impl_tuple!(A[sa,oa], B[sb,ob], C[sc,oc], D[sd,od], E[se,oe], F[sf,of], G[sg,og]);
impl_tuple!(A[sa,oa], B[sb,ob], C[sc,oc], D[sd,od], E[se,oe], F[sf,of], G[sg,og], H[sh,oh]);
impl_tuple!(A[sa,oa], B[sb,ob], C[sc,oc], D[sd,od], E[se,oe], F[sf,of], G[sg,og], H[sh,oh], I[si,oi]);
impl_tuple!(A[sa,oa], B[sb,ob], C[sc,oc], D[sd,od], E[se,oe], F[sf,of], G[sg,og], H[sh,oh], I[si,oi], J[sj,oj]);
impl_tuple!(A[sa,oa], B[sb,ob], C[sc,oc], D[sd,od], E[se,oe], F[sf,of], G[sg,og], H[sh,oh], I[si,oi], J[sj,oj], K[sk,ok]);
impl_tuple!(A[sa,oa], B[sb,ob], C[sc,oc], D[sd,od], E[se,oe], F[sf,of], G[sg,og], H[sh,oh], I[si,oi], J[sj,oj], K[sk,ok], L[sl,ol]);

#[cfg(test)]
mod tests {
	use super::*;
	use crate::Arena;
	use crate::{ComponentValues, Parse, Parser, ToCursors};
	use css_lexer::EmptyAtomSet;

	fn parse<'a, T: Parse<'a> + ToCursors>(alloc: &'a Arena, source: &'a str) -> T {
		let lexer = css_lexer::Lexer::new(&EmptyAtomSet::ATOMS, source);
		let mut parser = Parser::new(alloc, source, lexer);
		let result = parser.parse_entirely::<T>();
		result.output.unwrap()
	}

	#[test]
	fn test_cursor_semantic_eq_ignores_offset() {
		let token = css_lexer::Token::COMMA;
		let cursor1 = Cursor::new(css_lexer::SourceOffset(0), token);
		let cursor2 = Cursor::new(css_lexer::SourceOffset(100), token);

		// Should be semantically equal despite different offsets
		assert!(cursor1.semantic_eq(&cursor2));

		// Standard PartialEq should distinguish them
		assert_ne!(cursor1, cursor2);
	}

	#[test]
	fn test_cursor_semantic_eq_ignores_associated_whitespace_for_all_delim_like_kinds() {
		// Colon, Semicolon, Comma, and the paren/curly/square brackets share Delim's bit
		// layout and can also carry associated-whitespace formatting hints. Those hints are
		// not semantic content, so two tokens differing only by them must compare equal.
		for token in [
			css_lexer::Token::COLON,
			css_lexer::Token::SEMICOLON,
			css_lexer::Token::COMMA,
			css_lexer::Token::LEFT_PAREN,
			css_lexer::Token::RIGHT_PAREN,
			css_lexer::Token::LEFT_CURLY,
			css_lexer::Token::RIGHT_CURLY,
			css_lexer::Token::LEFT_SQUARE,
			css_lexer::Token::RIGHT_SQUARE,
		] {
			let plain = Cursor::new(css_lexer::SourceOffset(0), token);
			let with_whitespace_rule = Cursor::new(
				css_lexer::SourceOffset(0),
				token.with_associated_whitespace(css_lexer::AssociatedWhitespaceRules::EnforceBefore),
			);
			assert!(
				plain.semantic_eq(&with_whitespace_rule),
				"{:?} should be semantic_eq regardless of associated whitespace",
				token.kind()
			);
		}
	}

	#[test]
	fn test_component_values_ignores_whitespace() {
		let source1 = "1px solid red";
		let source2 = "1px  solid  red"; // Extra whitespace

		let alloc = Arena::new();
		let values1 = parse::<ComponentValues>(&alloc, source1);
		let values2 = parse::<ComponentValues>(&alloc, source2);

		// Semantically equal despite whitespace
		assert!(values1.semantic_eq(&values2));
	}

	#[test]
	fn test_component_values_different_values() {
		let source1 = "1px solid red";
		let source2 = "2px solid red";

		let alloc = Arena::new();
		let values1 = parse::<ComponentValues>(&alloc, source1);
		let values2 = parse::<ComponentValues>(&alloc, source2);

		// Should NOT be equal due to different values
		assert!(!values1.semantic_eq(&values2));
	}
}
