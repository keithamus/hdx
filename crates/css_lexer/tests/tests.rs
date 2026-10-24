use allocator_api2::alloc::Global;
use atom_set::AtomSet;
use css_lexer::{CommentStyle, EmptyAtomSet, Feature, Kind, Lexer, QuoteStyle, SourceCursor, SourceOffset};
use derive_atom_set::AtomSet as DeriveAtomSet;

#[test]
fn tokenizes_empty() {
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "");
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_unicode_range_single_codepoint() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+A5", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0xA5);
	assert_eq!(token.unicode_range_end(), 0xA5);
	assert_eq!(token.len(), 4);
	assert_eq!(lexer.offset(), 4);
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_unicode_range_basic_range() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+0-7F", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0x0);
	assert_eq!(token.unicode_range_end(), 0x7F);
	assert_eq!(token.len(), 6);
	assert_eq!(lexer.offset(), 6);
}

#[test]
fn tokenizes_unicode_range_with_wildcards() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+30??", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0x3000);
	assert_eq!(token.unicode_range_end(), 0x30FF);
	assert_eq!(token.len(), 6);
}

#[test]
fn tokenizes_unicode_range_full_wildcards() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+??????", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0x000000);
	assert_eq!(token.unicode_range_end(), 0xFFFFFF);
	assert_eq!(token.len(), 8); // "U+" + 6 * "?"
}

#[test]
fn tokenizes_unicode_range_lowercase_u() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "u+590-5ff", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0x590);
	assert_eq!(token.unicode_range_end(), 0x5FF);
	assert_eq!(token.len(), 9);
}

#[test]
fn tokenizes_unicode_range_six_digit_range() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+000000-10FFFF", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0x0);
	assert_eq!(token.unicode_range_end(), 0x10FFFF);
	assert_eq!(token.len(), 15);
}

#[test]
fn tokenizes_unicode_range_single_hex_range() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+FF00-FF9F", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0xFF00);
	assert_eq!(token.unicode_range_end(), 0xFF9F);
	assert_eq!(token.len(), 11);
}

#[test]
fn tokenizes_unicode_range_in_comma_list() {
	let source = "U+A5, U+4E00-9FFF";
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, source, Feature::UnicodeRange);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::UnicodeRange);
		assert_eq!(token.unicode_range_start(), 0xA5);
		assert_eq!(token.unicode_range_end(), 0xA5);
		assert_eq!(token.len(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Comma);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::UnicodeRange);
		assert_eq!(token.unicode_range_start(), 0x4E00);
		assert_eq!(token.unicode_range_end(), 0x9FFF);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn unicode_range_without_feature_falls_back_to_ident() {
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "U+A5");
	let token = lexer.advance();
	assert_eq!(token, Kind::Ident);
}

#[test]
fn unicode_range_not_matching_pattern_falls_back_to_ident() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+G", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::Ident);
}

#[test]
fn unicode_range_stops_after_six_hex_digits() {
	// U+1234567 should be U+123456 (6 hex) then "7" as separate token
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+1234567", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0x123456);
	assert_eq!(token.unicode_range_end(), 0x123456);
	assert_eq!(token.len(), 8); // "U+123456"
	let next = lexer.advance();
	assert_eq!(next, Kind::Number); // "7"
}

#[test]
fn unicode_range_wildcards_limited_to_six_total() {
	// U+1?? has 1 hex + 2 ? = 3 < 6, so it greedily consumes up to 6-1=5 ?s
	// But there are only 2 ?s, so total is 3
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+1??", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0x100);
	assert_eq!(token.unicode_range_end(), 0x1FF);
	assert_eq!(token.len(), 5);
}

#[test]
fn unicode_range_wildcard_no_hex_digits() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+??", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0x00);
	assert_eq!(token.unicode_range_end(), 0xFF);
	assert_eq!(token.len(), 4);
}

#[test]
fn tokenizes_unicode_range_font_face_example() {
	let source = "U+0000-00FF";
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, source, Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0x0);
	assert_eq!(token.unicode_range_end(), 0xFF);
	assert_eq!(token.len(), 11);
}

