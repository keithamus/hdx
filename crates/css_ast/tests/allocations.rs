use css_ast::{CssAtomSet, StyleSheet};
use css_lexer::Lexer;
use css_parse::{Arena, Parser};
#[cfg(feature = "_dhat-heap-testing")]
use dhat::{Alloc, HeapStats, Profiler, assert_eq};
use std::fs::read_to_string;

#[cfg(feature = "_dhat-heap-testing")]
#[global_allocator]
static ALLOC: Alloc = Alloc;

#[test]
fn allocation_test() {
	let simple_alloc_size = 1984;
	let simple_alloc = Arena::with_capacity(simple_alloc_size);
	let simple_str = "body{color:blue}";
	let simple_lexer = Lexer::new(&CssAtomSet::ATOMS, simple_str);
	let mut simple_parser = Parser::new(&simple_alloc, simple_str, simple_lexer);

	let escaped_alloc_size = 16320;
	let escaped_alloc = Arena::with_capacity(escaped_alloc_size);
	let escaped_str = "bo\\d y{background-image:\\75\\52\\6c(a);width:1\\70\\78}";
	let escaped_lexer = Lexer::new(&CssAtomSet::ATOMS, escaped_str);
	let mut escape_parser = Parser::new(&escaped_alloc, escaped_str, escaped_lexer);

	let big_alloc_size = 331_222_976;
	let big_alloc = Arena::with_capacity(big_alloc_size);
	let big_str = read_to_string("../../coverage/popular/tailwind.2.2.19.min.css").unwrap();
	let big_lexer = Lexer::new(&CssAtomSet::ATOMS, &big_str);
	let mut big_parser = Parser::new(&big_alloc, &big_str, big_lexer);

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
		assert_eq!(stats.total_blocks, 0);
		assert_eq!(stats.total_bytes, 0);
	}

	// XXX: If these fail because the numbers go down, great! If they go up, investigate why.
	assert_eq!(simple_alloc.used_bytes(), 1280);
	assert_eq!(escaped_alloc.used_bytes(), 5312);
	assert_eq!(big_alloc.used_bytes(), 120_760_512);
}
