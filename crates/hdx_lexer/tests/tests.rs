use bumpalo::Bump;
use hdx_lexer::{CommentStyle, DimensionUnit, Feature, Kind, Lexer, QuoteStyle, SourceOffset};

#[test]
fn tokenizes_empty() {
	let mut lexer = Lexer::new("");
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 0);
}

#[test]
fn tokenizes_tilde_as_delim() {
	let mut lexer = Lexer::new("~");
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Delim);
		assert_eq!(token, '~');
	}
	assert_eq!(lexer.offset(), 1);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 1);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 1);
}

#[test]
fn tokenizes_newlines_as_whitespace() {
	let mut lexer = Lexer::new("\n\n");
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 2);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 2);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 2);
}

#[test]
fn tokenizes_multiple_newlines_as_whitespace() {
	let mut lexer = Lexer::new("\r\n");
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 2);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 2);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 2);
}

#[test]
fn tokenizes_multiple_whitespace_as_whitespace() {
	let mut lexer = Lexer::new("\t \t \t");
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 1);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 2);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 3);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 4);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 5);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 5);
}

#[test]
fn tokenizes_ident_then_newline() {
	let allocator = Bump::default();
	let source = "foo\n";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	let token = lexer.advance();
	assert_eq!(token, Kind::Ident);
	assert_eq!(token.len(), 3);
	assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "foo");
	assert_eq!(lexer.offset(), 3);
	let token = lexer.advance();
	assert_eq!(token, Kind::Whitespace);
	assert_eq!(token.len(), 1);
	assert_eq!(lexer.offset(), 4);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 4);
}

#[test]
fn tokenizes_block_comment() {
	let allocator = Bump::default();
	let source = "/* foo */";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	let token = lexer.advance();
	assert_eq!(token, Kind::Comment);
	assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "/* foo */");
	assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), " foo ");
	assert_eq!(lexer.offset(), 9);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 9);
}

#[test]
fn tokenizes_single_line_comments_with_flag() {
	let allocator = Bump::default();
	let source = "\nfoo// bar baz bing\nbong";
	let mut lexer = Lexer::new_with_features(source, Feature::SingleLineComments);
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.advance(), Kind::Ident);
	assert_eq!(lexer.offset(), 4);
	let token = lexer.advance();
	assert_eq!(token, Kind::Comment);
	assert_eq!(token, CommentStyle::Single);
	assert_eq!(lexer.offset(), 19);
	assert_eq!(token.with_cursor(SourceOffset(4)).str_slice(source), "// bar baz bing");
	assert_eq!(token.with_cursor(SourceOffset(4)).parse_str(source, &allocator), " bar baz bing");
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 20);
	assert_eq!(lexer.advance(), Kind::Ident);
	assert_eq!(lexer.offset(), 24);
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_basic_selector() {
	let allocator = Bump::default();
	let source = ".foo:bar[baz=bing]";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Delim);
		assert_eq!(token, '.');
		assert_eq!(lexer.offset(), 1);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(1)).str_slice(source), "foo");
		assert_eq!(token.with_cursor(SourceOffset(1)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 4);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Colon);
		assert_eq!(token, ':');
		assert_eq!(lexer.offset(), 5);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(5)).str_slice(source), "bar");
		assert_eq!(token.with_cursor(SourceOffset(5)).parse_str(source, &allocator), "bar");
		assert_eq!(lexer.offset(), 8);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::LeftSquare);
		assert_eq!(token, '[');
		assert_eq!(lexer.offset(), 9);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(9)).str_slice(source), "baz");
		assert_eq!(token.with_cursor(SourceOffset(9)).parse_str(source, &allocator), "baz");
		assert_eq!(lexer.offset(), 12);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Delim);
		assert_eq!(token, '=');
		assert_eq!(lexer.offset(), 13);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(13)).str_slice(source), "bing");
		assert_eq!(token.with_cursor(SourceOffset(13)).parse_str(source, &allocator), "bing");
		assert_eq!(lexer.offset(), 17);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::RightSquare);
		assert_eq!(token, ']');
		assert_eq!(lexer.offset(), 18);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Eof);
		assert_eq!(lexer.offset(), 18);
	}
}

