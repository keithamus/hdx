use hdx_atom::{atom, Atom};
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
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 0);
}

#[test]
fn tokenizes_tilde_as_ddelim() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "~");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Delim('~'));
	assert_eq!(lex.pos(), 1);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 1);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 1);
}

#[test]
fn tokenizes_newlines_as_whitespace() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "\r\n");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Whitespace);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 2);
}

#[test]
fn tokenizes_multiple_newlines_as_whitespace() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "\r\n");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Whitespace);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 2);
}

#[test]
fn tokenizes_multiple_whitespace_as_whitespace() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "\t \t \t");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Whitespace);
	assert_eq!(lex.pos(), 5);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 5);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 5);
}

#[test]
fn tokenizes_trivial_css_file() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "body { color: black }/* fin */");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Ident(atom!("body")));
	assert_eq!(lex.pos(), 4);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Whitespace);
	assert_eq!(lex.pos(), 5);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::LeftCurly);
	assert_eq!(lex.pos(), 6);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Whitespace);
	assert_eq!(lex.pos(), 7);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Ident(atom!("color")));
	assert_eq!(lex.pos(), 12);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Colon);
	assert_eq!(lex.pos(), 13);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Whitespace);
	assert_eq!(lex.pos(), 14);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Ident(atom!("black")));
	assert_eq!(lex.pos(), 19);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Whitespace);
	assert_eq!(lex.pos(), 20);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::RightCurly);
	assert_eq!(lex.pos(), 21);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Comment(Atom::from(" fin ")));
	assert_eq!(lex.pos(), 30);
	assert_eq!(lex.advance_including_whitespace_and_comments(), Token::Eof);
	assert_eq!(lex.pos(), 30);
}

#[test]
fn skips_whitespace_and_comments_with_next() {
	let allocator = Allocator::default();
	let mut lex = Lexer::new(&allocator, "body { color: black }/* fin */");
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance(), Token::Ident(atom!("body")));
	assert_eq!(lex.pos(), 4);
	assert_eq!(lex.advance(), Token::LeftCurly);
	assert_eq!(lex.pos(), 6);
	assert_eq!(lex.advance(), Token::Ident(atom!("color")));
	assert_eq!(lex.pos(), 12);
	assert_eq!(lex.advance(), Token::Colon);
	assert_eq!(lex.pos(), 13);
	assert_eq!(lex.advance(), Token::Ident(atom!("black")));
	assert_eq!(lex.pos(), 19);
	assert_eq!(lex.advance(), Token::RightCurly);
	assert_eq!(lex.pos(), 21);
	assert_eq!(lex.advance(), Token::Eof);
	assert_eq!(lex.pos(), 30);
}
