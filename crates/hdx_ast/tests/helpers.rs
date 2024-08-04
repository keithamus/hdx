#[macro_export]
macro_rules! assert_snap_ast {
	($source_path: literal) => {{
		use bumpalo::Bump;
		use hdx_ast::css::StyleSheet;
		use hdx_parser::{Features, Parser};
		use std::fs::read_to_string;

		let allocator = Bump::default();
		let source_text = read_to_string($source_path).unwrap();
		let mut parser = Parser::new(&allocator, &source_text, Features::default());
		let result = parser.parse_entirely::<StyleSheet>();
		if !result.errors.is_empty() {
			panic!("\n\nParse {:?} failed. Saw error {:?}", $source_path, result.errors[0]);
		}
		#[cfg(feature = "serde")]
		insta::assert_json_snapshot!(result.output.unwrap())
	}};
}
