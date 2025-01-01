#[macro_export]
macro_rules! assert_snap_tokens {
	($source_path: literal) => {
		use css_lexer::{Kind, Lexer};
		use std::fs::read_to_string;

		let source_text = read_to_string($source_path).unwrap();
		let mut lexer = Lexer::new(&source_text);
		let mut tokens = vec![];
		loop {
			let cursor = lexer.advance();
			if cursor.kind() == Kind::Eof {
				break;
			}
			if !matches!(cursor.kind(), Kind::Whitespace | Kind::Comment) {
				tokens.push(cursor);
			}
		}
		insta::assert_json_snapshot!(tokens);
	};
}