#[test]
fn unicode_range_end_range_stops_at_six_hex_digits() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+0-1234567", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0x0);
	assert_eq!(token.unicode_range_end(), 0x123456);
	assert_eq!(token.len(), 10); // "U+0-123456" = 2 + 1 + 1 + 6
	assert_eq!(lexer.advance(), Kind::Number); // "7"
}

#[test]
fn unicode_range_token_values_roundtrip() {
	{
		let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+A5", Feature::UnicodeRange);
		let token = lexer.advance();
		assert_eq!(token.unicode_range_start(), 0xA5);
		assert_eq!(token.unicode_range_end(), 0xA5);
		assert_eq!(token.len(), 4);
	}
	{
		let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+000000-10FFFF", Feature::UnicodeRange);
		let token = lexer.advance();
		assert_eq!(token.unicode_range_start(), 0x0);
		assert_eq!(token.unicode_range_end(), 0x10FFFF);
		assert_eq!(token.len(), 15);
	}
	{
		let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+1F600", Feature::UnicodeRange);
		let token = lexer.advance();
		assert_eq!(token.unicode_range_start(), 0x1F600);
		assert_eq!(token.unicode_range_end(), 0x1F600);
		assert_eq!(token.len(), 7);
	}
}

#[test]
fn unicode_range_dash_not_followed_by_hex_is_single_range() {
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "U+A5-G", Feature::UnicodeRange);
	let token = lexer.advance();
	assert_eq!(token, Kind::UnicodeRange);
	assert_eq!(token.unicode_range_start(), 0xA5);
	assert_eq!(token.unicode_range_end(), 0xA5);
	assert_eq!(token.len(), 4); // "U+A5"
}

#[test]
fn tokenizes_tilde_as_delim() {
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "~");
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
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "\n\n");
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
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "\r\n");
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
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, "\t \t \t", Feature::SeparateWhitespace);
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
	let source = "foo\n";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	let token = lexer.advance();
	assert_eq!(token, Kind::Ident);
	assert_eq!(token.len(), 3);
	let c = token.with_cursor(SourceOffset(0));
	let str = c.str_slice(source);
	let sc = SourceCursor::from(c, str);
	assert_eq!(sc.parse(Global), "foo");
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
	let source = "/* foo */";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	let token = lexer.advance();
	assert_eq!(token, Kind::Comment);
	let c = token.with_cursor(SourceOffset(0));
	let str = c.str_slice(source);
	let sc = SourceCursor::from(c, str);
	assert_eq!(str, "/* foo */");
	assert_eq!(sc.parse(Global), " foo ");
	assert_eq!(lexer.offset(), 9);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 9);
}

#[test]
fn tokenizes_single_line_comments_with_flag() {
	let source = "\nfoo// bar baz bing\nbong";
	let mut lexer = Lexer::new_with_features(&EmptyAtomSet::ATOMS, source, Feature::SingleLineComments);
	assert_eq!(lexer.offset(), 0);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.advance(), Kind::Ident);
	assert_eq!(lexer.offset(), 4);
	let token = lexer.advance();
	assert_eq!(token, Kind::Comment);
	assert_eq!(token, CommentStyle::Single);
	assert_eq!(lexer.offset(), 19);
	let c = token.with_cursor(SourceOffset(4));
	let str = c.str_slice(source);
	let sc = SourceCursor::from(c, str);
	assert_eq!(str, "// bar baz bing");
	assert_eq!(sc.parse(Global), " bar baz bing");
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 20);
	assert_eq!(lexer.advance(), Kind::Ident);
	assert_eq!(lexer.offset(), 24);
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_basic_selector() {
	let source = ".foo:bar[baz=bing]";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
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
		let c = token.with_cursor(SourceOffset(1));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "foo");
		assert_eq!(sc.parse(Global), "foo");
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
		let c = token.with_cursor(SourceOffset(5));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "bar");
		assert_eq!(sc.parse(Global), "bar");
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
		let c = token.with_cursor(SourceOffset(9));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "baz");
		assert_eq!(sc.parse(Global), "baz");
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
		let c = token.with_cursor(SourceOffset(13));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "bing");
		assert_eq!(sc.parse(Global), "bing");
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
	let source = "body { color: black }/* fin */";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(lexer.offset(), 4);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "body");
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
		let c = token.with_cursor(SourceOffset(7));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "color");
	}
	assert_eq!(lexer.offset(), 12);
	assert_eq!(lexer.advance(), Kind::Colon);
	assert_eq!(lexer.offset(), 13);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 14);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		let c = token.with_cursor(SourceOffset(14));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "black");
	}
	assert_eq!(lexer.offset(), 19);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 20);
	assert_eq!(lexer.advance(), Kind::RightCurly);
	assert_eq!(lexer.offset(), 21);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Comment);
		let c = token.with_cursor(SourceOffset(21));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), " fin ");
	}
	assert_eq!(lexer.offset(), 30);
	assert_eq!(lexer.advance(), Kind::Eof);
	assert_eq!(lexer.offset(), 30);
}

