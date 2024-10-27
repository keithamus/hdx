use bumpalo::Bump;
use hdx_atom::atom;
use hdx_lexer::{Include, Kind, Lexer, Token};

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Lexer>(), 80);
	assert_eq!(::std::mem::size_of::<Token>(), 8);
	assert_eq!(::std::mem::size_of::<Kind>(), 1);
}

#[test]
fn empty() {
	let allocator = Bump::default();
	let mut lex = Lexer::new(&allocator, "", Include::all());
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 0);
}

#[test]
fn tokenizes_tilde_as_ddelim() {
	let allocator = Bump::default();
	let mut lex = Lexer::new(&allocator, "~", Include::all());
	assert_eq!(lex.pos(), 0);
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Delim);
		assert_eq!(tok.char(), Some('~'));
	}
	assert_eq!(lex.pos(), 1);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 1);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 1);
}

#[test]
fn tokenizes_newlines_as_whitespace() {
	let allocator = Bump::default();
	let mut lex = Lexer::new(&allocator, "\r\n", Include::all());
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance().kind(), Kind::Whitespace);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 2);
}

#[test]
fn tokenizes_multiple_newlines_as_whitespace() {
	let allocator = Bump::default();
	let mut lex = Lexer::new(&allocator, "\r\n", Include::all());
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance().kind(), Kind::Whitespace);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 2);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 2);
}

#[test]
fn tokenizes_multiple_whitespace_as_whitespace() {
	let allocator = Bump::default();
	let mut lex = Lexer::new(&allocator, "\t \t \t", Include::none());
	assert_eq!(lex.pos(), 0);
	assert_eq!(lex.advance().kind(), Kind::Whitespace);
	assert_eq!(lex.pos(), 5);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 5);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 5);
}

#[test]
fn tokenizes_trivial_css_file() {
	let allocator = Bump::default();
	let mut lex = Lexer::new(&allocator, "body { color: black }/* fin */", Include::all());
	assert_eq!(lex.pos(), 0);
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Ident);
		assert_eq!(lex.parse_atom(tok), atom!("body"));
	}
	assert_eq!(lex.pos(), 4);
	assert_eq!(lex.advance().kind(), Kind::Whitespace);
	assert_eq!(lex.pos(), 5);
	assert_eq!(lex.advance().kind(), Kind::LeftCurly);
	assert_eq!(lex.pos(), 6);
	assert_eq!(lex.advance().kind(), Kind::Whitespace);
	assert_eq!(lex.pos(), 7);
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Ident);
		assert_eq!(lex.parse_atom(tok), atom!("color"));
	}
	assert_eq!(lex.pos(), 12);
	assert_eq!(lex.advance().kind(), Kind::Colon);
	assert_eq!(lex.pos(), 13);
	assert_eq!(lex.advance().kind(), Kind::Whitespace);
	assert_eq!(lex.pos(), 14);
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Ident);
		assert_eq!(lex.parse_atom(tok), atom!("black"));
	}
	assert_eq!(lex.pos(), 19);
	assert_eq!(lex.advance().kind(), Kind::Whitespace);
	assert_eq!(lex.pos(), 20);
	assert_eq!(lex.advance().kind(), Kind::RightCurly);
	assert_eq!(lex.pos(), 21);
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Comment);
		assert_eq!(lex.parse_str(tok), " fin ");
	}
	assert_eq!(lex.pos(), 30);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 30);
}

#[test]
fn skips_whitespace_and_comments_with_next() {
	let allocator = Bump::default();
	let mut lex = Lexer::new(&allocator, "body { color: black }/* fin */", Include::none());
	assert_eq!(lex.pos(), 0);
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Ident);
		assert_eq!(lex.parse_atom(tok), atom!("body"));
	}
	assert_eq!(lex.pos(), 4);
	assert_eq!(lex.advance().kind(), Kind::LeftCurly);
	assert_eq!(lex.pos(), 6);
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Ident);
		assert_eq!(lex.parse_atom(tok), atom!("color"));
	}
	assert_eq!(lex.pos(), 12);
	assert_eq!(lex.advance().kind(), Kind::Colon);
	assert_eq!(lex.pos(), 13);
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Ident);
		assert_eq!(lex.parse_atom(tok), atom!("black"));
	}
	assert_eq!(lex.pos(), 19);
	assert_eq!(lex.advance().kind(), Kind::RightCurly);
	assert_eq!(lex.pos(), 21);
	assert_eq!(lex.advance().kind(), Kind::Eof);
	assert_eq!(lex.pos(), 30);
}

#[test]
fn tokenizes_wtf() {
	let allocator = Bump::default();
	let mut lex = Lexer::new(&allocator, "\\75 rl(a)\n", Include::none());
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Url);
		assert_eq!(tok.len(), 9);
		assert_eq!(lex.parse_raw_str(tok), "\\75 rl(a)");
		assert_eq!(lex.parse_str(tok), "a");
	}
}

#[test]
fn returns_correct_str_inner_value() {
	let allocator = Bump::default();
	let mut lex = Lexer::new(&allocator, "@foo #foo foo( url(foo) url(  foo) 'foo'", Include::none());
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::AtKeyword);
		assert_eq!(lex.parse_raw_str(tok), "@foo");
		assert_eq!(lex.parse_str(tok), "foo");
	}
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Hash);
		assert_eq!(lex.parse_raw_str(tok), "#foo");
		assert_eq!(lex.parse_str(tok), "foo");
	}
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Function);
		assert_eq!(lex.parse_raw_str(tok), "foo(");
		assert_eq!(lex.parse_str(tok), "foo");
	}
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Url);
		assert_eq!(lex.parse_raw_str(tok), "url(foo)");
		assert_eq!(lex.parse_str(tok), "foo");
	}
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Url);
		assert_eq!(lex.parse_raw_str(tok), "url(  foo)");
		assert_eq!(lex.parse_str(tok), "foo");
	}
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::String);
		assert_eq!(lex.parse_raw_str(tok), "'foo'");
		assert_eq!(lex.parse_str(tok), "foo");
	}
}

#[test]
fn returns_correct_str_escaped_value() {
	let allocator = Bump::default();
	let mut lex =
		Lexer::new(&allocator, "@f\\6fo #f\\6fo f\\6fo( url( f\\6fo) u\\72l( f\\6fo) 'f\\6fo'", Include::none());
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::AtKeyword);
		assert_eq!(lex.parse_raw_str(tok), "@f\\6fo");
		assert_eq!(lex.parse_str(tok), "foo");
	}
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Hash);
		assert_eq!(lex.parse_raw_str(tok), "#f\\6fo");
		assert_eq!(lex.parse_str(tok), "foo");
	}
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Function);
		assert_eq!(lex.parse_raw_str(tok), "f\\6fo(");
		assert_eq!(lex.parse_str(tok), "foo");
	}
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Url);
		assert_eq!(lex.parse_raw_str(tok), "url( f\\6fo)");
		assert_eq!(lex.parse_str(tok), "foo");
	}
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::Url);
		assert_eq!(lex.parse_raw_str(tok), "u\\72l( f\\6fo)");
		assert_eq!(lex.parse_str(tok), "foo");
	}
	{
		let tok = lex.advance();
		assert_eq!(tok.kind(), Kind::String);
		assert_eq!(lex.parse_raw_str(tok), "'f\\6fo'");
		assert_eq!(lex.parse_str(tok), "foo");
	}
}
