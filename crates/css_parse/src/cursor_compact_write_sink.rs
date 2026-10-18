use crate::{
	AssociatedWhitespaceRules, Cursor, CursorSink, Kind, KindSet, QuoteStyle, SourceCursor, SourceCursorSink, Token,
};

/// This is a [CursorSink] that wraps a sink (`impl SourceCursorSink`) and on each [CursorSink::append()] call, will write
/// the contents of the cursor [Cursor] given into the given sink - using the given `&'a str` as the original source.
/// Some tokens will not be output, and Whitespace tokens will always write out as a single `' '`. It can be used as a
/// light-weight minifier for ToCursors structs.
pub struct CursorCompactWriteSink<'a, T: SourceCursorSink<'a>> {
	source_text: &'a str,
	sink: T,
	last_token: Option<Token>,
	pending: Option<SourceCursor<'a>>,
	pending_comment: Option<SourceCursor<'a>>,
}

// Tokens that get buffered as `pending` rather than emitted immediately.
const PENDING_KINDSET: KindSet = KindSet::new(&[Kind::Semicolon, Kind::Whitespace]);
// `;` is redundant immediately before/after these.
const REDUNDANT_SEMI_KINDSET: KindSet = KindSet::new(&[Kind::Semicolon, Kind::RightCurly]);
// Whitespace immediately before these tokens is never meaningful in any CSS grammar, so it can always be removed -
// even when the parser marked it significant. `:` is absent because `.a :hover` is not `.a:hover`, and `)`/`]` only
// appear here as the *closing* token of a block, where `foo( a )` is always `foo(a)`.
const NO_WHITESPACE_BEFORE_KINDSET: KindSet = KindSet::new(&[
	Kind::Whitespace,
	Kind::Comma,
	Kind::Semicolon,
	Kind::LeftCurly,
	Kind::RightCurly,
	Kind::RightParen,
	Kind::RightSquare,
	Kind::Eof,
]);
// Whitespace immediately after these tokens is never meaningful. `)`/`]` are absent because `:is(a) b` and `[x] b`
// require a space, therefore we cannot reliable elide them.
const NO_WHITESPACE_AFTER_KINDSET: KindSet = KindSet::new(&[
	Kind::Comma,
	Kind::Semicolon,
	Kind::Colon,
	Kind::LeftCurly,
	Kind::RightCurly,
	Kind::LeftParen,
	Kind::LeftSquare,
]);

impl<'a, T: SourceCursorSink<'a>> CursorCompactWriteSink<'a, T> {
	pub fn new(source_text: &'a str, sink: T) -> Self {
		Self { source_text, sink, last_token: None, pending: None, pending_comment: None }
	}

	/// Would emitting `next` immediately after `last_token` change tokenisation?
	fn needs_separator(&self, next: Token) -> bool {
		self.last_token.is_some_and(|t| t.needs_separator_for(next))
	}

	/// True when the previously-emitted token never needs a separator after it
	/// (e.g. `,`, `)`, `{`). Whitespace can be dropped and re-injection skipped.
	fn last_forbids_ws_after(&self) -> bool {
		self.last_token.is_some_and(|t| t == NO_WHITESPACE_AFTER_KINDSET)
	}

	fn emit(&mut self, c: SourceCursor<'a>) {
		self.last_token = Some(c.token());
		self.sink.append(c);
	}

	fn write(&mut self, c: SourceCursor<'a>) {
		let kind = c.token().kind();
		if kind == Kind::Comment {
			let can_separate =
				self.pending.is_none() && self.pending_comment.is_none() && !self.last_forbids_ws_after();
			if can_separate {
				self.pending_comment = Some(c);
			}
			return;
		}
		if self.pending_comment.take().is_some()
			&& kind != Kind::Whitespace
			&& kind != Kind::Eof
			&& self.needs_separator(c.token())
		{
			self.emit(SourceCursor::EMPTY_COMMENT);
		}

		if kind == Kind::Whitespace && self.pending.is_some_and(|c| c == Kind::Semicolon) {
			return;
		}

		let enforce_before = c.token() == AssociatedWhitespaceRules::EnforceBefore;
		let suppress_separator = self.last_forbids_ws_after() && !enforce_before;
		let significant_pending_whitespace = self.pending.is_some_and(|p| p.token().whitespace_is_significant());
		if let Some(prev) = self.pending.take() {
			let keep = match prev.token().kind() {
				Kind::Semicolon => {
					kind != REDUNDANT_SEMI_KINDSET && self.last_token.is_some_and(|t| t != REDUNDANT_SEMI_KINDSET)
				}
				_ if suppress_separator || (!enforce_before && kind == NO_WHITESPACE_BEFORE_KINDSET) => false,
				_ => (significant_pending_whitespace && self.last_token.is_some()) || self.needs_separator(c.token()),
			};
			if keep {
				self.emit(prev.compact());
			}
		}

		if kind == PENDING_KINDSET {
			// Adjacent whitespace tokens collapse into one, which stays significant if either part was.
			self.pending = Some(if significant_pending_whitespace && kind == Kind::Whitespace {
				c.with_significant_whitespace(true)
			} else {
				c
			});
			return;
		}
		if kind == Kind::Eof {
			return;
		}

		if !suppress_separator && self.needs_separator(c.token()) {
			self.sink.append(SourceCursor::SPACE);
		}

		let out = if kind == Kind::String { c.with_quotes(QuoteStyle::Double).compact() } else { c.compact() };
		self.emit(out);
	}
}