#[test]
fn tokenizes_basic_css_file() {
	let allocator = Bump::default();
	let source = "body { color: black }/* fin */";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(lexer.offset(), 4);
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "body");
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 5);
	assert_eq!(lexer.advance(), Kind::LeftCurly);
	assert_eq!(lexer.offset(), 6);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 7);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(7)).parse_str(source, &allocator), "color");
	}
	assert_eq!(lexer.offset(), 12);
	assert_eq!(lexer.advance(), Kind::Colon);
	assert_eq!(lexer.offset(), 13);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 14);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(14)).parse_str(source, &allocator), "black");
	}
	assert_eq!(lexer.offset(), 19);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 20);
	assert_eq!(lexer.advance(), Kind::RightCurly);
	assert_eq!(lexer.offset(), 21);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Comment);
		assert_eq!(token.with_cursor(SourceOffset(21)).parse_str(source, &allocator), " fin ");
	}
	assert_eq!(lexer.offset(), 30);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 30);
}

#[test]
fn tokenizes_skipping_whitespace_and_comments() {
	let allocator = Bump::default();
	let source = "body { color: black }/* fin */";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "body");
	}
	assert_eq!(lexer.offset(), 4);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.advance(), Kind::LeftCurly);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 7);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(7)).parse_str(source, &allocator), "color");
	}
	assert_eq!(lexer.offset(), 12);
	assert_eq!(lexer.advance(), Kind::Colon);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 14);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(14)).parse_str(source, &allocator), "black");
	}
	assert_eq!(lexer.offset(), 19);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.advance(), Kind::RightCurly);
	assert_eq!(lexer.advance(), Kind::Comment);
	assert_eq!(lexer.offset(), 30);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 30);
}

#[test]
fn tokenizes_unterminated_url() {
	let allocator = Bump::default();
	let source = "url( a";
	let mut lexer = Lexer::new(source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "url( a");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "a");
	}
}

#[test]
fn tokenizes_wtf() {
	let allocator = Bump::default();
	let source = "\\75 rl(a)\n";
	let mut lexer = Lexer::new(source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "\\75 rl(a)");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "a");
	}
}

#[test]
fn tokenizes_returning_correct_str_inner_value() {
	let allocator = Bump::default();
	let source = "@foo #foo foo( url(foo) url(  foo) 'foo' \"foo\" 20px 30% 100.0--foo";
	let mut lexer = Lexer::new(source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::AtKeyword);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "@foo");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Hash);
		assert_eq!(token.with_cursor(SourceOffset(5)).str_slice(source), "#foo");
		assert_eq!(token.with_cursor(SourceOffset(5)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 9);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Function);
		assert_eq!(token.with_cursor(SourceOffset(10)).str_slice(source), "foo(");
		assert_eq!(token.with_cursor(SourceOffset(10)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 14);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.with_cursor(SourceOffset(15)).str_slice(source), "url(foo)");
		assert_eq!(token.with_cursor(SourceOffset(15)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 23);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.with_cursor(SourceOffset(24)).str_slice(source), "url(  foo)");
		assert_eq!(token.with_cursor(SourceOffset(24)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 34);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.with_cursor(SourceOffset(35)).str_slice(source), "'foo'");
		assert_eq!(token.with_cursor(SourceOffset(35)).parse_str(source, &allocator), "foo");
		assert_eq!(token, QuoteStyle::Single);
		assert_eq!(lexer.offset(), 40);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.with_cursor(SourceOffset(41)).str_slice(source), "\"foo\"");
		assert_eq!(token.with_cursor(SourceOffset(41)).parse_str(source, &allocator), "foo");
		assert_eq!(token, QuoteStyle::Double);
		assert_eq!(lexer.offset(), 46);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.with_cursor(SourceOffset(47)).str_slice(source), "20px");
		assert_eq!(token.with_cursor(SourceOffset(47)).parse_str(source, &allocator), "px");
		assert_eq!(lexer.offset(), 51);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.with_cursor(SourceOffset(52)).str_slice(source), "30%");
		assert_eq!(token.with_cursor(SourceOffset(52)).parse_str(source, &allocator), "%");
		assert_eq!(lexer.offset(), 55);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.with_cursor(SourceOffset(56)).str_slice(source), "100.0--foo");
		assert_eq!(token.with_cursor(SourceOffset(56)).parse_str(source, &allocator), "--foo");
		assert_eq!(lexer.offset(), 66);
	}
}

