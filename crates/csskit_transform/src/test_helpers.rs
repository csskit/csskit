#[cfg(test)]
macro_rules! assert_transform {
	($features: ident :: $transform: ident, $atoms: ident, $node: ident, $str: literal, $expected: literal) => {{
		use bumpalo::Bump;
		use css_lexer::{Lexer, QuoteStyle};
		use css_parse::{CursorOverlaySink, CursorPrettyWriteSink, Parser, ToCursors};

		let source_text = $str;

		let allocator = Bump::default();
		let lexer = Lexer::new(&$atoms::ATOMS, source_text);
		let mut parser = Parser::new(&allocator, source_text, lexer);
		let mut result = parser.parse_entirely::<$node>();
		assert!(result.errors.is_empty(), "({:?}) saw error {:?}", source_text, result.errors[0]);

		let expected = $expected;
		let elexer = Lexer::new(&$atoms::ATOMS, expected);
		let mut eparser = Parser::new(&allocator, expected, elexer);
		let eresult = eparser.parse_entirely::<$node>();
		assert!(eresult.errors.is_empty(), "({:?}) saw error {:?}", expected, result.errors[0]);

		let mut transformer: $crate::Transformer<_, $node, $crate::$features> =
			$crate::Transformer::new_in(&allocator, $crate::$features::$transform, &$atoms::ATOMS, source_text);
		let mut actual = String::new();
		if let Some(ref mut output) = result.output {
			transformer.transform(output);
			let overlays = transformer.overlays();
			let mut overlay_stream = CursorOverlaySink::new(
				source_text,
				&*overlays,
				CursorPrettyWriteSink::new(source_text, &mut actual, None, QuoteStyle::Double),
			);
			result.output.to_cursors(&mut overlay_stream);

			assert!(transformer.has_changed(), "Transformer did not transform {}", source_text);
		} else {
			panic!("Transformer could not transform result {:?}", result.output);
		}

		let lexer = Lexer::new(&$atoms::ATOMS, &actual);
		let mut parser = Parser::new(&allocator, &actual, lexer);
		let result = parser.parse_entirely::<$node>();
		assert!(result.errors.is_empty(), "Minified code '({:?})' saw error {:?}", source_text, result.errors[0]);

		let mut expected_pretty = String::new();
		let mut expected_sink = CursorPrettyWriteSink::new(&expected, &mut expected_pretty, None, QuoteStyle::Double);
		eresult.output.to_cursors(&mut expected_sink);

		assert!(
			actual == expected_pretty,
			"\n\nDid not match expected format:\n\n```pre-transformed\n{}```\n```transformed\n{}```\n```expected\n{}\n```",
			source_text,
			actual,
			expected_pretty
		);
	}};
}

#[cfg(test)]
pub(crate) use assert_transform;

#[cfg(test)]
macro_rules! assert_no_transform {
	($features: ident :: $transform: ident, $atoms: ident, $node: ident, $str: literal) => {{
		use bumpalo::Bump;
		use css_lexer::{Lexer, QuoteStyle};
		use css_parse::{CursorOverlaySink, CursorPrettyWriteSink, Parser, ToCursors};

		let source_text = $str;

		let allocator = Bump::default();
		let lexer = Lexer::new(&$atoms::ATOMS, source_text);
		let mut parser = Parser::new(&allocator, source_text, lexer);
		let mut result = parser.parse_entirely::<$node>();
		assert!(result.errors.is_empty(), "({:?}) saw error {:?}", source_text, result.errors[0]);

		let mut transformer: $crate::Transformer<_, $node, $crate::$features> =
			$crate::Transformer::new_in(&allocator, $crate::$features::$transform, &$atoms::ATOMS, source_text);
		let mut actual = String::new();
		if let Some(ref mut output) = result.output {
			transformer.transform(output);
			let overlays = transformer.overlays();
			let mut overlay_stream = CursorOverlaySink::new(
				source_text,
				&*overlays,
				CursorPrettyWriteSink::new(source_text, &mut actual, None, QuoteStyle::Double),
			);
			result.output.to_cursors(&mut overlay_stream);

			assert!(!transformer.has_changed(), "Transformer claims to have changed! {} \"{}\"", source_text, actual);
		} else {
			panic!("Transformer could not transform result {:?}", result.output);
		}
	}};
}

#[cfg(test)]
pub(crate) use assert_no_transform;
