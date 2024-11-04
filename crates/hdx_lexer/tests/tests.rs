use bumpalo::Bump;
use hdx_atom::{atom, Atom};
use hdx_lexer::{DimensionUnit, Feature, Include, Kind, Lexer, QuoteStyle, Token};

#[test]
fn size_test() {
	assert_eq!(::std::mem::size_of::<Lexer>(), 32);
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
		assert_eq!(lexer.parse_raw_str(token), "foo");
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
		assert_eq!(lexer.parse_raw_str(token), "bar");
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
		assert_eq!(lexer.parse_raw_str(token), "baz");
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
		assert_eq!(lexer.parse_raw_str(token), "bing");
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

#[test]
fn tokenizer_encodes_small_numbers_into_token_bytes() {
	let mut lexer = Lexer::new("0 11 52 00004 12682 +12 -14 32767 -32767", Include::none());
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 1);
		assert_eq!(lexer.parse_raw_str(token), "0");
		assert_eq!(token.stored_small_number(), Some(0.0));
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 2);
		assert_eq!(lexer.parse_raw_str(token), "11");
		assert_eq!(token.stored_small_number(), Some(11.0));
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 2);
		assert_eq!(lexer.parse_raw_str(token), "52");
		assert_eq!(token.stored_small_number(), Some(52.0));
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 5);
		assert_eq!(lexer.parse_raw_str(token), "00004");
		assert_eq!(token.stored_small_number(), Some(4.0));
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 5);
		assert_eq!(lexer.parse_raw_str(token), "12682");
		assert_eq!(token.stored_small_number(), Some(12682.0));
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 3);
		assert_eq!(lexer.parse_raw_str(token), "+12");
		assert_eq!(token.stored_small_number(), Some(12.0));
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 3);
		assert_eq!(lexer.parse_raw_str(token), "-14");
		assert_eq!(token.stored_small_number(), Some(-14.0));
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 5);
		assert_eq!(lexer.parse_raw_str(token), "32767");
		assert_eq!(token.stored_small_number(), Some(32767.0));
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "-32767");
		assert_eq!(token.stored_small_number(), Some(-32767.0));
	}
}

#[test]
fn tokenizer_does_not_encode_large_or_weird_numbers_into_token_bytes() {
	let mut lexer = Lexer::new("4e12 0.132 .4 32768", Include::none());
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 4);
		assert_eq!(token.numeric_len(), 4);
		assert_eq!(lexer.parse_raw_str(token), "4e12");
		assert_eq!(token.stored_small_number(), None);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 5);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(lexer.parse_raw_str(token), "0.132");
		assert_eq!(token.stored_small_number(), None);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 2);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(lexer.parse_raw_str(token), ".4");
		assert_eq!(token.stored_small_number(), None);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Number);
		assert_eq!(token.len(), 5);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(lexer.parse_raw_str(token), "32768");
		assert_eq!(token.stored_small_number(), None);
	}
}

#[test]
fn tokenizer_encodes_known_small_dimensions_into_token_bytes() {
	let mut lexer = Lexer::new("0s 11px 52rem 00004dvw 2682% +12rad -14deg 8191x -8191q", Include::none());
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 1);
		assert_eq!(token.len(), 2);
		assert_eq!(lexer.parse_raw_str(token), "0s");
		assert_eq!(token.stored_small_number(), Some(0.0));
		assert_eq!(token.dimension_unit(), DimensionUnit::S);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 4);
		assert_eq!(lexer.parse_raw_str(token), "11px");
		assert_eq!(token.stored_small_number(), Some(11.0));
		assert_eq!(token.dimension_unit(), DimensionUnit::Px);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 5);
		assert_eq!(lexer.parse_raw_str(token), "52rem");
		assert_eq!(token.stored_small_number(), Some(52.0));
		assert_eq!(token.dimension_unit(), DimensionUnit::Rem);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(token.len(), 8);
		assert_eq!(lexer.parse_raw_str(token), "00004dvw");
		assert_eq!(token.stored_small_number(), Some(4.0));
		assert_eq!(token.dimension_unit(), DimensionUnit::Dvw);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 4);
		assert_eq!(token.len(), 5);
		assert_eq!(lexer.parse_raw_str(token), "2682%");
		assert_eq!(token.stored_small_number(), Some(2682.0));
		assert_eq!(token.dimension_unit(), DimensionUnit::Percent);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 3);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "+12rad");
		assert_eq!(token.stored_small_number(), Some(12.0));
		assert_eq!(token.dimension_unit(), DimensionUnit::Rad);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 3);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "-14deg");
		assert_eq!(token.stored_small_number(), Some(-14.0));
		assert_eq!(token.dimension_unit(), DimensionUnit::Deg);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 4);
		assert_eq!(token.len(), 5);
		assert_eq!(lexer.parse_raw_str(token), "8191x");
		assert_eq!(token.stored_small_number(), Some(8191.0));
		assert_eq!(token.dimension_unit(), DimensionUnit::X);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "-8191q");
		assert_eq!(token.stored_small_number(), Some(-8191.0));
		assert_eq!(token.dimension_unit(), DimensionUnit::Q);
	}
}