#[test]
fn tokenizes_returning_correct_str_escaped_value() {
	let allocator = Bump::default();
	let source = "@f\\6fo #f\\6fo f\\6fo( url( f\\6fo) u\\72l( f\\6fo) 'f\\6fo'";
	let mut lexer = Lexer::new(source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::AtKeyword);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "@f\\6fo");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 6);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Hash);
		assert_eq!(token.with_cursor(SourceOffset(7)).str_slice(source), "#f\\6fo");
		assert_eq!(token.with_cursor(SourceOffset(7)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 13);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Function);
		assert_eq!(token.with_cursor(SourceOffset(14)).str_slice(source), "f\\6fo(");
		assert_eq!(token.with_cursor(SourceOffset(14)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 20);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.with_cursor(SourceOffset(21)).str_slice(source), "url( f\\6fo)");
		assert_eq!(token.with_cursor(SourceOffset(21)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 32);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.with_cursor(SourceOffset(33)).str_slice(source), "u\\72l( f\\6fo)");
		assert_eq!(token.with_cursor(SourceOffset(33)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 46);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.with_cursor(SourceOffset(47)).str_slice(source), "'f\\6fo'");
		assert_eq!(token.with_cursor(SourceOffset(47)).parse_str(source, &allocator), "foo");
		assert_eq!(lexer.offset(), 54);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_returning_correct_unicode_values() {
	let allocator = Bump::default();
	let source = "@foo🍔 '🍔' --foo-🍔";
	let mut lexer = Lexer::new(source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::AtKeyword);
		assert_eq!(token.len(), 8);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "@foo🍔");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "foo🍔");
		assert_eq!(lexer.offset(), 8);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.len(), 6);
		assert_eq!(token, QuoteStyle::Single);
		assert_eq!(token.with_cursor(SourceOffset(9)).str_slice(source), "'🍔'");
		assert_eq!(token.with_cursor(SourceOffset(9)).parse_str(source, &allocator), "🍔");
		assert_eq!(lexer.offset(), 15);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 10);
		assert_eq!(token.with_cursor(SourceOffset(16)).str_slice(source), "--foo-🍔");
		assert_eq!(token.with_cursor(SourceOffset(16)).parse_str(source, &allocator), "--foo-🍔");
		assert_eq!(lexer.offset(), 26);
	}
}

