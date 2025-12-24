/// Assert CSS selector query matches expected node count and optionally verify node IDs.
/// Matching uses metadata-based early filtering to skip selectors that can't match.
///
/// Usage:
/// - `assert_query!(css, selector, count)` - assert match count
/// - `assert_query!(css, selector, count, [NodeId::A, NodeId::B])` - assert count and node IDs
#[macro_export]
macro_rules! assert_query {
	($source:expr, $selector:expr, $expected:expr) => {
		$crate::assert_query!($source, $selector => css_ast::StyleSheet, $expected, [])
	};

	($source:expr, $selector:expr, $expected:expr, [$($node_id:expr),* $(,)?]) => {
		$crate::assert_query!($source, $selector => css_ast::StyleSheet, $expected, [$($node_id),*])
	};

	($source:expr, $selector:expr => $parse_type:ty, $expected:expr) => {
		$crate::assert_query!($source, $selector => $parse_type, $expected, [])
	};

	($source:expr, $selector:expr => $parse_type:ty, $expected:expr, [$($node_id:expr),* $(,)?]) => {{
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
		let matches: Vec<_> = matcher.run(&parsed).collect();

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

		// Verify node IDs if provided (order is deterministic - exit order from tree traversal)
		let expected_ids: Vec<css_ast::visit::NodeId> = vec![$($node_id),*];
		if !expected_ids.is_empty() {
			let actual_ids: Vec<_> = matches.iter().map(|m| m.node_id).collect();
			assert_eq!(
				actual_ids,
				expected_ids,
				"\n\nQuery {:?} on {:?} returned wrong node IDs\nExpected: {:?}\nActual: {:?}",
				$selector,
				source,
				expected_ids,
				actual_ids
			);
		}

		matches
	}};
}
