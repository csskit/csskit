/// (Requires feature "testing") Given a Node, and a string, this will expand to code that sets up a parser, and parses the given string against the
/// given node. If the parse failed this macro will [panic] with a readable failure. It then writes the result out using
/// [crate::CursorWriteSink], writing the parsed Node back out to a string. If resulting string from the given string, then the
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
		let mut cursors = $crate::CursorWriteSink::new(&source_text, &mut actual);
		{
			use $crate::ToCursors;
			result.to_cursors(&mut cursors);
		}
		if expected != actual {
			panic!("\n\nParse failed: did not match expected format:\n\n   parser input: {:?}\n  parser output: {:?}\n       expected: {:?}\n", source_text, actual, expected);
		}
		#[allow(clippy::redundant_pattern_matching)] // Avoid .clone().unwrap()
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
				let mut cursors = $crate::CursorWriteSink::new(&source_text, &mut actual);
				use $crate::ToCursors;
				result.to_cursors(&mut cursors);
				panic!("\n\nExpected errors but it passed without error.\n\n   parser input: {:?}\n  parser output: {:?}\n       expected: (Error)", source_text, actual);
			}
		}
	};
}
#[cfg(test)]
pub(crate) use assert_parse_error;

/// (Requires feature "testing") Given a Node, and a multiline string, this will expand to code that sets up a parser,
/// and parses the first line of the given string with the parser. It will then create a second string based on the span
/// data and append it to the first line of the string, showing what was parsed and where the span rests.
///
/// This uses `parse`, as it will be often used in situations where there may be trailing unparsed tokens.
///
/// ```
/// use css_parse::*;
/// assert_parse_span!(T![Ident], r#"
///     an_ident another_ident
///     ^^^^^^^^
/// "#);
/// ```
#[macro_export]
macro_rules! assert_parse_span {
	($ty: ty, $str: literal) => {
		let expected = $str;
		let (indent, source_text) = expected
			.lines()
			.find(|line| !line.trim().is_empty())
			.map(|line| {
				let content = line.trim_start();
				let ws_len = line.len() - content.len();
				(&line[..ws_len], content)
			})
			.unwrap_or(("", ""));
		let bump = ::bumpalo::Bump::default();
		let mut parser = $crate::Parser::new(&bump, source_text);
		let result = parser.parse::<$ty>();
		match result {
			Ok(result) => {
				use $crate::ToSpan;
				let span = result.to_span();
				let actual = format!("\n{}{}\n{}{}\n", indent, source_text, indent, "^".repeat(span.len() as usize));
				if expected.trim() != actual.trim() {
					panic!(
						"\n\nParse on {}:{} succeeded but span differs:\n\n  expected: {}\n  actual: {}\n",
						file!(),
						line!(),
						expected,
						actual,
					);
				}
			}
			Err(err) => {
				panic!("\n\nParse on {}:{} failed. ({:?}) saw error {:?}", file!(), line!(), source_text, err);
			}
		}
	};
}
#[cfg(test)]
pub(crate) use assert_parse_span;