#[test]
fn tokenizes_numbers_into_token_bytes() {
	let source = "0 11 52 00004 12682 +12 -14 32767 -32767 4e12 0.132 .4 32768 +123456789";
	let mut lexer = Lexer::new(source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 1);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "0");
		assert_eq!(token.value(), 0.0);
		assert_eq!(lexer.offset(), 1);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 2);
		assert_eq!(token.with_cursor(SourceOffset(2)).str_slice(source), "11");
		assert_eq!(token.value(), 11.0);
		assert_eq!(lexer.offset(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 2);
		assert_eq!(token.with_cursor(SourceOffset(5)).str_slice(source), "52");
		assert_eq!(token.value(), 52.0);
		assert_eq!(lexer.offset(), 7);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 5);
		assert_eq!(token.with_cursor(SourceOffset(8)).str_slice(source), "00004");
		assert_eq!(token.value(), 4.0);
		assert_eq!(lexer.offset(), 13);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 5);
		assert_eq!(token.with_cursor(SourceOffset(14)).str_slice(source), "12682");
		assert_eq!(token.value(), 12682.0);
		assert_eq!(lexer.offset(), 19);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 3);
		assert_eq!(token.with_cursor(SourceOffset(20)).str_slice(source), "+12");
		assert_eq!(token.value(), 12.0);
		assert_eq!(lexer.offset(), 23);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 3);
		assert_eq!(token.with_cursor(SourceOffset(24)).str_slice(source), "-14");
		assert_eq!(token.value(), -14.0);
		assert_eq!(lexer.offset(), 27);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 5);
		assert_eq!(token.with_cursor(SourceOffset(28)).str_slice(source), "32767");
		assert_eq!(token.value(), 32767.0);
		assert_eq!(lexer.offset(), 33);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(34)).str_slice(source), "-32767");
		assert_eq!(token.value(), -32767.0);
		assert_eq!(lexer.offset(), 40);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 4);
		assert_eq!(token.numeric_len(), 4);
		assert_eq!(token.with_cursor(SourceOffset(41)).str_slice(source), "4e12");
		assert_eq!(token.value(), 4e12);
		assert_eq!(lexer.offset(), 45);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 5);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(token.with_cursor(SourceOffset(46)).str_slice(source), "0.132");
		assert_eq!(token.value(), 0.132);
		assert_eq!(lexer.offset(), 51);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 2);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.with_cursor(SourceOffset(52)).str_slice(source), ".4");
		assert_eq!(token.value(), 0.4);
		assert_eq!(lexer.offset(), 54);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 5);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(token.with_cursor(SourceOffset(55)).str_slice(source), "32768");
		assert_eq!(token.value(), 32768.0);
		assert_eq!(lexer.offset(), 60);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 10);
		assert_eq!(token.numeric_len(), 10);
		assert_eq!(token.with_cursor(SourceOffset(61)).str_slice(source), "+123456789");
		assert_eq!(token.value(), 123456789.0);
		assert_eq!(lexer.offset(), 71);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_small_dimensions_into_token_bytes() {
	let source =
		"0s 11px 52rem 00004dvw 2682% +12rad -14deg 8191x -8191q 1.2345678901234s 4e12px 0.132rem .4dvw 40--custom";
	let mut lexer = Lexer::new(source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 1);
		assert_eq!(token.len(), 2);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "0s");
		assert_eq!(token.value(), 0.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::S);
		assert_eq!(lexer.offset(), 2);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 4);
		assert_eq!(token.with_cursor(SourceOffset(3)).str_slice(source), "11px");
		assert_eq!(token.value(), 11.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::Px);
		assert_eq!(lexer.offset(), 7);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 5);
		assert_eq!(token.with_cursor(SourceOffset(8)).str_slice(source), "52rem");
		assert_eq!(token.value(), 52.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::Rem);
		assert_eq!(lexer.offset(), 13);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(token.len(), 8);
		assert_eq!(token.with_cursor(SourceOffset(14)).str_slice(source), "00004dvw");
		assert_eq!(token.value(), 4.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::Dvw);
		assert_eq!(lexer.offset(), 22);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 4);
		assert_eq!(token.len(), 5);
		assert_eq!(token.with_cursor(SourceOffset(23)).str_slice(source), "2682%");
		assert_eq!(token.value(), 2682.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::Percent);
		assert_eq!(lexer.offset(), 28);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 3);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(29)).str_slice(source), "+12rad");
		assert_eq!(token.value(), 12.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::Rad);
		assert_eq!(lexer.offset(), 35);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 3);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(36)).str_slice(source), "-14deg");
		assert_eq!(token.value(), -14.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::Deg);
		assert_eq!(lexer.offset(), 42);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 4);
		assert_eq!(token.len(), 5);
		assert_eq!(token.with_cursor(SourceOffset(43)).str_slice(source), "8191x");
		assert_eq!(token.value(), 8191.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::X);
		assert_eq!(lexer.offset(), 48);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(49)).str_slice(source), "-8191q");
		assert_eq!(token.value(), -8191.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::Q);
		assert_eq!(lexer.offset(), 55);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 15);
		assert_eq!(token.len(), 16);
		assert_eq!(token.with_cursor(SourceOffset(56)).str_slice(source), "1.2345678901234s");
		assert_eq!(token.value(), 1.234_567_9);
		assert_eq!(token.dimension_unit(), DimensionUnit::S);
		assert_eq!(lexer.offset(), 72);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 4);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(73)).str_slice(source), "4e12px");
		assert_eq!(token.value(), 4e12);
		assert_eq!(token.dimension_unit(), DimensionUnit::Px);
		assert_eq!(lexer.offset(), 79);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 5);
		assert_eq!(token.len(), 8);
		assert_eq!(token.with_cursor(SourceOffset(80)).str_slice(source), "0.132rem");
		assert_eq!(token.value(), 0.132);
		assert_eq!(token.dimension_unit(), DimensionUnit::Rem);
		assert_eq!(lexer.offset(), 88);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 5);
		assert_eq!(token.with_cursor(SourceOffset(89)).str_slice(source), ".4dvw");
		assert_eq!(token.value(), 0.4);
		assert_eq!(token.dimension_unit(), DimensionUnit::Dvw);
		assert_eq!(lexer.offset(), 94);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 10);
		assert_eq!(token.with_cursor(SourceOffset(95)).str_slice(source), "40--custom");
		assert_eq!(token.value(), 40.0);
		assert_eq!(token.dimension_unit(), DimensionUnit::Unknown);
		assert_eq!(lexer.offset(), 105);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_encoding_flags_for_dashed_idents() {
	let source = "foo --bar baz --bing";
	let mut lexer = Lexer::new(source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "foo");
		assert!(!token.is_dashed_ident());
		assert_eq!(lexer.offset(), 3);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(4)).str_slice(source), "--bar");
		assert!(token.is_dashed_ident());
		assert_eq!(lexer.offset(), 9);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(10)).str_slice(source), "baz");
		assert!(!token.is_dashed_ident());
		assert_eq!(lexer.offset(), 13);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(14)).str_slice(source), "--bing");
		assert!(token.is_dashed_ident());
		assert_eq!(lexer.offset(), 20);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_tricky_idents() {
	let allocator = Bump::default();
	let source = "@\\\\@ foo\\\\\n";
	let mut lexer = Lexer::new(source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::AtKeyword);
		assert_eq!(token.len(), 3);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "@\\\\");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "\\");
		assert_eq!(lexer.offset(), 3);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Delim);
		assert_eq!(token.len(), 1);
		assert_eq!(token.with_cursor(SourceOffset(3)).str_slice(source), "@");
		assert_eq!(token, '@');
		assert_eq!(lexer.offset(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 5);
		assert_eq!(token.with_cursor(SourceOffset(5)).str_slice(source), "foo\\\\");
		assert_eq!(lexer.offset(), 10);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_string_with_escaped_newlines() {
	let allocator = Bump::default();
	let source = "'\\\r\n \\\n'";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.len(), 8);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "'\\\r\n \\\n'");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), " ");
	}
}

