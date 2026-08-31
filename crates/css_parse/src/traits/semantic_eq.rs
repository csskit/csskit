use super::prelude::*;
use css_lexer::{AssociatedWhitespaceRules, Kind, KindSet, SourceCursor};

/// Trait for semantic equality comparison that ignores source positions and whitespace.
///
/// This trait provides semantic comparison for CSS AST nodes, comparing their structural
/// content and meaning rather than their exact representation in source code. Two nodes
/// are semantically equal if they represent the same CSS construct, regardless of source
/// position or trivia.
///
/// Both nodes must come from `source_text`, because a [Cursor] keeps only the facts which fit in a
/// [Token][css_lexer::Token] and a name too large for an atom stays in the source text.
pub trait SemanticEq {
	/// Returns `true` if `self` and `other`, both read from `source_text`, are semantically equal.
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool;
}

impl SemanticEq for Cursor {
	fn semantic_eq(&self, other: &Self, source_text: &str) -> bool {
		let kind = self.token().kind();
		if kind != other.token().kind() {
			return false;
		}
		if KindSet::NAMED.contains(kind) {
			// Only a named token needs its source text, so only a named token pays for the slice.
			let this = SourceCursor::from(*self, self.str_slice(source_text));
			let other = SourceCursor::from(*other, other.str_slice(source_text));
			return this.semantic_eq(&other, source_text);
		}
		self.token().with_associated_whitespace(AssociatedWhitespaceRules::NONE)
			== other.token().with_associated_whitespace(AssociatedWhitespaceRules::NONE)
	}
}

impl SemanticEq for SourceCursor<'_> {
	fn semantic_eq(&self, other: &Self, _source_text: &str) -> bool {
		let token = self.token();
		let other_token = other.token();
		let kind = token.kind();
		if kind != other_token.kind() {
			return false;
		}
		match kind {
			Kind::Ident | Kind::Function | Kind::AtKeyword => {
				token.is_dashed_ident() == other_token.is_dashed_ident()
					&& match (token.atom_bits(), other_token.atom_bits()) {
						(0, 0) => self.eq_parsed_ignore_ascii_case(other),
						(a, b) => a == b,
					}
			}
			Kind::Dimension => {
				token.value() == other_token.value()
					&& match (token.atom_bits(), other_token.atom_bits()) {
						(0, 0) => self.eq_parsed_ignore_ascii_case(other),
						// The token does not keep the dashed-ness of a unit, and the atom lookup for a
						// dashed unit skips the leading `--`, so equal atoms must also have equal unit
						// lengths to tell `1--px` from `1px`.
						(a, b) => {
							a == b && token.len() - token.leading_len() == other_token.len() - other_token.leading_len()
						}
					}
			}
			Kind::String | Kind::Url | Kind::Hash => self.eq_parsed(other),
			Kind::UnicodeRange => {
				token.unicode_range_start() == other_token.unicode_range_start()
					&& token.unicode_range_end() == other_token.unicode_range_end()
			}
			// The remaining named kinds, such as comments and the `Bad` kinds, keep no facts of what
			// they hold, so only their whole source text tells them apart.
			_ if KindSet::NAMED.contains(kind) => self.source() == other.source(),
			_ => {
				self.token().with_associated_whitespace(AssociatedWhitespaceRules::NONE)
					== other.token().with_associated_whitespace(AssociatedWhitespaceRules::NONE)
			}
		}
	}
}

impl<T> SemanticEq for Option<T>
where
	T: SemanticEq,
{
	fn semantic_eq(&self, s: &Self, source_text: &str) -> bool {
		match (self, s) {
			(Some(a), Some(b)) => a.semantic_eq(b, source_text),
			(None, None) => true,
			(_, _) => false,
		}
	}
}

