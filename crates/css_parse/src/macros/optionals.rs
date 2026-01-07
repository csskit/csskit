use crate::{Cursor, CursorSink, Parse, Parser, Peek, Result as ParserResult, SemanticEq, Span, ToCursors, ToSpan};

macro_rules! impl_optionals {
	($($name:ident, ($($T:ident[ $A:ident, $B:ident ]),+))+) => {
		$(
			#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
			#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
			pub struct $name<$($T),+>($(pub Option<$T>),+);

			impl<'a, $($T),+> Peek<'a> for $name<$($T),+>
			where
				$($T: Parse<'a> + Peek<'a>,)+
			{
				fn peek<I>(p: &Parser<'a, I>, c: Cursor) -> bool
				where
					I: Iterator<Item = crate::Cursor> + Clone,
				{
					$($T::peek(p, c) ||)+ false
				}
			}

			impl<'a, $($T),+> Parse<'a> for $name<$($T),+>
			where
				$($T: Parse<'a> + Peek<'a>,)+
			{
				fn parse<I>(p: &mut Parser<'a, I>) -> ParserResult<Self>
				where
					I: Iterator<Item = crate::Cursor> + Clone,
				{
					let ($($A),+) = parse_optionals!(p, $($A:$T),+);
					Ok(Self($($A),+))
				}
			}

			impl<'a, $($T),+> ToCursors for $name<$($T),+>
			where
				$($T: ToCursors,)+
			{
				fn to_cursors(&self, s: &mut impl CursorSink) {
					let $name($($A),+) = self;
					$($A.to_cursors(s);)+
			 }
			}

			impl<$($T),+> ToSpan for  $name<$($T),+>
			where
				$($T: ToSpan,)+
			{
				fn to_span(&self) -> Span {
					let $name($($A),+) = self;
					Span::DUMMY $(+$A.to_span())+
				}
			}

			impl<$($T),+> SemanticEq for  $name<$($T),+>
			where
				$($T: SemanticEq,)+
			{
				fn semantic_eq(&self, o: &Self) -> bool {
					let $name($($A),+) = self;
					let $name($($B),+) = o;
					$($A.semantic_eq($B))&&+
				}
			}

			impl<$($T),+> From<$name<$($T),+>> for ($(Option<$T>),+)
			{
				fn from(value: $name<$($T),+>) -> Self {
					let $name($($A),+) = value;
					($($A),+)
				}
			}

			impl<$($T),+> From<($(Option<$T>),+)> for $name<$($T),+>
			{
				fn from(value: ($(Option<$T>),+)) -> Self {
					let ($($A),+) = value;
					Self($($A),+)
				}
			}
		)+
	};
}

#[macro_export]
macro_rules! parse_optionals {
	($p: ident, $($name:ident: $T:ty),+) => {
		{
			$(let mut $name: Option<$T> = None;)+

			while $($name.is_none())||+ {
				$(
					if $name.is_none() {
							$name = $p.parse_if_peek::<$T>()?;
							if $name.is_some() { continue; }
					}
				)+

				break;
			}

			if $($name.is_none())&&+ {
				Err($crate::Diagnostic::new($p.next(), $crate::Diagnostic::unexpected))?
			}

			($($name),+)
		 }
	};
}

/// A helper type for parsing optional CSS grammar patterns where items can appear in any order
/// but at most once each (the `||` combinator in CSS grammar).
///
/// # Example
/// ```ignore
/// // For CSS grammar: [ foo || <number> ]
/// let (foo, num) = p.parse::<Optionals![Ident, Number]>()?;
/// ```
#[macro_export]
macro_rules! Optionals {
	($t:ty) => { compile_error!("Use Option<T> dummy"); };
	($t:ty, $u:ty) => { $crate::Optionals2<$t, $u> };
	($t:ty, $u:ty, $v:ty) => { $crate::Optionals3<$t, $u, $v> };
	($t:ty, $u:ty, $v:ty, $w:ty) => { $crate::Optionals4<$t, $u, $v, $w> };
	($t:ty, $u:ty, $v:ty, $w:ty, $x:ty) => { $crate::Optionals5<$t, $u, $v, $w, $x> };
}

impl_optionals! {
	Optionals2, (A[sa, oa], B[sb, ob])
	Optionals3, (A[sa, oa], B[sb, ob], C[sc, oc])
	Optionals4, (A[sa, oa], B[sb, ob], C[sc, oc], D[sd, od])
	Optionals5, (A[sa, oa], B[sb, ob], C[sc, oc], D[sd, od], E[se, oe])
}

#[cfg(test)]
mod tests {

	use super::*;
	use crate::{EmptyAtomSet, test_helpers::*, token_macros::*};

	type CaseA = Optionals![Number, Ident];
	type CaseB = Optionals![Number, Ident, String];
	type CaseC = Optionals![Number, Ident, String, Ident];
	type CaseD = Optionals![Number, Ident, String, Ident, Dimension];

	#[test]
	fn size_test() {
		assert_eq!(std::mem::size_of::<Optionals2<Ident, Number>>(), 32);
	}

	#[test]
	fn test_writes() {
		assert_parse!(EmptyAtomSet::ATOMS, CaseA, "123 foo", Optionals2(Some(_), Some(_)));
		assert_parse!(EmptyAtomSet::ATOMS, CaseA, "foo 123", Optionals2(Some(_), Some(_)));
		assert_parse!(EmptyAtomSet::ATOMS, CaseA, "123", Optionals2(Some(_), None));
		assert_parse!(EmptyAtomSet::ATOMS, CaseA, "foo", Optionals2(None, Some(_)));

		assert_parse!(EmptyAtomSet::ATOMS, CaseB, "123 foo 'bar'", Optionals3(Some(_), Some(_), Some(_)));
		// assert_parse!(EmptyAtomSet::ATOMS, CaseB, "foo 'bar' 123", Optionals3(Some(_), Some(_), Some(_)));
		assert_parse!(EmptyAtomSet::ATOMS, CaseB, "123", Optionals3(Some(_), None, None));
		assert_parse!(EmptyAtomSet::ATOMS, CaseB, "'foo'", Optionals3(None, None, Some(_)));

		assert_parse!(EmptyAtomSet::ATOMS, CaseC, "foo 123 bar 'bar'", Optionals4(Some(_), Some(_), Some(_), Some(_)));
	}

	#[test]
	fn test_spans() {
		assert_parse_span!(
			EmptyAtomSet::ATOMS,
			CaseA,
			r#"
			foo 123 bar
			^^^^^^^
		"#
		);

		assert_parse_span!(
			EmptyAtomSet::ATOMS,
			CaseA,
			r#"
			123 foo bar
			^^^^^^^
		"#
		);

		assert_parse_span!(
			EmptyAtomSet::ATOMS,
			CaseA,
			r#"
			123 'foo'
			^^^
		"#
		);

		assert_parse_span!(
			EmptyAtomSet::ATOMS,
			CaseD,
			r#"
			45px foo 123 'bar' 'baz'
			^^^^^^^^^^^^^^^^^^
		"#
		);

		assert_parse!(
			EmptyAtomSet::ATOMS,
			CaseD,
			"foo 123 40px bar 'bar'",
			Optionals5(Some(_), Some(_), Some(_), Some(_), Some(_))
		);
	}
}
