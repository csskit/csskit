use css_lexer::{Kind, Lexer};
#[cfg(feature = "_dhat-heap-testing")]
use dhat::{Alloc, HeapStats, Profiler, assert_eq};
#[cfg(not(feature = "_dhat-heap-testing"))]
use std::assert_eq;
use std::fs::read_to_string;

#[cfg(feature = "_dhat-heap-testing")]
#[global_allocator]
static ALLOC: Alloc = Alloc;

#[test]
fn ensure_no_allocations_during_lex() {
	let str = "body{color:blue}";
	let mut lexer = Lexer::new(str);
	let mut token_count = 0;

	let str2 = "bo\\d y{\\75\\52\\6c(a);width:1\\70\\78}";
	let mut token_count2 = 0;
	let mut lexer2 = Lexer::new(str2);

	let big = read_to_string("../../coverage/popular/tailwind.2.2.19.min.css").unwrap();
	let mut lexer_big = Lexer::new(&big);

	#[cfg(feature = "_dhat-heap-testing")]
	let _profiler = Profiler::builder()
		.testing()
		.file_name(format!("../../target/css_lexer_allocations_test-{}.json", std::process::id()))
		.build();

	loop {
		let cursor = lexer.advance();
		token_count += 1;
		if cursor.kind() == Kind::Eof {
			break;
		}
	}
	assert_eq!(token_count, 7);

	loop {
		let cursor = lexer2.advance();
		token_count2 += 1;
		if cursor.kind() == Kind::Eof {
			break;
		}
	}
	assert_eq!(token_count2, 9);

	loop {
		let cursor = lexer_big.advance();
		if cursor.kind() == Kind::Eof {
			break;
		}
	}

	#[cfg(feature = "_dhat-heap-testing")]
	{
		let stats = HeapStats::get();
		assert_eq!(stats.total_blocks, 0);
		assert_eq!(stats.total_bytes, 0);
	}
}