#[test]
fn tokenizes_skipping_whitespace_and_comments() {
	let source = "body { color: black }/* fin */";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "body");
	}
	assert_eq!(lexer.offset(), 4);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.advance(), Kind::LeftCurly);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 7);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		let c = token.with_cursor(SourceOffset(7));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "color");
	}
	assert_eq!(lexer.offset(), 12);
	assert_eq!(lexer.advance(), Kind::Colon);
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.offset(), 14);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		let c = token.with_cursor(SourceOffset(14));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "black");
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
	let source = "url( a";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "url( a");
		assert_eq!(sc.parse(Global), "a");
	}
}

#[test]
fn tokenizes_escaped_url() {
	let source = "url( \\)";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 7);
		assert!(token.contains_escape_chars());
		assert!(token.url_has_leading_space());
		assert!(!token.url_has_closing_paren());
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "url( \\)");
		assert_eq!(sc.parse(Global), ")");
	}
}

#[test]
fn tokenizes_wtf() {
	let source = "\\75 rl(a)\n";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "\\75 rl(a)");
		assert_eq!(sc.parse(Global), "a");
	}
}

#[test]
fn tokenizes_returning_correct_str_inner_value() {
	let source = "@foo #foo foo( url(foo) url(  foo) 'foo' \"foo\" 20px 30% 100.0--foo";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::AtKeyword);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "@foo");
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(lexer.offset(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Hash);
		assert_eq!(token.with_cursor(SourceOffset(5)).str_slice(source), "#foo");
		let c = token.with_cursor(SourceOffset(5));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(token.hex_value(), 0);
		assert_eq!(lexer.offset(), 9);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Function);
		assert_eq!(token.with_cursor(SourceOffset(10)).str_slice(source), "foo(");
		let c = token.with_cursor(SourceOffset(10));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(lexer.offset(), 14);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.with_cursor(SourceOffset(15)).str_slice(source), "url(foo)");
		let c = token.with_cursor(SourceOffset(15));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(lexer.offset(), 23);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.with_cursor(SourceOffset(24)).str_slice(source), "url(  foo)");
		let c = token.with_cursor(SourceOffset(24));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(lexer.offset(), 34);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.with_cursor(SourceOffset(35)).str_slice(source), "'foo'");
		let c = token.with_cursor(SourceOffset(35));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(token, QuoteStyle::Single);
		assert_eq!(lexer.offset(), 40);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.with_cursor(SourceOffset(41)).str_slice(source), "\"foo\"");
		let c = token.with_cursor(SourceOffset(41));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(token, QuoteStyle::Double);
		assert_eq!(lexer.offset(), 46);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.with_cursor(SourceOffset(47)).str_slice(source), "20px");
		let c = token.with_cursor(SourceOffset(47));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "px");
		assert_eq!(lexer.offset(), 51);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.with_cursor(SourceOffset(52)).str_slice(source), "30%");
		let c = token.with_cursor(SourceOffset(52));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "%");
		assert_eq!(lexer.offset(), 55);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.with_cursor(SourceOffset(56)).str_slice(source), "100.0--foo");
		let c = token.with_cursor(SourceOffset(56));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "--foo");
		assert_eq!(lexer.offset(), 66);
	}
}

