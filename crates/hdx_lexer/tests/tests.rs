use bumpalo::Bump;
use hdx_atom::{atom, Atom};
use hdx_lexer::{Include, Kind, Lexer, QuoteStyle, Token};

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Lexer>(), 80);
	assert_eq!(::std::mem::size_of::<Token>(), 8);
	assert_eq!(::std::mem::size_of::<Kind>(), 1);
}

#[test]
fn empty() {
	let mut lexer = Lexer::new("", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 0);
}

#[test]
fn tokenizes_tilde_as_delim() {
	let mut lexer = Lexer::new("~", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Delim);
		assert_eq!(token.char(), Some('~'));
	}
	assert_eq!(lexer.offset(), 1);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 1);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 1);
}

#[test]
fn tokenizes_newlines_as_whitespace() {
	let mut lexer = Lexer::new("\r\n", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 2);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 2);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 2);
}

#[test]
fn tokenizes_multiple_newlines_as_whitespace() {
	let mut lexer = Lexer::new("\r\n", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 2);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 2);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 2);
}

#[test]
fn tokenizes_multiple_whitespace_as_whitespace() {
	let mut lexer = Lexer::new("\t \t \t", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 5);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 5);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 5);
}

#[test]
fn tokenizes_basic_selector() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new(".foo:bar[baz=bing]", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Delim);
		assert_eq!(token.char(), Some('.'));
		assert_eq!(lexer.offset(), 1);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(lexer.parse_atom(token, &allocator), Atom::from("foo"));
		assert_eq!(lexer.offset(), 4);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Colon);
		assert_eq!(token.char(), Some(':'));
		assert_eq!(lexer.offset(), 5);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(lexer.parse_atom(token, &allocator), Atom::from("bar"));
		assert_eq!(lexer.offset(), 8);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::LeftSquare);
		assert_eq!(token.char(), Some('['));
		assert_eq!(lexer.offset(), 9);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(lexer.parse_atom(token, &allocator), Atom::from("baz"));
		assert_eq!(lexer.offset(), 12);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Delim);
		assert_eq!(token.char(), Some('='));
		assert_eq!(lexer.offset(), 13);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(lexer.parse_atom(token, &allocator), Atom::from("bing"));
		assert_eq!(lexer.offset(), 17);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::RightSquare);
		assert_eq!(token.char(), Some(']'));
		assert_eq!(lexer.offset(), 18);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Eof);
		assert_eq!(lexer.offset(), 18);
	}
}

#[test]
fn tokenizes_basic_css_file() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("body { color: black }/* fin */", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(lexer.offset(), 4);
		assert_eq!(lexer.parse_atom(token, &allocator), atom!("body"));
	}
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 5);
	assert_eq!(lexer.advance().kind(), Kind::LeftCurly);
	assert_eq!(lexer.offset(), 6);
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 7);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(lexer.parse_atom(token, &allocator), atom!("color"));
	}
	assert_eq!(lexer.offset(), 12);
	assert_eq!(lexer.advance().kind(), Kind::Colon);
	assert_eq!(lexer.offset(), 13);
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 14);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(lexer.parse_atom(token, &allocator), atom!("black"));
	}
	assert_eq!(lexer.offset(), 19);
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 20);
	assert_eq!(lexer.advance().kind(), Kind::RightCurly);
	assert_eq!(lexer.offset(), 21);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Comment);
		assert_eq!(lexer.parse_str(token, &allocator), " fin ");
	}
	assert_eq!(lexer.offset(), 30);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 30);
}

#[test]
fn skips_whitespace_and_comments_with_next() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("body { color: black }/* fin */", Include::none());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(lexer.parse_atom(token, &allocator), atom!("body"));
	}
	assert_eq!(lexer.offset(), 4);
	assert_eq!(lexer.advance().kind(), Kind::LeftCurly);
	assert_eq!(lexer.offset(), 6);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(lexer.parse_atom(token, &allocator), atom!("color"));
	}
	assert_eq!(lexer.offset(), 12);
	assert_eq!(lexer.advance().kind(), Kind::Colon);
	assert_eq!(lexer.offset(), 13);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(lexer.parse_atom(token, &allocator), atom!("black"));
	}
	assert_eq!(lexer.offset(), 19);
	assert_eq!(lexer.advance().kind(), Kind::RightCurly);
	assert_eq!(lexer.offset(), 21);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
	assert_eq!(lexer.offset(), 30);
}

#[test]
fn tokenizes_unterminated_url() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("url( a", Include::none());
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "url( a");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
}

#[test]
fn tokenizes_wtf() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("\\75 rl(a)\n", Include::none());
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(lexer.parse_raw_str(token), "\\75 rl(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
}

#[test]
fn returns_correct_str_inner_value() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("@foo #foo foo( url(foo) url(  foo) 'foo' \"foo\" 20px 30% 100.0--foo", Include::none());
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::AtKeyword);
		assert_eq!(lexer.parse_raw_str(token), "@foo");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Hash);
		assert_eq!(lexer.parse_raw_str(token), "#foo");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Function);
		assert_eq!(lexer.parse_raw_str(token), "foo(");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(lexer.parse_raw_str(token), "url(foo)");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(lexer.parse_raw_str(token), "url(  foo)");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::String);
		assert_eq!(lexer.parse_raw_str(token), "'foo'");
		assert_eq!(token.quote_style(), QuoteStyle::Single);
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::String);
		assert_eq!(lexer.parse_raw_str(token), "\"foo\"");
		assert_eq!(token.quote_style(), QuoteStyle::Double);
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(lexer.parse_raw_str(token), "20px");
		assert_eq!(lexer.parse_str(token, &allocator), "px");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(lexer.parse_raw_str(token), "30%");
		assert_eq!(lexer.parse_str(token, &allocator), "%");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(lexer.parse_raw_str(token), "100.0--foo");
		assert_eq!(lexer.parse_str(token, &allocator), "--foo");
	}
}

#[test]
fn returns_correct_str_escaped_value() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("@f\\6fo #f\\6fo f\\6fo( url( f\\6fo) u\\72l( f\\6fo) 'f\\6fo'", Include::none());
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::AtKeyword);
		assert_eq!(lexer.parse_raw_str(token), "@f\\6fo");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Hash);
		assert_eq!(lexer.parse_raw_str(token), "#f\\6fo");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Function);
		assert_eq!(lexer.parse_raw_str(token), "f\\6fo(");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(lexer.parse_raw_str(token), "url( f\\6fo)");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(lexer.parse_raw_str(token), "u\\72l( f\\6fo)");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::String);
		assert_eq!(lexer.parse_raw_str(token), "'f\\6fo'");
		assert_eq!(lexer.parse_str(token, &allocator), "foo");
	}
}

#[test]
fn returns_correct_unicode_values() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("@foo🍔 '🍔' --foo-🍔", Include::none());
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::AtKeyword);
		assert_eq!(token.len(), 8);
		assert_eq!(lexer.parse_raw_str(token), "@foo🍔");
		assert_eq!(lexer.parse_str(token, &allocator), "foo🍔");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::String);
		assert_eq!(token.len(), 6);
		assert_eq!(token.quote_style(), QuoteStyle::Single);
		assert_eq!(lexer.parse_raw_str(token), "'🍔'");
		assert_eq!(lexer.parse_str(token, &allocator), "🍔");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(token.len(), 10);
		assert_eq!(lexer.parse_raw_str(token), "--foo-🍔");
		assert_eq!(lexer.parse_str(token, &allocator), "--foo-🍔");
	}
}
