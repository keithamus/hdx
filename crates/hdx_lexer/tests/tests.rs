use hdx_lexer::{Kind, Lexer, Span, Token, TokenValue};
use oxc_allocator::Allocator;

fn consume_lex<'a>(allocator: &'a Allocator, source: &'a str) -> (Lexer<'a>, Vec<Token>) {
	let mut lex = Lexer::new(allocator, source);
	let mut tokens: Vec<Token> = vec![];
	loop {
		let token = lex.next_token();
		if token.kind == Kind::Eof {
			break;
		}
		tokens.push(token);
	}
	(lex, tokens)
}

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Token>(), 32);
	assert_eq!(::std::mem::size_of::<TokenValue>(), 16);
}

#[test]
fn smoke_test() {
	let allocator = Allocator::default();
	let (_lex, tokens) = consume_lex(&allocator, "");
	assert_eq!(tokens, vec![]);
}

#[test]
fn tokenizes_tilde_as_ddelim() {
	let allocator = Allocator::default();
	let (_lex, tokens) = consume_lex(&allocator, "~");
	assert_eq!(
		tokens,
		vec![Token {
			kind: Kind::Delim,
			span: Span::new(0, 1),
			escaped: false,
			value: TokenValue::Char('~'),
		}]
	);
}

#[test]
fn tokenizes_newlines_as_whitespace() {
	let allocator = Allocator::default();
	let (_lex, tokens) = consume_lex(&allocator, "\r\n");
	assert_eq!(
		tokens,
		vec![Token {
			kind: Kind::Whitespace,
			span: Span::new(0, 2),
			escaped: false,
			value: TokenValue::None,
		}]
	);
}

#[test]
fn tokenizes_multiple_newlines_as_whitespace() {
	let allocator = Allocator::default();
	let (_lex, tokens) = consume_lex(&allocator, "\n\r");
	assert_eq!(
		tokens,
		vec![Token {
			kind: Kind::Whitespace,
			span: Span::new(0, 2),
			escaped: false,
			value: TokenValue::None,
		}]
	);
}

#[test]
fn tokenizes_multiple_whitespace_as_whitespace() {
	let allocator = Allocator::default();
	let (_lex, tokens) = consume_lex(&allocator, "\t \t \t");
	assert_eq!(
		tokens,
		vec![Token {
			kind: Kind::Whitespace,
			span: Span::new(0, 5),
			escaped: false,
			value: TokenValue::None,
		}]
	);
}