#[test]
fn tokenizes_returning_correct_str_escaped_value() {
	let source = "@f\\6fo #f\\6fo f\\6fo( url( f\\6fo) u\\72l( f\\6fo) 'f\\6fo'";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::AtKeyword);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "@f\\6fo");
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(lexer.offset(), 6);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Hash);
		let c = token.with_cursor(SourceOffset(7));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "#f\\6fo");
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(token.hex_value(), 0);
		assert_eq!(lexer.offset(), 13);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Function);
		assert_eq!(token.with_cursor(SourceOffset(14)).str_slice(source), "f\\6fo(");
		let c = token.with_cursor(SourceOffset(14));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(lexer.offset(), 20);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.with_cursor(SourceOffset(21)).str_slice(source), "url( f\\6fo)");
		let c = token.with_cursor(SourceOffset(21));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(lexer.offset(), 32);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.with_cursor(SourceOffset(33)).str_slice(source), "u\\72l( f\\6fo)");
		let c = token.with_cursor(SourceOffset(33));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(lexer.offset(), 46);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.with_cursor(SourceOffset(47)).str_slice(source), "'f\\6fo'");
		let c = token.with_cursor(SourceOffset(47));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "foo");
		assert_eq!(lexer.offset(), 54);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_returning_correct_unicode_values() {
	let source = "@foo🍔 '🍔' --foo-🍔";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::AtKeyword);
		assert_eq!(token.len(), 8);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "@foo🍔");
		assert_eq!(sc.parse(Global), "foo🍔");
		assert_eq!(lexer.offset(), 8);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.len(), 6);
		assert_eq!(token, QuoteStyle::Single);
		assert_eq!(token.with_cursor(SourceOffset(9)).str_slice(source), "'🍔'");
		let c = token.with_cursor(SourceOffset(9));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "🍔");
		assert_eq!(lexer.offset(), 15);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 10);
		assert_eq!(token.with_cursor(SourceOffset(16)).str_slice(source), "--foo-🍔");
		let c = token.with_cursor(SourceOffset(16));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "--foo-🍔");
		assert_eq!(lexer.offset(), 26);
	}
}

#[test]
fn tokenizes_numbers_into_token_bytes() {
	let source = "0 11 52 00004 12682 +12 -14 32767 -32767 4e12 0.132 .4 32768 +123456789";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Number);
		assert_eq!(token.len(), 1);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		assert_eq!(str, "0");
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
fn tokenizes_encoding_flags_for_dashed_idents() {
	let source = "foo --bar baz --bing";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "foo");
		assert_eq!(sc.parse(Global), "foo");
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
	let source = "@\\\\@ foo\\\\\n";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::AtKeyword);
		assert_eq!(token.len(), 3);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "@\\\\");
		assert_eq!(sc.parse(Global), "\\");
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
	let source = "'\\\r\n \\\n'";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.len(), 8);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "'\\\r\n \\\n'");
		assert_eq!(sc.parse(Global), " ");
	}
}

