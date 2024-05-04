#[macro_export]
macro_rules! assert_snap_tokens {
	($source_path: literal) => {
		use bumpalo::Bump;
		use hdx_lexer::{Include, Lexer, Token};
		use std::fs::read_to_string;

		let allocator = Bump::default();
		let source_text = read_to_string($source_path).unwrap();
		let mut lexer = Lexer::new(&allocator, &source_text);
		let mut tokens = vec![];
		loop {
			let token = lexer.advance_with(Include::all());
			if token == Token::Eof {
				break;
			}
			tokens.push(token);
		}
		insta::assert_json_snapshot!(tokens);
	};
}
