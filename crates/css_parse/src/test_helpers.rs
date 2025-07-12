/// (Requires feature "testing") Given a Node, and a string, this will expand to code that sets up a parser, and parses the given string against the
/// given node. If the parse failed this macro will [panic] with a readable failure. It then writes the result out using
/// [crate::CursorFmtSink], writing the parsed Node back out to a string. If resulting string from the given string, then the
/// macro will [panic] with a readable failure.
///
/// In rare cases it might be necessary to ensure the resulting string _differs_ from the input, for example if a
/// grammar is serialized in a specific order but allows parsing in any order (many style values do this). In these
/// cases, a second string can be provided which will be asserted gainst the output instead.
///
/// ```
/// use css_parse::*;
/// assert_parse!(T![Ident], "foo");
/// // Equivalent to:
/// assert_parse!(T![Ident], "foo", "foo");
/// ```
///
/// For more complex types (for example enum variants), you might want to assert that the given AST
/// node matches an expected pattern (for example, one enum variant was chosen over another). In
/// these cases, passing the match pattern as the third (or fourth) argument will assert that the
/// parsed output struct matches the given pattern:
///
/// ```
/// use css_parse::*;
/// use csskit_derives::*;
/// #[derive(Parse, ToCursors, Debug)]
/// enum IdentOrNumber {
///     Ident(T![Ident]),
///     Number(T![Number]),
/// }
/// assert_parse!(IdentOrNumber, "foo", IdentOrNumber::Ident(_));
/// assert_parse!(IdentOrNumber, "12", IdentOrNumber::Number(_));
/// ```
#[macro_export]
macro_rules! assert_parse {
	($ty: ty, $str: literal, $str2: literal, $($ast: pat)+) => {
		let source_text = $str;
		let expected = $str2;
		let bump = ::bumpalo::Bump::default();
		let mut parser = $crate::Parser::new(&bump, &source_text);
		let result = parser.parse_entirely::<$ty>();
		if !result.errors.is_empty() {
			panic!("\n\nParse failed. ({:?}) saw error {:?}", source_text, result.errors[0]);
		}
		let mut actual = ::bumpalo::collections::String::new_in(&bump);
		let mut cursors = $crate::CursorFmtSink::new(&source_text, &mut actual);
		{
			use $crate::ToCursors;
			result.to_cursors(&mut cursors);
		}
		if expected != actual {
			panic!("\n\nParse failed: did not match expected format:\n\n   parser input: {:?}\n  parser output: {:?}\n       expected: {:?}\n", source_text, actual, expected);
		}
		if !matches!(result.output, Some($($ast)|+)) {
			panic!(
        "\n\nParse succeeded but struct did not match given match pattern:\n\n           input: {:?}\n  match pattern: {}\n  parsed struct: {:#?}\n",
        source_text,
        stringify!($($ast)|+),
        result.output.unwrap(),
      );
    }
	};
	($ty: ty, $str: literal) => {
		assert_parse!($ty, $str, $str, _);
	};
	($ty: ty, $str: literal, $str2: literal) => {
		assert_parse!($ty, $str, $str2, _);
	};
	($ty: ty, $str: literal, $($ast: pat)+) => {
		assert_parse!($ty, $str, $str, $($ast)+);
	};
}
#[cfg(test)]
pub(crate) use assert_parse;

/// (Requires feature "testing") Given a Node, and a string, this will expand to code that sets up a parser, and parses the given string against the
/// given node. If the parse succeeded this macro will [panic] with a readable failure.
///
/// In rare cases it might be necessary to ensure the resulting string _differs_ from the input, for example if a
/// grammar is serialized in a specific order but allows parsing in any order (many style values do this). In these
/// cases, a second string can be provided which will be asserted gainst the output instead.
///
/// ```
/// use css_parse::*;
/// assert_parse_error!(T![Ident], "1");
/// ```
#[macro_export]
macro_rules! assert_parse_error {
	($ty: ty, $str: literal) => {
		let source_text = $str;
		let bump = ::bumpalo::Bump::default();
		let mut parser = $crate::Parser::new(&bump, source_text);
		let result = parser.parse::<$ty>();
		if parser.at_end() {
			if let Ok(result) = result {
				let mut actual = ::bumpalo::collections::String::new_in(&bump);
				let mut cursors = $crate::CursorFmtSink::new(&source_text, &mut actual);
				use $crate::ToCursors;
				result.to_cursors(&mut cursors);
				panic!("\n\nExpected errors but it passed without error.\n\n   parser input: {:?}\n  parser output: {:?}\n       expected: (Error)", source_text, actual);
			}
		}
	};
}
#[cfg(test)]
pub(crate) use assert_parse_error;