#[test]
fn tokenizes_string_or_ident_with_null_char() {
	let source = "fo\0o 'ba\0r' \0foo";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 4);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "fo\0o");
		assert_eq!(sc.parse(Global), "fo\u{fffd}o");
		assert_eq!(lexer.offset(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.len(), 6);
		let c = token.with_cursor(SourceOffset(5));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "'ba\0r'");
		assert_eq!(sc.parse(Global), "ba\u{fffd}r");
		assert_eq!(lexer.offset(), 11);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 4);
		assert_eq!(token.with_cursor(SourceOffset(12)).str_slice(source), "\0foo");
		let c = token.with_cursor(SourceOffset(12));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "\u{fffd}foo");
		assert_eq!(lexer.offset(), 16);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_null_as_ident_replacement() {
	let source = "\0 \0d ";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 1);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "\0");
		assert_eq!(sc.parse(Global), "\u{FFFD}");
		assert_eq!(lexer.offset(), 1);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 2);
		assert_eq!(token.with_cursor(SourceOffset(2)).str_slice(source), "\0d");
		let c = token.with_cursor(SourceOffset(2));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "\u{FFFD}d");
		assert_eq!(lexer.offset(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_bad_url() {
	let source = "url(a\") url( a a) url( a a\\)";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::BadUrl);
		assert_eq!(token.len(), 7);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "url(a\")");
		assert_eq!(sc.parse(Global), "url(a\")");
		assert_eq!(lexer.offset(), 7);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::BadUrl);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(8)).str_slice(source), "url( a a)");
		let c = token.with_cursor(SourceOffset(8));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "url( a a)");
		assert_eq!(lexer.offset(), 17);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::BadUrl);
		assert_eq!(token.len(), 10);
		assert_eq!(token.with_cursor(SourceOffset(18)).str_slice(source), "url( a a\\)");
		let c = token.with_cursor(SourceOffset(18));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "url( a a\\)");
		assert_eq!(lexer.offset(), 28);
	}
}

#[test]
fn tokenizes_null_dimension() {
	let source = "4waPtwEEGH\\\u{0000}jV3zM6hh6w30N0PC";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 1);
		assert_eq!(token.len(), 28);
		assert_eq!(token.value(), 4.0);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "waPtwEEGH\u{FFFD}jV3zM6hh6w30N0PC");
	}
}

#[test]
fn tokenizes_string_with_escaped_crlf() {
	let source = "'a\\12\r\nb'";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::String);
		assert_eq!(token.len(), 9);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "'a\\12\r\nb'");
		assert_eq!(sc.parse(Global), "a\u{0012}b");
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_idents_with_escaped_whitespace() {
	let source = "\\61  b";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 4);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "\\61 ");
		assert_eq!(sc.parse(Global), "\u{0061}");
		assert_eq!(lexer.offset(), 4);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident);
		assert_eq!(token.len(), 1);
		let c = token.with_cursor(SourceOffset(5));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "b");
		assert_eq!(sc.parse(Global), "b");
		assert_eq!(lexer.offset(), 6);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_weird_url_function_names() {
	let source = "url(a)uRl(a)Url(a)URL(a)uRL(a)URl(a)UrL(a)\\75 rl(a)\\55 rl(a)u\\72 l(a)u\\52 l(a)ur\\4c (a)ur\\6c (a)\\75\\52\\6c(a)ur\\69(a)\\61 rl(a)";
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, source);
	assert_eq!(lexer.offset(), 0);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "url(a)");
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 6);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(6)).str_slice(source), "uRl(a)");
		let c = token.with_cursor(SourceOffset(6));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 12);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(12)).str_slice(source), "Url(a)");
		let c = token.with_cursor(SourceOffset(12));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 18);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(18)).str_slice(source), "URL(a)");
		let c = token.with_cursor(SourceOffset(18));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 24);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(24)).str_slice(source), "uRL(a)");
		let c = token.with_cursor(SourceOffset(24));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 30);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(30)).str_slice(source), "URl(a)");
		let c = token.with_cursor(SourceOffset(30));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 36);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(36)).str_slice(source), "UrL(a)");
		let c = token.with_cursor(SourceOffset(36));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 42);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(42)).str_slice(source), "\\75 rl(a)");
		let c = token.with_cursor(SourceOffset(42));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 51);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(51)).str_slice(source), "\\55 rl(a)");
		let c = token.with_cursor(SourceOffset(51));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 60);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(60)).str_slice(source), "u\\72 l(a)");
		let c = token.with_cursor(SourceOffset(60));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 69);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(69)).str_slice(source), "u\\52 l(a)");
		let c = token.with_cursor(SourceOffset(69));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 78);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(78)).str_slice(source), "ur\\4c (a)");
		let c = token.with_cursor(SourceOffset(78));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 87);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 9);
		assert_eq!(token.with_cursor(SourceOffset(87)).str_slice(source), "ur\\6c (a)");
		let c = token.with_cursor(SourceOffset(87));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 96);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Url);
		assert_eq!(token.len(), 12);
		assert_eq!(token.with_cursor(SourceOffset(96)).str_slice(source), "\\75\\52\\6c(a)");
		let c = token.with_cursor(SourceOffset(96));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "a");
		assert_eq!(lexer.offset(), 108);
	}
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Function);
		assert_eq!(token.len(), 6);
		assert_eq!(token.with_cursor(SourceOffset(108)).str_slice(source), "ur\\69(");
		let c = token.with_cursor(SourceOffset(108));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "uri");
		assert_eq!(lexer.offset(), 114);
	}
	assert_eq!(lexer.advance(), Kind::Ident);
	assert_eq!(lexer.advance(), Kind::RightParen);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Function);
		assert_eq!(token.len(), 7);
		assert_eq!(token.with_cursor(SourceOffset(116)).str_slice(source), "\\61 rl(");
		let c = token.with_cursor(SourceOffset(116));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(sc.parse(Global), "arl");
		assert_eq!(lexer.offset(), 123);
	}
	assert_eq!(lexer.advance(), Kind::Ident);
	assert_eq!(lexer.advance(), Kind::RightParen);
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn tokenizes_hex_values_correctly() {
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "#ff0");
	let token = lexer.advance();
	assert_eq!(format!("{:x}", token.hex_value()), "ffff00ff");
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "#ffg");
	let token = lexer.advance();
	assert_eq!(format!("{:x}", token.hex_value()), "0");
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "#CAFEBABE");
	let token = lexer.advance();
	assert_eq!(format!("{:x}", token.hex_value()), "cafebabe");
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "#CAFE BABE");
	let token = lexer.advance();
	assert_eq!(format!("{:x}", token.hex_value()), "ccaaffee");
}