#[test]
fn tokenizer_does_not_encode_large_or_weird_dimensions_into_token_bytes() {
	let mut lexer = Lexer::new("1.2345678901234s 4e12px 0.132rem .4dvw 40--custom", Include::none());
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 15);
		assert_eq!(token.len(), 16);
		assert_eq!(lexer.parse_raw_str(token), "1.2345678901234s");
		assert_eq!(token.stored_small_number(), None);
		assert_eq!(token.dimension_unit(), DimensionUnit::Unknown);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 4);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "4e12px");
		assert_eq!(token.stored_small_number(), None);
		assert_eq!(token.dimension_unit(), DimensionUnit::Unknown);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(token.len(), 8);
		assert_eq!(lexer.parse_raw_str(token), "0.132rem");
		assert_eq!(token.stored_small_number(), None);
		assert_eq!(token.dimension_unit(), DimensionUnit::Unknown);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 5);
		assert_eq!(lexer.parse_raw_str(token), ".4dvw");
		assert_eq!(token.stored_small_number(), None);
		assert_eq!(token.dimension_unit(), DimensionUnit::Unknown);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 10);
		assert_eq!(lexer.parse_raw_str(token), "40--custom");
		assert_eq!(token.stored_small_number(), None);
		assert_eq!(token.dimension_unit(), DimensionUnit::Unknown);
	}
}

#[test]
fn tricky_idents() {
	let mut lexer = Lexer::new("@\\@ foo\\\n", Include::none());
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 4);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "4e12px");
		assert_eq!(token.stored_small_number(), None);
		assert_eq!(token.dimension_unit(), DimensionUnit::Unknown);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(token.len(), 8);
		assert_eq!(lexer.parse_raw_str(token), "0.132rem");
		assert_eq!(token.stored_small_number(), None);
		assert_eq!(token.dimension_unit(), DimensionUnit::Unknown);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 5);
		assert_eq!(lexer.parse_raw_str(token), ".4dvw");
		assert_eq!(token.stored_small_number(), None);
		assert_eq!(token.dimension_unit(), DimensionUnit::Unknown);
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 10);
		assert_eq!(lexer.parse_raw_str(token), "40--custom");
		assert_eq!(token.stored_small_number(), None);
		assert_eq!(token.dimension_unit(), DimensionUnit::Unknown);
	}
}

#[test]
fn tokenizes_string_with_escaped_newlines() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("'\\\r\n \\\n'", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::String);
		assert_eq!(token.len(), 8);
		assert_eq!(lexer.parse_raw_str(token), "'\\\r\n \\\n'");
		assert_eq!(lexer.parse_str(token, &allocator), " ");
	}
}

#[test]
fn tokenizes_string_or_ident_with_null_char() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("fo\0o 'ba\0r' \0foo", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(token.len(), 4);
		assert_eq!(lexer.parse_raw_str(token), "fo\0o");
		assert_eq!(lexer.parse_str(token, &allocator), "fo\u{fffd}o");
	}
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::String);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "'ba\0r'");
		assert_eq!(lexer.parse_str(token, &allocator), "ba\u{fffd}r");
	}
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(token.len(), 4);
		assert_eq!(lexer.parse_raw_str(token), "\0foo");
		assert_eq!(lexer.parse_str(token, &allocator), "\u{fffd}foo");
	}
	assert_eq!(lexer.advance().kind(), Kind::Eof);
}