impl<'a, T: SourceCursorSink<'a>> Drop for CursorCompactWriteSink<'a, T> {
	fn drop(&mut self) {
		if let Some(prev) = self.pending.take()
			&& prev == Kind::Semicolon
		{
			self.emit(prev);
		}
	}
}

impl<'a, T: SourceCursorSink<'a>> CursorSink for CursorCompactWriteSink<'a, T> {
	fn append(&mut self, c: Cursor) {
		self.write(SourceCursor::from(c, c.str_slice(self.source_text)))
	}
}

impl<'a, T: SourceCursorSink<'a>> SourceCursorSink<'a> for CursorCompactWriteSink<'a, T> {
	fn append(&mut self, c: SourceCursor<'a>) {
		self.write(c)
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use crate::Arena;
	use crate::{ComponentValues, EmptyAtomSet, Parser, ToCursors};
	use css_lexer::Lexer;

	macro_rules! assert_format {
		($before: literal, $after: literal) => {
			assert_format!(ComponentValues, $before, $after);
		};
		($struct: ident, $before: literal, $after: literal) => {
			let source_text = $before;
			let alloc = Arena::default();
			let mut sink = String::new();
			{
				let mut stream = CursorCompactWriteSink::new(source_text, &mut sink);
				let lexer = Lexer::new(&EmptyAtomSet::ATOMS, source_text);
				let mut parser = Parser::new(&alloc, source_text, lexer);
				parser.parse_entirely::<$struct>().with_trivia().to_cursors(&mut stream);
			}
			assert_eq!(sink, $after.trim());
		};
	}

	#[test]
	fn test_basic() {
		assert_format!("foo{bar: baz();}", r#"foo{bar:baz()}"#);
	}

	#[test]
	fn test_removes_redundant_semis() {
		assert_format!("foo{bar: 1;;;;bing: 2;;;}", r#"foo{bar:1;bing:2}"#);
	}

	#[test]
	fn normalizes_quotes() {
		assert_format!("bar:'baz';bing:'quux';x:url('foo')", r#"bar:"baz";bing:"quux";x:url("foo")"#);
	}

	#[test]
	fn test_does_not_ignore_whitespace_component_values() {
		assert_format!("div dialog:modal > td p a", "div dialog:modal > td p a");
	}

	#[test]
	fn test_compacts_whitespace() {
		assert_format!(
			r#"
		body   >   div {
			bar:  baz
		}
		"#,
			"body > div{bar:baz}"
		);
		assert_format!(".a   .b", ".a .b");
	}

	#[test]
	fn test_does_not_compact_whitespace_resulting_in_new_ident() {
		assert_format!("12px - 1px", "12px - 1px");
	}

	#[test]
	fn test_removes_whitespace_after_comma() {
		assert_format!("foo(a, b, c)", "foo(a,b,c)");
		assert_format!("rgb(255, 128, 0)", "rgb(255,128,0)");
	}

	#[test]
	fn test_keeps_whitespace_after_right_paren_and_square() {
		assert_format!("foo() bar", "foo() bar");
		assert_format!("rgb(0, 0, 0) solid", "rgb(0,0,0) solid");
		assert_format!("[x] bar", "[x] bar");
	}

	#[test]
	fn test_removes_whitespace_after_right_curly() {
		assert_format!("@media screen{} .foo{}", "@media screen{}.foo{}");
	}

	#[test]
	fn test_compacts_numbers_with_leading_zero() {
		assert_format!("opacity: 0.8", "opacity:.8");
		assert_format!("opacity: 0.5", "opacity:.5");
		assert_format!("opacity: 0.123", "opacity:.123");
	}

	#[test]
	fn test_compacts_numbers_with_trailing_zeros() {
		assert_format!("width: 1.0px", "width:1px");
		assert_format!("width: 1.500px", "width:1.5px");
		assert_format!("width: 2.000px", "width:2px");
	}

	#[test]
	fn test_compacts_numbers_with_sign() {
		assert_format!("margin: -0.5px", "margin:-.5px");
		assert_format!("margin: +1.5px", "margin:1.5px");
		assert_format!("margin: +0.8px", "margin:.8px");
	}

	#[test]
	fn test_compacts_edge_case_numbers() {
		assert_format!("opacity: 0.0", "opacity:0");
		assert_format!("opacity: 0", "opacity:0");
		assert_format!("opacity: 1", "opacity:1");
	}

	#[test]
	fn test_does_not_change_numbers_without_optimization() {
		assert_format!("width: 123px", "width:123px");
		assert_format!("width: .5px", "width:.5px");
	}

	#[test]
	fn test_preserves_trailing_semicolons() {
		assert_format!("foo;", "foo;");
	}

	#[test]
	fn test_removes_trailing_semis_when_after_curly() {
		assert_format!("{foo};", "{foo}");
	}

	#[test]
	fn test_drops_trailing_whitespace() {
		assert_format!("foo  ", "foo");
		assert_format!("foo; ", "foo;");
	}

	#[test]
	fn test_preserves_comment_absence_in_custom_properties() {
		assert_format!("div{--bar:a/**/b}", "div{--bar:a/**/b}");
		assert_format!("div { --bar: a/**/b }", "div{--bar:a/**/b}");
		assert_format!("div{--bar:a /* comment */ b}", "div{--bar:a b}");
		assert_format!("div{--bar:a/**//**/b}", "div{--bar:a/**/b}");
		assert_format!("div{--bar:a /* x */ /* y */ b}", "div{--bar:a b}");
		assert_format!("div{/*comment*/--bar:a}", "div{--bar:a}");
		assert_format!("div{--bar:a/*comment*/}", "div{--bar:a}");
		assert_format!("div{--bar:a  /**/  b}", "div{--bar:a b}");
		assert_format!("@container style(--bar:a/**/b){}", "@container style(--bar:a/**/b){}");
		assert_format!("@container style(--bar:a/**/b){}", "@container style(--bar:a/**/b){}");
		assert_format!("foo /**/bar", "foo bar");
		assert_format!("foo{/**/bar}", "foo{bar}");
		assert_format!("foo:/**/bar", "foo:bar");
		assert_format!("foo(/**/bar)", "foo(bar)");
		assert_format!("foo/**/,bar", "foo,bar");
		assert_format!("/**/foo", "foo");
		assert_format!("div{--bar:a/*some really long comment text*/b}", "div{--bar:a/**/b}");
	}

	#[test]
	fn test_at_rule_no_space_before_paren() {
		assert_format!(
			"@media(prefers-reduced-motion:no-preference){:root{}}",
			"@media(prefers-reduced-motion:no-preference){:root{}}"
		);
		assert_format!(
			"@media (prefers-reduced-motion:no-preference){:root{}}",
			"@media (prefers-reduced-motion:no-preference){:root{}}"
		);
		assert_format!("@media(min-width:576px){}", "@media(min-width:576px){}");
	}
}