#[test]
fn tokenizes_atoms_correctly() {
	#[derive(DeriveAtomSet, Debug, Default, PartialEq, Copy, Clone)]
	enum CustomAtom {
		#[default]
		Empty,
		Url,
		Foo,
		Bar,
		Baz,
	}

	static CUSTOM_ATOMS: CustomAtom = CustomAtom::Empty;
	let mut lexer = Lexer::new(&CUSTOM_ATOMS, "foo");
	let token = lexer.advance();
	assert_eq!(CustomAtom::from_bits(token.atom_bits()), CustomAtom::Foo);

	let mut lexer = Lexer::new(&CUSTOM_ATOMS, "bar");
	let token = lexer.advance();
	assert_eq!(CustomAtom::from_bits(token.atom_bits()), CustomAtom::Bar);

	let mut lexer = Lexer::new(&CUSTOM_ATOMS, "baz");
	let token = lexer.advance();
	assert_eq!(CustomAtom::from_bits(token.atom_bits()), CustomAtom::Baz);

	let mut lexer = Lexer::new(&CUSTOM_ATOMS, "baz(");
	let token = lexer.advance();
	assert_eq!(CustomAtom::from_bits(token.atom_bits()), CustomAtom::Baz);

	let mut lexer = Lexer::new(&CUSTOM_ATOMS, "@baz(");
	let token = lexer.advance();
	assert_eq!(CustomAtom::from_bits(token.atom_bits()), CustomAtom::Baz);

	let mut lexer = Lexer::new(&CUSTOM_ATOMS, "--baz");
	let token = lexer.advance();
	assert!(token.is_dashed_ident());
	assert_eq!(token.atom_bits(), 0);

	let mut lexer = Lexer::new(&CUSTOM_ATOMS, "18foo");
	let token = lexer.advance();
	assert_eq!(CustomAtom::from_bits(token.atom_bits()), CustomAtom::Foo);

	let mut lexer = Lexer::new(&CUSTOM_ATOMS, "18--foo");
	let token = lexer.advance();
	assert_eq!(token.atom_bits(), 0);
}