impl<'a, T, A: Allocator> SemanticEq for Vec<'a, T, A>
where
	T: SemanticEq,
{
	fn semantic_eq(&self, s: &Self, source_text: &str) -> bool {
		if self.len() != s.len() {
			return false;
		}
		for i in 0..self.len() {
			if !self[i].semantic_eq(&s[i], source_text) {
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
            fn semantic_eq(&self, o: &Self, source_text: &str) -> bool {
                let ($($A),*) = self;
                let ($($B),*) = o;
                $($A.semantic_eq(&$B, source_text))&&*
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
	use crate::{ComponentValues, Parse, Parser, SimpleBlock, T, ToCursors, assert_semantic_eq, assert_semantic_ne};
	use css_lexer::{AtomSet, EmptyAtomSet};

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
		assert!(cursor1.semantic_eq(&cursor2, ""));

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
				plain.semantic_eq(&with_whitespace_rule, ""),
				"{:?} should be semantic_eq regardless of associated whitespace",
				token.kind()
			);
		}
	}

	fn pair<'a>(alloc: &'a Arena, source: &'a str) -> (ComponentValues<'a>, ComponentValues<'a>) {
		let (first, second) = parse::<(SimpleBlock, SimpleBlock)>(alloc, source);
		(first.values, second.values)
	}

	#[test]
	fn test_component_values_ignores_whitespace() {
		let alloc = Arena::new();
		let source = "(1px solid red)(1px  solid  red)";
		let (first, second) = pair(&alloc, source);
		assert!(first.semantic_eq(&second, source));
	}

	#[test]
	fn test_component_values_different_values() {
		let alloc = Arena::new();
		let source = "(1px solid red)(2px solid red)";
		let (first, second) = pair(&alloc, source);
		assert!(!first.semantic_eq(&second, source));
	}

	#[test]
	fn test_idents_of_equal_length_are_told_apart() {
		let alloc = Arena::new();
		let source = "(foo)(bar)";
		let (first, second) = pair(&alloc, source);
		assert!(!first.semantic_eq(&second, source));

		let source = "(foo)(foo)";
		let (first, second) = pair(&alloc, source);
		assert!(first.semantic_eq(&second, source));
	}

	#[test]
	fn test_strings_of_equal_length_are_told_apart() {
		let alloc = Arena::new();
		let source = "(\"i\")(\"j\")";
		let (first, second) = pair(&alloc, source);
		assert!(!first.semantic_eq(&second, source));
	}

	#[test]
	fn test_ident_spellings_are_equal() {
		let alloc = Arena::new();
		let source = "(Foo)(foo)";
		let (first, second) = pair(&alloc, source);
		assert!(first.semantic_eq(&second, source));

		let source = "(b\\61r)(bar)";
		let (first, second) = pair(&alloc, source);
		assert!(first.semantic_eq(&second, source));
	}

	#[test]
	fn test_string_spellings() {
		let alloc = Arena::new();
		let source = "(\"A\")(\"a\")";
		let (first, second) = pair(&alloc, source);
		assert!(!first.semantic_eq(&second, source));

		let source = "(\"a\")('a')";
		let (first, second) = pair(&alloc, source);
		assert!(first.semantic_eq(&second, source));

		let source = "(\"\\61\")(\"a\")";
		let (first, second) = pair(&alloc, source);
		assert!(first.semantic_eq(&second, source));
	}

	#[test]
	fn test_hashes_are_case_sensitive() {
		let alloc = Arena::new();
		let source = "(#Foo)(#foo)";
		let (first, second) = pair(&alloc, source);
		assert!(!first.semantic_eq(&second, source));
	}

	#[test]
	fn test_dimension_spellings_are_equal() {
		let alloc = Arena::new();
		let source = "(1.0Px)(1px)";
		let (first, second) = pair(&alloc, source);
		assert!(first.semantic_eq(&second, source));

		let source = "(1px)(2px)";
		let (first, second) = pair(&alloc, source);
		assert!(!first.semantic_eq(&second, source));
	}

	// The tests above use an empty atom set, so every ident and unit compares through the source
	// text. An atom set which holds them compares by atom instead.
	#[derive(Debug, Default, derive_atom_set::AtomSet, Copy, Clone, PartialEq)]
	pub enum TestAtomSet {
		#[default]
		_None,
		Foo,
		Bar,
		Px,
	}

	impl TestAtomSet {
		const ATOMS: TestAtomSet = TestAtomSet::_None;
	}

	#[test]
	fn test_ident_atoms() {
		assert_semantic_eq!(TestAtomSet::ATOMS, T![Ident], "Foo", "foo");
		assert_semantic_ne!(TestAtomSet::ATOMS, T![Ident], "foo", "bar");
		// A dashed ident is not the plain ident of the same atom.
		assert_semantic_ne!(TestAtomSet::ATOMS, T![Ident], "--foo", "foo");
	}

	#[test]
	fn test_dimension_unit_atoms() {
		assert_semantic_eq!(TestAtomSet::ATOMS, T![Dimension], "1.0Px", "1px");
		assert_semantic_ne!(TestAtomSet::ATOMS, T![Dimension], "1px", "2px");
		// The atom of a dashed unit skips the leading `--`, so the units must also be equally long.
		assert_semantic_ne!(TestAtomSet::ATOMS, T![Dimension], "1--px", "1px");
	}
}
