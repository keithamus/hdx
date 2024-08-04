#[macro_export]
macro_rules! assert_snap_tokens {
	($source_path: literal) => {
		use hdx_lexer::{Include, Kind, Lexer};
		use std::fs::read_to_string;

		let source_text = read_to_string($source_path).unwrap();
		let mut lexer = Lexer::new(&source_text, Include::none());
		let mut tokens = vec![];
		loop {
			let token = lexer.advance();
			if token.kind() == Kind::Eof {
				break;
			}
			tokens.push(token);
		}
		insta::assert_json_snapshot!(tokens);
	};
}