#[test]
fn tokenizes_escaped_idents_from_plain_runs_and_escapes() {
	#[derive(DeriveAtomSet, Debug, Default, PartialEq, Copy, Clone)]
	enum CustomAtom {
		#[default]
		Empty,
		Foo,
		Bar,
		Baz,
	}

	static CUSTOM_ATOMS: CustomAtom = CustomAtom::Empty;
	for (source, atom, lower, dashed) in [
		("f\\6f o", CustomAtom::Foo, true, false),
		("b\\61r", CustomAtom::Bar, true, false),
		("\\62 az", CustomAtom::Baz, true, false),
		("ba\\z", CustomAtom::Baz, true, false),
		("b\\41r", CustomAtom::Bar, true, false),
		("Ba\\r", CustomAtom::Bar, false, false),
		("a\\:b_c", CustomAtom::Empty, false, false),
		("a\\:b-c", CustomAtom::Empty, true, false),
		("--a\\:b", CustomAtom::Empty, true, true),
		("sm\\:hover\\:bg-blue-500", CustomAtom::Empty, true, false),
		("caf\u{e9}\\:x", CustomAtom::Empty, false, false),
	] {
		let mut lexer = Lexer::new(&CUSTOM_ATOMS, source);
		let token = lexer.advance();
		assert_eq!(token, Kind::Ident, "{source:?}");
		assert_eq!(token.len() as usize, source.len(), "{source:?}");
		assert!(token.contains_escape_chars(), "{source:?}");
		assert_eq!(CustomAtom::from_bits(token.atom_bits()), atom, "{source:?}");
		assert_eq!(token.is_lower_case(), lower, "{source:?}");
		assert_eq!(token.is_dashed_ident(), dashed, "{source:?}");
		assert_eq!(lexer.advance(), Kind::Eof, "{source:?}");
	}
}

#[test]
fn tokenizes_escaped_dimensions_into_token_bytes() {
	#[derive(DeriveAtomSet, Debug, Default, PartialEq, Copy, Clone)]
	enum CustomAtom {
		#[default]
		None,
		Url,
		S,
		Px,
		Cqmin,
	}

	const CUSTOM_ATOMS: CustomAtom = CustomAtom::None;
	let source = "0\\73  11\\50\\78  52\\63 \\71 \\6d \\69 \\6e";
	let mut lexer = Lexer::new(&CUSTOM_ATOMS, source);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 1);
		assert_eq!(token.len(), 5);
		let c = token.with_cursor(SourceOffset(0));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "0\\73 ");
		assert_eq!(sc.parse(Global), "s");
		assert_eq!(token.value(), 0.0);
		assert_eq!(CustomAtom::from_bits(token.atom_bits()), CustomAtom::S);
		assert_eq!(lexer.offset(), 5);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 9);
		let c = token.with_cursor(SourceOffset(6));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "11\\50\\78 ");
		assert_eq!(sc.parse(Global), "Px");
		assert_eq!(token.value(), 11.0);
		assert_eq!(CustomAtom::from_bits(token.atom_bits()), CustomAtom::Px);
		assert_eq!(lexer.offset(), 15);
	}
	assert_eq!(lexer.advance(), Kind::Whitespace);
	{
		let token = lexer.advance();
		assert_eq!(token, Kind::Dimension);
		assert_eq!(token.numeric_len(), 2);
		assert_eq!(token.len(), 21);
		let c = token.with_cursor(SourceOffset(16));
		let str = c.str_slice(source);
		let sc = SourceCursor::from(c, str);
		assert_eq!(str, "52\\63 \\71 \\6d \\69 \\6e");
		assert_eq!(sc.parse(Global), "cqmin");
		assert_eq!(token.value(), 52.0);
		assert_eq!(CustomAtom::from_bits(token.atom_bits()), CustomAtom::Cqmin);
		assert_eq!(lexer.offset(), 37);
	}
	assert_eq!(lexer.advance(), Kind::Eof);
}

#[test]
fn test_bad_flag_manipulation() {
	let mut lexer = Lexer::new(&EmptyAtomSet::ATOMS, "foo");
	let token = lexer.advance();
	assert_eq!(token, Kind::Ident);
	assert!(!token.is_bad());
	let bad_token = token.with_bad_flag();
	assert!(bad_token.is_bad());
	assert_eq!(bad_token, Kind::BadIdent);
}