#[test]
fn tokenizes_string_or_ident_with_null_char() {
	let allocator = Bump::default();
	let source = "fo\0o 'ba\0r' \0foo";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 4);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "fo\0o");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "fo\u{fffd}o");
		assert_eq!(lexer.offset(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(5)).str_slice(source), "'ba\0r'");
		assert_eq!(token.with_cursor(SourceOffset(5)).parse_str(source, &allocator), "ba\u{fffd}r");
		assert_eq!(lexer.offset(), 11);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 4);
		assert_eq!(token.with_cursor(SourceOffset(12)).str_slice(source), "\0foo");
		assert_eq!(token.with_cursor(SourceOffset(12)).parse_str(source, &allocator), "\u{fffd}foo");
		assert_eq!(lexer.offset(), 16);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_null_as_ident_replacement() {
	let allocator = Bump::default();
	let source = "\0 \0d ";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.with_cursor(SourceOffset(0)).offset(), 0);
		assert_eq!(token.len(), 1);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "\0");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "\u{FFFD}");
		assert_eq!(lexer.offset(), 1);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 2);
		assert_eq!(token.with_cursor(SourceOffset(2)).str_slice(source), "\0d");
		assert_eq!(token.with_cursor(SourceOffset(2)).parse_str(source, &allocator), "\u{FFFD}d");
		assert_eq!(lexer.offset(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_bad_url() {
	let allocator = Bump::default();
	let source = "url(a\") url( a a) url( a a\\)";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::BadUrl);
		assert_eq!(token.len(), 7);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "url(a\")");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "url(a\")");
		assert_eq!(lexer.offset(), 7);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::BadUrl);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(8)).str_slice(source), "url( a a)");
		assert_eq!(token.with_cursor(SourceOffset(8)).parse_str(source, &allocator), "url( a a)");
		assert_eq!(lexer.offset(), 17);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::BadUrl);
		assert_eq!(token.len(), 10);
		assert_eq!(token.with_cursor(SourceOffset(18)).str_slice(source), "url( a a\\)");
		assert_eq!(token.with_cursor(SourceOffset(18)).parse_str(source, &allocator), "url( a a\\)");
		assert_eq!(lexer.offset(), 28);
	}
}

