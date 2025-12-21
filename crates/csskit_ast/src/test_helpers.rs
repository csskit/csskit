/// Assert CSS selector query matches expected node count.
/// Matching uses metadata-based early filtering to skip selectors that can't match.
#[macro_export]
macro_rules! assert_query {
	($source:expr, $selector:expr, $expected:expr) => {
		$crate::assert_query!($source, $selector => css_ast::StyleSheet, $expected)
	};

	($source:expr, $selector:expr => $parse_type:ty, $expected:expr) => {{
		use bumpalo::Bump;
		use css_ast::CssAtomSet;
		use css_lexer::Lexer;
		use css_parse::Parser;

		let bump = Bump::new();
		let selector_bump = Bump::new();
		let source = $source;
		let selector_str: &str = $selector;
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source);
		let mut parser = Parser::new(&bump, source, lexer);
		let parsed = parser.parse::<$parse_type>().expect("failed to parse CSS");

		let selector_lexer = Lexer::new(&$crate::CsskitAtomSet::ATOMS, selector_str);
		let mut selector_parser = Parser::new(&selector_bump, selector_str, selector_lexer);
		let selector_result = selector_parser.parse_entirely::<$crate::selector::QuerySelectorList>();
		let selectors = selector_result.output.expect("failed to parse selector");

		let matcher = $crate::selector::SelectorMatcher::new(&selectors, selector_str, source);
		let matches = matcher.run(&parsed);

		assert_eq!(
			matches.len(),
			$expected,
			"\n\nQuery {:?} on {:?} returned {} matches, expected {}\nMatches: {:?}",
			$selector,
			source,
			matches.len(),
			$expected,
			matches
		);
		matches
	}};
}
