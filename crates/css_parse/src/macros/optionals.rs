use crate::{CursorSink, Parse, Parser, Peek, Result as ParserResult, Span, ToCursors, ToSpan};

macro_rules! impl_optionals {
	($($name:ident, ($($T:ident),+))+) => {
		$(
			#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
			#[cfg_attr(feature = "serde", derive(serde::Serialize), serde())]
			pub struct $name<$($T),+>($(pub Option<$T>),+);

			impl<'a, $($T),+> Parse<'a> for $name<$($T),+>
			where
				$($T: Parse<'a> + Peek<'a>,)+
			{
				#[allow(non_snake_case)]
				fn parse(p: &mut Parser<'a>) -> ParserResult<Self> {
					let ($($T),+) = parse_optionals!(p, $($T:$T),+);
					Ok(Self($($T),+))
				}
			}

			impl<'a, $($T),+> ToCursors for $name<$($T),+>
			where
				$($T: ToCursors,)+
			{
				#[allow(non_snake_case)]
				fn to_cursors(&self, s: &mut impl CursorSink) {
					let $name($($T),+) = self;
					$($T.to_cursors(s);)+
			 }
			}

			impl<$($T),+> ToSpan for  $name<$($T),+>
			where
				$($T: ToSpan,)+
			{
				#[allow(non_snake_case)]
				fn to_span(&self) -> Span {
					let $name($($T),+) = self;
					Span::DUMMY $(+$T.to_span())+
				}
			}

			impl<$($T),+> From<$name<$($T),+>> for ($(Option<$T>),+)
			{
				#[allow(non_snake_case)]
				fn from(value: $name<$($T),+>) -> Self {
					let $name($($T),+) = value;
					($($T),+)
				}
			}

			impl<$($T),+> From<($(Option<$T>),+)> for $name<$($T),+>
			{
				#[allow(non_snake_case)]
				fn from(value: ($(Option<$T>),+)) -> Self {
					let ($($T),+) = value;
					Self($($T),+)
				}
			}
		)+
	};
}

#[macro_export]
macro_rules! parse_optionals {
	($p: ident, $($name:ident: $T:ty),+) => {
		{
			#[allow(non_snake_case)]
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
				Err($crate::diagnostics::Unexpected($p.next()))?
			}

			(($($name),+))
		 }
	};
}

/// A helper type for parsing optional CSS grammar patterns where items can appear in any order
/// but at most once each (the `||` combinator in CSS grammar).
///
/// # Example
/// ```ignore
/// // For CSS grammar: [ foo | <number> ]
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
	Optionals2, (A, B)
	Optionals3, (A, B, C)
	Optionals4, (A, B, C, D)
	Optionals5, (A, B, C, D, E)
}

#[cfg(test)]
mod tests {
	use super::*;
	use crate::test_helpers::*;
	use crate::token_macros::*;

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
		assert_parse!(CaseA, "123 foo", Optionals2(Some(_), Some(_)));
		assert_parse!(CaseA, "foo 123", "123 foo", Optionals2(Some(_), Some(_)));
		assert_parse!(CaseA, "123", Optionals2(Some(_), None));
		assert_parse!(CaseA, "foo", Optionals2(None, Some(_)));

		assert_parse!(CaseB, "123 foo 'bar'", "123 foo'bar'", Optionals3(Some(_), Some(_), Some(_)));
		assert_parse!(CaseB, "foo 'bar' 123", "123 foo'bar'", Optionals3(Some(_), Some(_), Some(_)));
		assert_parse!(CaseB, "123", Optionals3(Some(_), None, None));
		assert_parse!(CaseB, "'foo'", Optionals3(None, None, Some(_)));

		assert_parse!(CaseC, "foo 123 bar 'bar'", "123 foo'bar'bar", Optionals4(Some(_), Some(_), Some(_), Some(_)));
	}

	#[test]
	fn test_spans() {
		assert_parse_span!(
			CaseA,
			r#"
			foo 123 bar
			^^^^^^^
		"#
		);

		assert_parse_span!(
			CaseA,
			r#"
			123 foo bar
			^^^^^^^
		"#
		);

		assert_parse_span!(
			CaseA,
			r#"
			123 'foo'
			^^^
		"#
		);

		assert_parse_span!(
			CaseD,
			r#"
			45px foo 123 'bar' 'baz'
			^^^^^^^^^^^^^^^^^^
		"#
		);

		assert_parse!(
			CaseD,
			"foo 123 40px bar 'bar'",
			"123 foo'bar'bar 40px",
			Optionals5(Some(_), Some(_), Some(_), Some(_), Some(_))
		);
	}
}