#[test]
fn tokenizes_null_dimension() {
	let allocator = Bump::default();
	let source = "4waPtwEEGH\\\u{0000}jV3zM6hh6w30N0PC";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 1);
		assert_eq!(token.len(), 28);
		assert_eq!(token.value(), 4.0);
		assert_eq!(
			token.with_cursor(SourceOffset(0)).parse_str(source, &allocator),
			"waPtwEEGH\u{FFFD}jV3zM6hh6w30N0PC"
		);
	}
}

#[test]
fn tokenizes_string_with_escaped_crlf() {
	let allocator = Bump::default();
	let source = "'a\\12\r\nb'";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "'a\\12\r\nb'");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "a\u{0012}b");
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_idents_with_escaped_whitespace() {
	let allocator = Bump::default();
	let source = "\\61  b";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 4);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "\\61 ");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "\u{0061}");
		assert_eq!(lexer.offset(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 1);
		assert_eq!(token.with_cursor(SourceOffset(5)).str_slice(source), "b");
		assert_eq!(token.with_cursor(SourceOffset(5)).parse_str(source, &allocator), "b");
		assert_eq!(lexer.offset(), 6);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_weird_url_function_names() {
	let allocator = Bump::default();
	let source = "url(a)uRl(a)Url(a)URL(a)uRL(a)URl(a)UrL(a)\\75 rl(a)\\55 rl(a)u\\72 l(a)u\\52 l(a)ur\\4c (a)ur\\6c (a)\\75\\52\\6c(a)ur\\69(a)\\61 rl(a)";
	let mut lexer = Lexer::new(source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(0)).str_slice(source), "url(a)");
		assert_eq!(token.with_cursor(SourceOffset(0)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 6);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(6)).str_slice(source), "uRl(a)");
		assert_eq!(token.with_cursor(SourceOffset(6)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 12);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(12)).str_slice(source), "Url(a)");
		assert_eq!(token.with_cursor(SourceOffset(12)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 18);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(18)).str_slice(source), "URL(a)");
		assert_eq!(token.with_cursor(SourceOffset(18)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 24);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(24)).str_slice(source), "uRL(a)");
		assert_eq!(token.with_cursor(SourceOffset(24)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 30);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(30)).str_slice(source), "URl(a)");
		assert_eq!(token.with_cursor(SourceOffset(30)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 36);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(36)).str_slice(source), "UrL(a)");
		assert_eq!(token.with_cursor(SourceOffset(36)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 42);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(42)).str_slice(source), "\\75 rl(a)");
		assert_eq!(token.with_cursor(SourceOffset(42)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 51);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(51)).str_slice(source), "\\55 rl(a)");
		assert_eq!(token.with_cursor(SourceOffset(51)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 60);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(60)).str_slice(source), "u\\72 l(a)");
		assert_eq!(token.with_cursor(SourceOffset(60)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 69);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(69)).str_slice(source), "u\\52 l(a)");
		assert_eq!(token.with_cursor(SourceOffset(69)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 78);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(78)).str_slice(source), "ur\\4c (a)");
		assert_eq!(token.with_cursor(SourceOffset(78)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 87);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(87)).str_slice(source), "ur\\6c (a)");
		assert_eq!(token.with_cursor(SourceOffset(87)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 96);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 12);
		assert_eq!(token.with_cursor(SourceOffset(96)).str_slice(source), "\\75\\52\\6c(a)");
		assert_eq!(token.with_cursor(SourceOffset(96)).parse_str(source, &allocator), "a");
		assert_eq!(lexer.offset(), 108);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Function);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(108)).str_slice(source), "ur\\69(");
		assert_eq!(token.with_cursor(SourceOffset(108)).parse_str(source, &allocator), "uri");
		assert_eq!(lexer.offset(), 114);
	}
	assert_eq!(lexer.advance(), Kind::Ident);
	assert_eq!(lexer.advance(), Kind::RightParen);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Function);
		assert_eq!(token.len(), 7);
		assert_eq!(token.with_cursor(SourceOffset(116)).str_slice(source), "\\61 rl(");
		assert_eq!(token.with_cursor(SourceOffset(116)).parse_str(source, &allocator), "arl");
		assert_eq!(lexer.offset(), 123);
	}
	assert_eq!(lexer.advance(), Kind::Ident);
	assert_eq!(lexer.advance(), Kind::RightParen);
	assert_eq!(lexer.advance(), Kind::Eof);
}
