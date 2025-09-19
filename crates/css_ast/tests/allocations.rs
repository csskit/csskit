use bumpalo::Bump;
use css_ast::StyleSheet;
use css_parse::Parser;
#[cfg(feature = "_dhat-heap-testing")]
use dhat::{Alloc, HeapStats, Profiler, assert_eq};
use std::fs::read_to_string;

#[cfg(feature = "_dhat-heap-testing")]
#[global_allocator]
static ALLOC: Alloc = Alloc;

#[test]
fn allocation_test() {
	let simple_bump_size = 1984;
	let simple_bump = Bump::with_capacity(simple_bump_size);
	let simple_str = "body{color:blue}";
	let mut simple_parser = Parser::new(&simple_bump, simple_str);

	let escaped_bump_size = 16320;
	let escaped_bump = Bump::with_capacity(escaped_bump_size);
	let escaped_str = "bo\\d y{background-image:\\75\\52\\6c(a);width:1\\70\\78}";
	let mut escape_parser = Parser::new(&escaped_bump, escaped_str);

	let big_bump_size = 331_222_976;
	let big_bump = Bump::with_capacity(big_bump_size);
	let big_str = read_to_string("../../coverage/popular/tailwind.2.2.19.min.css").unwrap();
	let mut big_parser = Parser::new(&big_bump, &big_str);

	#[cfg(feature = "_dhat-heap-testing")]
	let _profiler = Profiler::builder()
		.testing()
		.file_name(format!("../../target/css_ast_allocations_test-{}.json", std::process::id()))
		.build();

	simple_parser.parse_entirely::<StyleSheet>();
	#[cfg(feature = "_dhat-heap-testing")]
	{
		let stats = HeapStats::get();
		assert_eq!(stats.total_blocks, 0);
		assert_eq!(stats.total_bytes, 0);
	}

	escape_parser.parse_entirely::<StyleSheet>();
	#[cfg(feature = "_dhat-heap-testing")]
	{
		let stats = HeapStats::get();
		assert_eq!(stats.total_blocks, 0);
		assert_eq!(stats.total_bytes, 0);
	}

	big_parser.parse_entirely::<StyleSheet>();
	#[cfg(feature = "_dhat-heap-testing")]
	{
		let stats = HeapStats::get();
		// These are due to allocations in `eq_ignore_ascii_case` debug_asserts!
		assert_eq!(stats.total_blocks, 61354);
		assert_eq!(stats.total_bytes, 246910);
	}

	// XXX: If these fail because the numbers go down, great! If they go up, investigate why.
	assert_eq!(simple_bump.allocated_bytes(), simple_bump_size);
	assert_eq!(escaped_bump.allocated_bytes(), escaped_bump_size);
	assert_eq!(big_bump.allocated_bytes(), big_bump_size);
}