#[test]
fn tokenizes_null_as_ident_replacement() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("\0 \0d ", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(token.offset(), 0);
		assert_eq!(token.len(), 1);
		assert_eq!(lexer.parse_raw_str(token), "\0");
		assert_eq!(lexer.parse_str(token, &allocator), "\u{FFFD}");
	}
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(token.offset(), 2);
		assert_eq!(token.len(), 2);
		assert_eq!(lexer.parse_raw_str(token), "\0d");
		assert_eq!(lexer.parse_str(token, &allocator), "\u{FFFD}d");
	}
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
}

#[test]
fn tokenizes_bad_url() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("url(a\")", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::BadUrl);
		assert_eq!(token.offset(), 0);
		assert_eq!(token.len(), 7);
		assert_eq!(lexer.parse_raw_str(token), "url(a\")");
		assert_eq!(lexer.parse_str(token, &allocator), "url(a\")");
	}
}

#[test]
fn tokenizes_null_dimension() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("4waPtwEEGH\\\u{0000}jV3zM6hh6w30N0PC", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Dimension);
		assert_eq!(token.numeric_len(), 1);
		assert_eq!(token.len(), 28);
		assert_eq!(token.stored_small_number(), None);
		assert_eq!(lexer.parse_number(token), 4.0);
		assert_eq!(lexer.parse_str(token, &allocator), "waPtwEEGH\u{FFFD}jV3zM6hh6w30N0PC");
	}
}

#[test]
fn tokenizes_string_with_escaped_crlf() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("'a\\12\r\nb'", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::String);
		assert_eq!(token.len(), 9);
		assert_eq!(lexer.parse_raw_str(token), "'a\\12\r\nb'");
		assert_eq!(lexer.parse_str(token, &allocator), "a\u{0012}b");
	}
	assert_eq!(lexer.advance().kind(), Kind::Eof);
}

#[test]
fn tokenizes_idents_with_escaped_whitespace() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("\\61  b", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(token.len(), 4);
		assert_eq!(lexer.parse_raw_str(token), "\\61 ");
		assert_eq!(lexer.parse_str(token, &allocator), "\u{0061}");
	}
	assert_eq!(lexer.advance().kind(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Ident);
		assert_eq!(token.len(), 1);
		assert_eq!(lexer.parse_raw_str(token), "b");
		assert_eq!(lexer.parse_str(token, &allocator), "b");
	}
	assert_eq!(lexer.advance().kind(), Kind::Eof);
}

#[test]
fn tokenizes_weird_url_function_names() {
	let allocator = Bump::default();
	let mut lexer = Lexer::new("url(a)uRl(a)Url(a)URL(a)uRL(a)URl(a)UrL(a)\\75 rl(a)\\55 rl(a)u\\72 l(a)u\\52 l(a)ur\\4c (a)ur\\6c (a)\\75\\52\\6c(a)ur\\69(a)\\61 rl(a)", Include::all_bits());
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "url(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "uRl(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "Url(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "URL(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "uRL(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "URl(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "UrL(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(lexer.parse_raw_str(token), "\\75 rl(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(lexer.parse_raw_str(token), "\\55 rl(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(lexer.parse_raw_str(token), "u\\72 l(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(lexer.parse_raw_str(token), "u\\52 l(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(lexer.parse_raw_str(token), "ur\\4c (a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(lexer.parse_raw_str(token), "ur\\6c (a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Url);
		assert_eq!(token.len(), 12);
		assert_eq!(lexer.parse_raw_str(token), "\\75\\52\\6c(a)");
		assert_eq!(lexer.parse_str(token, &allocator), "a");
	}
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Function);
		assert_eq!(token.len(), 6);
		assert_eq!(lexer.parse_raw_str(token), "ur\\69(");
		assert_eq!(lexer.parse_str(token, &allocator), "uri");
	}
	assert_eq!(lexer.advance().kind(), Kind::Ident);
	assert_eq!(lexer.advance().kind(), Kind::RightParen);
	{
		let token = lexer.advance();
		assert_eq!(token.kind(), Kind::Function);
		assert_eq!(token.len(), 7);
		assert_eq!(lexer.parse_raw_str(token), "\\61 rl(");
		assert_eq!(lexer.parse_str(token, &allocator), "arl");
	}
	assert_eq!(lexer.advance().kind(), Kind::Ident);
	assert_eq!(lexer.advance().kind(), Kind::RightParen);
	assert_eq!(lexer.advance().kind(), Kind::Eof);
}
