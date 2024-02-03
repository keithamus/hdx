use hdx_atom::atom;
use hdx_lexer::{Lexer, Token};
use oxc_allocator::Allocator;

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Token>(), 16);
}

#[test]
fn empty() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 0);
}

#[test]
fn tokenizes_tilde_as_ddelim() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "~");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.next_token(), Token::Delim('~'));
	assert_eq!(lex.pos(), 1);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 1);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 1);
}

#[test]
fn tokenizes_newlines_as_whitespace() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "\r\n");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.next_token(), Token::Whitespace);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 2);
}

#[test]
fn tokenizes_multiple_newlines_as_whitespace() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "\r\n");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.next_token(), Token::Whitespace);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 2);
}

#[test]
fn tokenizes_multiple_whitespace_as_whitespace() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "\t \t \t");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.next_token(), Token::Whitespace);
	assert_eq!(lex.pos(), 5);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 5);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 5);
}

#[test]
fn tokenizes_trivial_css_file() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "body { color: black }");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.next_token(), Token::Ident(atom!("body")));
	assert_eq!(lex.pos(), 4);
	assert_eq!(lex.next_token(), Token::Whitespace);
	assert_eq!(lex.pos(), 5);
	assert_eq!(lex.next_token(), Token::LeftCurly);
	assert_eq!(lex.pos(), 6);
	assert_eq!(lex.next_token(), Token::Whitespace);
	assert_eq!(lex.pos(), 7);
	assert_eq!(lex.next_token(), Token::Ident(atom!("color")));
	assert_eq!(lex.pos(), 12);
	assert_eq!(lex.next_token(), Token::Colon);
	assert_eq!(lex.pos(), 13);
	assert_eq!(lex.next_token(), Token::Whitespace);
	assert_eq!(lex.pos(), 14);
	assert_eq!(lex.next_token(), Token::Ident(atom!("black")));
	assert_eq!(lex.pos(), 19);
	assert_eq!(lex.next_token(), Token::Whitespace);
	assert_eq!(lex.pos(), 20);
	assert_eq!(lex.next_token(), Token::RightCurly);
	assert_eq!(lex.pos(), 21);
	assert_eq!(lex.next_token(), Token::Eof);
	assert_eq!(lex.pos(), 21);
}
