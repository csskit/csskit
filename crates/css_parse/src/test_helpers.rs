/// (Requires feature "testing") Given a Node, and a string, this will expand to code that sets up a parser, and parses the given string against the
/// given node. If the parse failed this macro will [panic] with a readable failure. It then writes the result out using
/// [crate::CursorWriteSink], writing the parsed Node back out to a string. If resulting string from the given string, then the
/// macro will [panic] with a readable failure.
///
/// ```
/// use css_parse::*;
/// assert_parse!(EmptyAtomSet::ATOMS, T![Ident], "foo");
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
/// assert_parse!(EmptyAtomSet::ATOMS, IdentOrNumber, "foo", IdentOrNumber::Ident(_));
/// assert_parse!(EmptyAtomSet::ATOMS, IdentOrNumber, "12", IdentOrNumber::Number(_));
/// ```
#[macro_export]
macro_rules! assert_parse {
	($atomset: path, $ty: ty, $str: literal, $($ast: pat)+) => {
		let source_text = $str;
		let bump = ::bumpalo::Bump::default();
		let lexer = css_lexer::Lexer::new(&$atomset, &source_text);
		let mut parser = $crate::Parser::new(&bump, &source_text, lexer);
		let result = parser.parse_entirely::<$ty>().with_trivia();
		if !result.errors.is_empty() {
			panic!("\n\nParse failed. ({:?}) saw error {:?}", source_text, result.errors[0]);
		}
		let mut actual = ::bumpalo::collections::String::new_in(&bump);
		{
			let mut write_sink = $crate::CursorWriteSink::new(&source_text, &mut actual);
			let mut ordered_sink = $crate::CursorOrderedSink::new(&bump, &mut write_sink);
			use $crate::ToCursors;
			result.to_cursors(&mut ordered_sink);
		}
		if source_text.trim() != actual.trim() {
			panic!("\n\nParse failed: did not match expected format:\n\n   parser input: {:?}\n  parser output: {:?}\n", source_text, actual);
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
	($atomset: path, $ty: ty, $str: literal) => {
		assert_parse!($atomset, $ty, $str, _);
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
/// assert_parse_error!(EmptyAtomSet::ATOMS, T![Ident], "1");
/// ```
#[macro_export]
macro_rules! assert_parse_error {
	($atomset: path, $ty: ty, $str: literal) => {
		let source_text = $str;
		let bump = ::bumpalo::Bump::default();
		let lexer = css_lexer::Lexer::new(&$atomset, source_text);
		let mut parser = $crate::Parser::new(&bump, source_text, lexer);
		let result = parser.parse::<$ty>();
		if parser.at_end() {
			if let Ok(result) = result {
				let mut actual = ::bumpalo::collections::String::new_in(&bump);
				{
					let mut write_sink = $crate::CursorWriteSink::new(&source_text, &mut actual);
					let mut ordered_sink = $crate::CursorOrderedSink::new(&bump, &mut write_sink);
					use $crate::ToCursors;
					result.to_cursors(&mut ordered_sink);
				}
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
/// assert_parse_span!(EmptyAtomSet::ATOMS, T![Ident], r#"
///     an_ident another_ident
///     ^^^^^^^^
/// "#);
/// ```
#[macro_export]
macro_rules! assert_parse_span {
	($atomset: path, $ty: ty, $str: literal) => {
		let expected = $str;
		let source_text = expected.lines().find(|line| !line.trim().is_empty()).unwrap_or("");
		let bump = ::bumpalo::Bump::default();
		let lexer = css_lexer::Lexer::new(&$atomset, source_text);
		let mut parser = $crate::Parser::new(&bump, source_text, lexer);
		let result = parser.parse::<$ty>();
		match result {
			Ok(result) => {
				use $crate::ToSpan;
				let span = result.to_span();
				let indent = &source_text[0..span.start().into()];
				if indent.trim().len() > 0 {
					panic!(
						"\n\nParse on {}:{} succeeded but has non-whitespace leading text: {}\n",
						file!(),
						line!(),
						indent
					);
				}
				let actual = format!("\n{}{}\n{}{}\n", indent, source_text, indent, "^".repeat(span.len() as usize));
				if expected.trim() != actual.trim() {
					panic!(
						"\n\nParse on {}:{} succeeded but span ({}) differs:\n\n  expected: {}\n  actual: {}\n",
						file!(),
						line!(),
						span,
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
