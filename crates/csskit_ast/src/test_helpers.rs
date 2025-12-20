/// Assert CSS selector query matches expected node count.
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
		let source = $source;
		let lexer = Lexer::new(&CssAtomSet::ATOMS, source);
		let mut parser = Parser::new(&bump, source, lexer);
		let parsed = parser.parse::<$parse_type>().expect("failed to parse CSS");

		let selectors = $crate::selector::QuerySelectorList::parse($selector).expect("failed to parse selector");
		let matcher = $crate::selector::SelectorMatcher::new(&selectors, source);
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
