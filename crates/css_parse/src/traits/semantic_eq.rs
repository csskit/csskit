use css_lexer::{Cursor, Kind};

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
		// For Delims, we ignore associated whitespace rules since
		// those are formatting hints, not semantic content
		match self.token().kind() {
			Kind::Delim => {
				self.token().with_associated_whitespace(css_lexer::AssociatedWhitespaceRules::none())
					== other.token().with_associated_whitespace(css_lexer::AssociatedWhitespaceRules::none())
			}
			_ => self.token() == other.token(),
		}
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

impl<'a, T> SemanticEq for bumpalo::collections::Vec<'a, T>
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
	use crate::{ComponentValues, Parse, Parser, ToCursors};
	use bumpalo::Bump;
	use css_lexer::EmptyAtomSet;

	fn parse<'a, T: Parse<'a> + ToCursors>(bump: &'a Bump, source: &'a str) -> T {
		let lexer = css_lexer::Lexer::new(&EmptyAtomSet::ATOMS, source);
		let mut parser = Parser::new(bump, source, lexer);
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
	fn test_component_values_ignores_whitespace() {
		let source1 = "1px solid red";
		let source2 = "1px  solid  red"; // Extra whitespace

		let bump = Bump::new();
		let values1 = parse::<ComponentValues>(&bump, source1);
		let values2 = parse::<ComponentValues>(&bump, source2);

		// Semantically equal despite whitespace
		assert!(values1.semantic_eq(&values2));
	}

	#[test]
	fn test_component_values_different_values() {
		let source1 = "1px solid red";
		let source2 = "2px solid red";

		let bump = Bump::new();
		let values1 = parse::<ComponentValues>(&bump, source1);
		let values2 = parse::<ComponentValues>(&bump, source2);

		// Should NOT be equal due to different values
		assert!(!values1.semantic_eq(&values2));
	}
}
